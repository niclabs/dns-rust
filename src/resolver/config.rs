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
    /// Availability of cache in this resolver.
    /// 
    /// This is whether the resolver uses cache or not.
    cache_available: bool,
    /// Activation of cache in this resolver.
    /// 
    /// This is whether the resolver uses cache or not when it is available.
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
            cache_available: true,
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
            cache_available: true,
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

    pub fn remove_server(&mut self) {
        self.name_servers = Vec::new();
    }
}

///Getters
impl ResolverConfig {

    pub fn get_name_servers(&self) -> Vec<(ClientUDPConnection,ClientTCPConnection)>{
        self.name_servers.clone()
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    pub fn get_retry(&self) -> u16{
        self.retry
    }

    pub fn get_cache_available(&self) -> bool{
        self.cache_available 
    }

    pub fn get_recursive_available(&self) -> bool{
        self.recursive_available
    }

    pub fn get_protocol(&self) -> ConnectionProtocol {
        self.protocol  
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }
}

///Setters
impl ResolverConfig{

    pub fn set_name_servers(&mut self,list_name_servers: Vec<(ClientUDPConnection,ClientTCPConnection)>) {
        self.name_servers = list_name_servers;
    }

    pub fn set_ddr(&mut self,addr:SocketAddr){
        self.bind_addr = addr;
    }

    pub fn set_retry(&mut self, retry:u16){
        self.retry = retry;
    }

    pub fn set_cache_available(&mut self, cache_available:bool){
        self.cache_available = cache_available;
    }

    pub fn set_recursive_available(&mut self,recursive_available:bool){
        self.recursive_available = recursive_available;
    }

    pub fn set_protocol(&mut self,protocol:ConnectionProtocol){
        self.protocol = protocol;
    }

    pub fn set_timeout(&mut self,timeout: Duration){
        self.timeout = timeout;
    }
}


#[cfg(test)]
mod tests_resolver_config {
    //TODO: FK test config and documentation

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
}
