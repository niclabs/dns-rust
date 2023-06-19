/// Trait for client connections
// use crate::message::rdata::Rdata;
// use crate::client::tcp_connection::TCPConecction;
// pub mod tcp_connection::TCPConecction
// use crate::client::udp_connection::UDPConnection;
// use crate::client::tcp_connection::TCPConnection;

use crate::message::{DnsMessage};
use std::net::{IpAddr,Ipv4Addr};
use std::time::Duration;
pub trait ClientConnection: Sized{//: 'static + Sized + Send + Sync + Unpin {

    //creates a ClientConecction TCP or
    fn new( bind_addr:IpAddr,
            timeout:Duration) -> Self;
    
    /// function sends query to resolver
    fn send() {
        println!("Default implementation of `send` in ClientConnection");
    }
}
