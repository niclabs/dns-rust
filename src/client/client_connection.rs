use crate::message::DnsMessage;
use std::{net::IpAddr,time::Duration};

use super::client_error::ClientError;

pub trait ClientConnection: Copy {//: 'static + Sized + Send + Sync + Unpin 

    //Creates a ClientConecction 
    fn new(server_addr:IpAddr,
        timeout:Duration) -> Self;

    //Sends query 
    fn send(self, dns_query: DnsMessage) -> Result<Vec<u8>, ClientError>;
    fn get_ip(&self) -> IpAddr;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionProtocol {
    UDP,
    TCP,
    UNKNOWN,
}

impl From<&str> for ConnectionProtocol {
    /// Function to connection type base on a str
    // from_str_to_connection_type
    fn from(conn: &str) -> ConnectionProtocol {
        match conn {
            "UDP" => ConnectionProtocol::UDP,
            "TCP" => ConnectionProtocol::TCP,
            _ => ConnectionProtocol::UNKNOWN,
        }
    } 
}