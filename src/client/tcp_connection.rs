use crate::client::ClientConnection;

use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use std::time::Duration;


pub struct TCPConnection {
    name_server: SocketAddr,
    bind_addr: Option<SocketAddr>,
    timeout: Duration,
}

impl TCPConnection {
    


}

impl ClientConnection for TCPConnection {

    ///Creates UDPConnection
    fn new() -> TCPConnection {
        TCPConnection {
            name_server: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
            bind_addr:None, 
            timeout: Duration::from_secs(0),
        }
    }

    //TODO: funcion enviar
    fn send(){
        println!("impl send() for TCPConnection");
    }
}


#[cfg(test)]
mod udp_connection_test{
    
    use super::*;
    #[test]
    fn create_tcp() {
        let _conn_new = TCPConnection::new();

    }
}