/// Trait for client connections
// use crate::message::rdata::Rdata;
// use crate::client::tcp_connection::TCPConecction;
// pub mod tcp_connection::TCPConecction
// use crate::client::udp_connection::UDPConnection;
// use crate::client::tcp_connection::TCPConnection;


pub trait ClientConnection {//: 'static + Sized + Send + Sync + Unpin {

    //creates a ClientConecction TCP or
    fn new() -> Self;
    
    /// function sends query to resolver
    fn send();
}