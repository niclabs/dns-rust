use crate::client::{udp_connection::ClientUDPConnection, tcp_connection::ClientTCPConnection,client_connection::ClientConnection };
use crate::client::client_connection::ConnectionProtocol;
use std::cmp::max;
use std::{net::{IpAddr,SocketAddr,Ipv4Addr}, time::Duration};

use super::server_info::ServerInfo;

const GOOGLE_PRIMARY_DNS_SERVER: [u8; 4] = [8, 8, 8, 8];
const GOOGLE_SECONDARY_DNS_SERVER: [u8; 4] = [8, 8, 4, 4];
const CLOUDFLARE_PRIMARY_DNS_SERVER: [u8; 4] = [1, 1, 1, 1];
const CLOUDFLARE_SECONDARY_DNS_SERVER: [u8; 4] = [1, 0, 0, 1];
const OPEN_DNS_PRIMARY_DNS_SERVER: [u8; 4] = [208, 67, 222, 222];
const OPEN_DNS_SECONDARY_DNS_SERVER: [u8; 4] = [208, 67, 220, 220];
const QUAD9_PRIMARY_DNS_SERVER: [u8; 4] = [9, 9, 9, 9];
const QUAD9_SECONDARY_DNS_SERVER: [u8; 4] = [149, 112, 112, 112];

#[derive(Clone, Debug, PartialEq, Eq)]

/// Configuration for the resolver.
/// 
/// This struct contains all the necessary configurations to create a new
/// resolver. This includes a list of connections to Name Servers, the socket
/// address of the resolver, the quantity of retries before the resolver
/// panic in a Temporary Error, availability of cache and recursive queries, 
/// the chosen transport protocol and the timeout for the connections.
pub struct ResolverConfig {
    /// Vector of tuples with the UDP and TCP connections to a Name Server.
    name_servers: Vec<ServerInfo>,
    /// Socket address of the resolver.
    bind_addr: SocketAddr,
    /// Maximum quantity of queries for each sent query. 
    /// 
    /// If this number is surpassed, the resolver is expected to panic in 
    /// a Temporary Error.
    retransmission_loop_attempts: u16,
    /// Activation of cache in this resolver.
    /// 
    /// This is whether the resolver uses cache or not.
    cache_enabled: bool,
    /// Availability of recursive queries in this resolver.
    /// 
    /// This is whether the resolver uses recursive queries or not.
    recursive_available: bool,
    /// Transport protocol for queries.
    /// 
    /// This is the transport protocol used by the resolver to send queries
    /// and corresponds to `ConnectionProtocol` enum type.
    protocol: ConnectionProtocol,
    /// Timeout for connections.
    /// 
    /// This corresponds a `Duration` type.
    timeout: Duration,
    max_retry_interval_seconds: u64,
    min_retry_interval_seconds: u64,
    // While local limits on the number of times a resolver will retransmit
    // a particular query to a particular name server address are
    // essential, the resolver should have a global per-request
    // counter to limit work on a single request.  The counter should
    // be set to some initial value and decremented whenever the
    // resolver performs any action (retransmission timeout,
    // retransmission, etc.)  If the counter passes zero, the request
    // is terminated with a temporary error.
    global_retransmission_limit: u16,
}

impl ResolverConfig {
    /// Creates a ResolverConfig with the given address, protocol and timeout.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::net::IpAddr;
    /// use std::time::Duration;
    /// use dns_resolver::client::client_connection::ConnectionProtocol;
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// 
    /// let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    /// let protocol = ConnectionProtocol::UDP;
    /// let timeout = Duration::from_secs(TIMEOUT);
    /// let resolver_config = ResolverConfig::new(addr, protocol, timeout);
    /// assert_eq!(resolver_config.get_addr(), SocketAddr::new(addr, 53));
    /// ```
    pub fn new(
        resolver_addr: IpAddr, 
        protocol: ConnectionProtocol, 
        timeout: Duration,
        ) -> Self {
        let resolver_config: ResolverConfig = ResolverConfig {
            name_servers: Vec::new(),
            bind_addr: SocketAddr::new(resolver_addr, 53),
            retransmission_loop_attempts: 3,
            cache_enabled: true,
            recursive_available: false,
            protocol: protocol,
            timeout: timeout,
            max_retry_interval_seconds: 10,
            min_retry_interval_seconds: 1,
            global_retransmission_limit: 30,
        };
        resolver_config
    }
    
    pub fn default()-> Self {
        // FIXME: these are examples values
        let retransmission_loop_attempts = 3;
        let global_retransmission_limit = 30;
        let timeout = Duration::from_secs(45);
        let max_retry_interval_seconds = 10;

        let mut servers_info = Vec::new();
        servers_info.push(ServerInfo::new_from_addr(GOOGLE_PRIMARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(CLOUDFLARE_PRIMARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(OPEN_DNS_PRIMARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(QUAD9_PRIMARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(GOOGLE_SECONDARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(CLOUDFLARE_SECONDARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(OPEN_DNS_SECONDARY_DNS_SERVER.into(), timeout));
        servers_info.push(ServerInfo::new_from_addr(QUAD9_SECONDARY_DNS_SERVER.into(), timeout));

        // Recommended by RFC 1536: max(4, 5/number_of_server_to_query)
        let number_of_server_to_query = servers_info.len() as u64;
        let min_retry_interval_seconds: u64 = max(1, 5/number_of_server_to_query).into();

        let resolver_config: ResolverConfig = ResolverConfig {
            name_servers: servers_info,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333),
            retransmission_loop_attempts: retransmission_loop_attempts,
            cache_enabled: true,
            recursive_available: false,
            protocol: ConnectionProtocol::UDP,
            timeout: timeout,
            max_retry_interval_seconds: max_retry_interval_seconds,
            min_retry_interval_seconds: min_retry_interval_seconds,
            global_retransmission_limit: global_retransmission_limit,
        };
        resolver_config
    }

    /// Adds a new Name Server to the list of Name Servers.
    /// 
    /// This corresponds to a tuple of UDP and TCP connections to a Name Server
    /// of the type `(ClientUDPConnection, ClientTCPConnection)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::net::IpAddr;
    /// use std::time::Duration;
    /// use dns_resolver::client::client_connection::ConnectionProtocol;
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// 
    /// let mut resolver_config = ResolverConfig::default();
    /// let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    /// resolver_config.add_servers(addr);
    /// assert_eq!(resolver_config.get_name_servers().len(), 2);
    /// ```
    pub fn add_servers(&mut self, addr: IpAddr) {
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(addr, self.timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(addr, self.timeout);

        let server_info = ServerInfo::new_with_ip(addr, conn_udp, conn_tcp);
        self.name_servers.push(server_info);
    }

    /// Remove all servers from the list of Name Servers.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::net::IpAddr;
    /// use std::time::Duration;
    /// use dns_resolver::client::client_connection::ConnectionProtocol;
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// 
    /// let mut resolver_config = ResolverConfig::default();
    /// let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    /// resolver_config.add_servers(addr);
    /// assert_eq!(resolver_config.get_name_servers().len(), 2);
    /// resolver_config.remove_servers();
    /// assert_eq!(resolver_config.get_name_servers().len(), 0);
    /// ```
    pub fn remove_servers(&mut self) {
        self.name_servers = Vec::new();
    }
}

///Getters
impl ResolverConfig {

    /// Returns the list of Name Servers.
    pub fn get_name_servers(&self) -> Vec<ServerInfo> {
        self.name_servers.clone()
    }

    /// Returns the socket address of the resolver.
    pub fn get_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    /// Returns the quantity of retries before the resolver panic in a
    /// Temporary Error.
    pub fn get_retransmission_loop_attempts(&self) -> u16 {
        self.retransmission_loop_attempts
    }

    /// Returns whether the cache is enabled or not.
    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }

    /// Returns whether the cache is enabled or not.
    pub fn get_recursive_available(&self) -> bool {
        self.recursive_available
    }

    /// Returns the transport protocol for queries.
    pub fn get_protocol(&self) -> ConnectionProtocol {
        self.protocol  
    }

    /// Returns the timeout for connections.
    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    pub fn get_max_retry_interval_seconds(&self) -> u64 {
        self.max_retry_interval_seconds
    }

    pub fn get_min_retry_interval_seconds(&self) -> u64 {
        self.min_retry_interval_seconds
    }

    pub fn get_global_retransmission_limit(&self) -> u16 {
        self.global_retransmission_limit
    }
}

///Setters
impl ResolverConfig{

    /// Sets the list of Name Servers.
    pub fn set_name_servers(&mut self, list_name_servers: Vec<ServerInfo>) {
        self.name_servers = list_name_servers;
    }

    /// Sets the socket address of the resolver.
    pub fn set_ddr(&mut self,addr:SocketAddr) {
        self.bind_addr = addr;
    }

    /// Sets the quantity of retries before the resolver panic in a
    /// Temporary Error.
    pub fn set_retransmission_loop_attempts(&mut self, retransmission_loop_attempts:u16) {
        self.retransmission_loop_attempts = retransmission_loop_attempts;
    }

    /// Sets whether the cache is enabled or not.
    pub fn set_cache_enabled(&mut self, cache_enabled:bool) {
        self.cache_enabled = cache_enabled;
    }

    /// Sets whether the cache is enabled or not.
    pub fn set_recursive_available(&mut self, recursive_available:bool) {
        self.recursive_available = recursive_available;
    }

    /// Sets the transport protocol for queries.
    pub fn set_protocol(&mut self, protocol:ConnectionProtocol) {
        self.protocol = protocol;
    }

    /// Sets the timeout for connections.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn set_max_retry_interval_seconds(&mut self, max_retry_interval_seconds: u64) {
        self.max_retry_interval_seconds = max_retry_interval_seconds;
    }

    pub fn set_min_retry_interval_seconds(&mut self, min_retry_interval_seconds: u64) {
        self.min_retry_interval_seconds = min_retry_interval_seconds;
    }

    pub fn set_global_retransmission_limit(&mut self, global_retransmission_limit: u16) {
        self.global_retransmission_limit = global_retransmission_limit;
    }
}


#[cfg(test)]
mod tests_resolver_config {
    use crate::async_resolver::server_info;
    //TODO: FK test config and documentation
    use crate::client::client_connection::ClientConnection;
    use crate::client::tcp_connection::ClientTCPConnection;
    use crate::client::udp_connection::ClientUDPConnection;
    use crate::client::client_connection::ConnectionProtocol;
    use crate::async_resolver::config::ResolverConfig;
    use std::net::{IpAddr,Ipv4Addr, SocketAddr};
    use std::time::Duration;
    static TIMEOUT: u64 = 10;

    #[test]
    fn create_resolver_config() {
        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let protocol = ConnectionProtocol::UDP;
        let timeout = Duration::from_secs(TIMEOUT);
        let resolver_config = ResolverConfig::new(addr, protocol, timeout);

        assert_eq!(resolver_config.get_addr(), SocketAddr::new(addr, 53));
    }

    #[test]
    fn add_servers() {
        let mut resolver_config = ResolverConfig::default();
        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        resolver_config.add_servers(addr);
        assert_eq!(resolver_config.get_name_servers().len(), 9);
    }

    #[test]
    fn get_and_set_name_servers() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_name_servers().len(), 8);

        let addr_1 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let tcp_conn_1 = ClientTCPConnection::new(addr_1, Duration::from_secs(TIMEOUT));
        let udp_conn_1 = ClientUDPConnection::new(addr_1, Duration::from_secs(TIMEOUT));

        let addr_2 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2));
        let tcp_conn_2 = ClientTCPConnection::new(addr_2, Duration::from_secs(TIMEOUT));
        let udp_conn_2 = ClientUDPConnection::new(addr_2, Duration::from_secs(TIMEOUT));
        let server_info_1 = server_info::ServerInfo::new_with_ip(addr_1, udp_conn_1, tcp_conn_1);
        let server_info_2 = server_info::ServerInfo::new_with_ip(addr_2, udp_conn_2, tcp_conn_2);

        let name_servers = vec![server_info_1, server_info_2];
        resolver_config.set_name_servers(name_servers.clone());

        assert_eq!(resolver_config.get_name_servers(), name_servers);
    }

    #[test]
    fn get_and_set_addr() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_addr(), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333));

        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        resolver_config.set_ddr(SocketAddr::new(addr, 10));

        assert_eq!(resolver_config.get_addr(), SocketAddr::new(addr, 10));
    }

    #[test]
    fn get_and_set_retransmission_loop_attempts() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_retransmission_loop_attempts(), 3);

        resolver_config.set_retransmission_loop_attempts(10);

        assert_eq!(resolver_config.get_retransmission_loop_attempts(), 10);
    }

    #[test]
    fn get_and_set_recursive_available() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_recursive_available(), false);

        resolver_config.set_recursive_available(true);

        assert_eq!(resolver_config.get_recursive_available(), true);
    }

    #[test]
    fn get_and_set_protocol() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_protocol(), ConnectionProtocol::UDP);

        resolver_config.set_protocol(ConnectionProtocol::TCP);

        assert_eq!(resolver_config.get_protocol(), ConnectionProtocol::TCP);
    }

    #[test]
    fn get_and_set_timeout() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_timeout(), Duration::from_secs(45));

        resolver_config.set_timeout(Duration::from_secs(10));

        assert_eq!(resolver_config.get_timeout(), Duration::from_secs(10));
    }

    #[test]
    fn get_and_set_cache_enabled() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.is_cache_enabled(), true);

        resolver_config.set_cache_enabled(false);

        assert_eq!(resolver_config.is_cache_enabled(), false);
    }

    #[test]
    fn remove_servers() {
        let mut resolver_config = ResolverConfig::default();
        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        resolver_config.add_servers(addr);
        assert_eq!(resolver_config.get_name_servers().len(), 2);
        resolver_config.remove_servers();
        assert_eq!(resolver_config.get_name_servers().len(), 0);
    }

    #[test]
    fn get_and_set_max_retry_interval_seconds() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_max_retry_interval_seconds(), 10);

        resolver_config.set_max_retry_interval_seconds(20);

        assert_eq!(resolver_config.get_max_retry_interval_seconds(), 20);
    }

    #[test]
    fn get_and_set_min_retry_interval_seconds() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_min_retry_interval_seconds(), 1);

        resolver_config.set_min_retry_interval_seconds(2);

        assert_eq!(resolver_config.get_min_retry_interval_seconds(), 2);
    }

    #[test]
    fn get_and_set_global_retransmission_limit() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_global_retransmission_limit(), 30);

        resolver_config.set_global_retransmission_limit(40);

        assert_eq!(resolver_config.get_global_retransmission_limit(), 40);
    }
}