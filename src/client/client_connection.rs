/// Trait for client connections
// use crate::message::rdata::Rdata;
// use crate::client::tcp_connection::TCPConecction;
// pub mod tcp_connection::TCPConecction
// use crate::client::udp_connection::UDPConnection;
// use crate::client::tcp_connection::TCPConnection;

use crate::message::DnsMessage;
use std::net::IpAddr;
use std::time::Duration;

use super::client_error::ClientError;

pub trait ClientConnection: Sized {//: 'static + Sized + Send + Sync + Unpin {

    //creates a ClientConecction TCP or UDP 
    fn new(server_addr:IpAddr,
        timeout:Duration) -> Self;
    
    /// function sends query to resolver
    fn send(&self, dns_query:DnsMessage) -> Result<DnsMessage, ClientError> ;

    //FIXME: iran aqui los geters y seters? la firma
}
