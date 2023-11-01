use crate::client::tcp_connection::ClientTCPConnection;


use crate::client::udp_connection::ClientUDPConnection;

use crate::message::DnsMessage;
use std::{net::IpAddr,time::Duration};

use super::client_error::ClientError;

pub trait ClientConnection: Copy {//: 'static + Sized + Send + Sync + Unpin 

    //Creates a ClientConecction 
    fn new(server_addr:IpAddr,
        timeout:Duration) -> Self;

    //Sends query 
    fn send(self,dns_query:DnsMessage) -> Result<DnsMessage, ClientError> ;
}

#[derive(Clone)]
pub enum ConnectionProtocol {
    UDP,
    TCP,
    UNKNOWN,
}

impl ConnectionProtocol {
    /// Function to connection type base on a str
    pub fn from_str_to_connection_type(conn: &str) -> ConnectionProtocol{
        match conn {
            "UDP" => ConnectionProtocol::UDP,
            "TCP" => ConnectionProtocol::TCP,
            _ => ConnectionProtocol::UNKNOWN,
        }
    } 
}