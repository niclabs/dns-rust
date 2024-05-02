use crate::client::tcp_connection::ClientTCPConnection;
use crate::client::udp_connection::ClientUDPConnection;
use crate::message::{resource_record::ResourceRecord, DnsMessage};
use std::net::IpAddr;


///This struscture is used to represent the information of a server.

#[derive(Debug, Clone)]
pub struct ServerInfo {
    //The IP address of the server.
    ip_addr: IpAddr,
    //The port of the server.
    port: u16,
    //The key of the server.
    key: String,
    // The algorithm of the server.
    algorithm: String,
    //UDP connection
    udp_connection: ClientUDPConnection,
    //TCP connection
    tcp_connection: ClientTCPConnection,
}

impl ServerInfo {
    /// Create a new `ServerInfo` instance.
    pub fn new(ip_addr: IpAddr, port: u16, key: String, algorithm: String, 
        udp_connection: ClientUDPConnection, tcp_connection: ClientTCPConnection) -> ServerInfo {
        ServerInfo {
            ip_addr,
            port,
            key,
            algorithm,
            udp_connection,
            tcp_connection,
        }
    }

    /// Implements get_ip_address
    /// Returns IpAddr.
    pub fn get_ip_addr(&self) -> IpAddr {
        self.ip_addr
    }

    /// Implements get the port of the server.
    /// return the port
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Get the key of the server.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Get the algorithm of the server.
    pub fn get_algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Get the UDP connection of the server.
    /// return the UDP connection
    pub fn get_udp_connection(&self) -> &ClientUDPConnection {
        &self.udp_connection
    }

    /// Get the TCP connection of the server.
    /// return the TCP connection
    pub fn get_tcp_connection(&self) -> &ClientTCPConnection {
        &self.tcp_connection
    }
}

#[cfg(test)]
mod server_info_tests {
    use crate::client::client_connection::ClientConnection;

    use super::*;
    use std::{net::{IpAddr, Ipv4Addr}, time::Duration};

    #[test]
    fn create_server_info() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new(ip_addr, Duration::from_secs(100));
        let server_info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        assert_eq!(server_info.get_ip_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(server_info.get_port(), 53);
        assert_eq!(server_info.get_key(), "key");
        assert_eq!(server_info.get_algorithm(), "algorithm");
        assert_eq!(server_info.get_udp_connection().get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(server_info.get_udp_connection().get_timeout(), Duration::from_secs(100));
        assert_eq!(server_info.get_tcp_connection().get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(server_info.get_tcp_connection().get_timeout(), Duration::from_secs(100));
    }
    
    #[test]
    fn get_ip_addr() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new(ip_addr, Duration::from_secs(100));
        let server_info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        assert_eq!(server_info.get_ip_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_port() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new(ip_addr, Duration::from_secs(100));
        let server_info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        assert_eq!(server_info.get_port(), 53);
    }

    #[test]
    fn get_key() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new(ip_addr, Duration::from_secs(100));
        let server_info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        assert_eq!(server_info.get_key(), "key");
    }

}

