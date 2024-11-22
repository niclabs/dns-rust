use crate::message::DnsMessage;
use std::net::IpAddr;
use tokio::time::Duration;
use super::client_error::ClientError;

use async_trait::async_trait;


#[async_trait]
pub trait ClientSecurity: Copy {//: 'static + Sized + Send + Sync + Unpin 

    //Creates a ClientConecction 
    fn new(server_addr:IpAddr,
        timeout:Duration) -> Self;

    fn new_default(server_addr:IpAddr, timeout:Duration) -> Self;

    //Sends query 
    async fn send(self, dns_query: DnsMessage) -> Result<Vec<u8>, ClientError>;
    // async fn send(self, dns_query: DnsMessage) -> Result<(Vec<u8>, IpAddr), ClientError>;
    fn get_ip(&self) -> IpAddr;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionProtocol {
    DTLS,
    TLS,
    UNKNOWN,
}

impl From<&str> for ConnectionProtocol {
    /// Function to connection type base on a str
    // from_str_to_connection_type
    fn from(conn: &str) -> ConnectionProtocol {
        match conn {
            "DTLS" => ConnectionProtocol::DTLS,
            "TLS" => ConnectionProtocol::TLS,
            _ => ConnectionProtocol::UNKNOWN,
        }
    } 
}