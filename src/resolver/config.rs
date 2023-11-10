use crate::client::{udp_connection::ClientUDPConnection, tcp_connection::ClientTCPConnection,client_connection::ClientConnection };
use crate::client::client_connection::ConnectionProtocol;
use std::{net::{IpAddr,SocketAddr,Ipv4Addr}, time::Duration, vec};

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
    name_servers: Vec<(ClientUDPConnection, ClientTCPConnection)>,
    /// Socket address of the resolver.
    bind_addr: SocketAddr,
    /// Maximum quantity of queries for each sent query. 
    /// 
    /// If this number is surpassed, the resolver is expected to panic in 
    /// a Temporary Error.
    retry: u16,
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
    pub fn new(resolver_addr: IpAddr, protocol: ConnectionProtocol, timeout: Duration) -> Self {
        let resolver_config: ResolverConfig = ResolverConfig {
            name_servers: Vec::new(),
            bind_addr: SocketAddr::new(resolver_addr, 53),
            retry: 30,
            cache_enabled: true,
            recursive_available: false,
            protocol: protocol,
            timeout: timeout,
        };
        resolver_config
    }
    
    pub fn default()-> Self {
        // FIXME: these are examples values
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)); 
        let timeout = Duration::from_secs(10);
    
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let resolver_config: ResolverConfig = ResolverConfig {
            name_servers: vec![(conn_udp,conn_tcp)],
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333),
            retry: 30,
            cache_enabled: true,
            recursive_available: false,
            protocol: ConnectionProtocol::UDP,
            timeout: timeout,
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
        
        self.name_servers.push((conn_udp,conn_tcp));
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
    pub fn get_name_servers(&self) -> Vec<(ClientUDPConnection,ClientTCPConnection)> {
        self.name_servers.clone()
    }

    /// Returns the socket address of the resolver.
    pub fn get_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    /// Returns the quantity of retries before the resolver panic in a
    /// Temporary Error.
    pub fn get_retry(&self) -> u16 {
        self.retry
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
}

///Setters
impl ResolverConfig{

    /// Sets the list of Name Servers.
    pub fn set_name_servers(&mut self, list_name_servers: Vec<(ClientUDPConnection,ClientTCPConnection)>) {
        self.name_servers = list_name_servers;
    }

    /// Sets the socket address of the resolver.
    pub fn set_ddr(&mut self,addr:SocketAddr) {
        self.bind_addr = addr;
    }

    /// Sets the quantity of retries before the resolver panic in a
    /// Temporary Error.
    pub fn set_retry(&mut self, retry:u16) {
        self.retry = retry;
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
}


#[cfg(test)]
mod tests_resolver_config {
    //TODO: FK test config and documentation

    use crate::client::client_connection::ClientConnection;
    use crate::client::tcp_connection::ClientTCPConnection;
    use crate::client::udp_connection::ClientUDPConnection;
    use crate::client::{config::TIMEOUT, client_connection::ConnectionProtocol};
    use crate::resolver::config::ResolverConfig;
    use std::net::{IpAddr,Ipv4Addr, SocketAddr};
    use std::time::Duration;

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
        assert_eq!(resolver_config.get_name_servers().len(), 2);
    }

    #[test]
    fn get_and_set_name_servers() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_name_servers().len(), 1);

        let addr_1 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let tcp_conn_1 = ClientTCPConnection::new(addr_1, Duration::from_secs(TIMEOUT));
        let udp_conn_1 = ClientUDPConnection::new(addr_1, Duration::from_secs(TIMEOUT));

        let addr_2 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2));
        let tcp_conn_2 = ClientTCPConnection::new(addr_2, Duration::from_secs(TIMEOUT));
        let udp_conn_2 = ClientUDPConnection::new(addr_2, Duration::from_secs(TIMEOUT));

        let name_servers = vec![(udp_conn_1, tcp_conn_1), (udp_conn_2, tcp_conn_2)];
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
    fn get_and_set_retry() {
        let mut resolver_config = ResolverConfig::default();

        assert_eq!(resolver_config.get_retry(), 30);

        resolver_config.set_retry(10);

        assert_eq!(resolver_config.get_retry(), 10);
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

        assert_eq!(resolver_config.get_timeout(), Duration::from_secs(TIMEOUT));

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
}
