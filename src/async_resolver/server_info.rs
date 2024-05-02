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
}


