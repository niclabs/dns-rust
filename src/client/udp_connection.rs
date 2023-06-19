use crate::client::ClientConnection;

use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use std::time::Duration;


pub struct UDPConnection {
    name_server: String,
    bind_addr: Option<SocketAddr>,
    timeout: Duration,
}

impl ClientConnection for UDPConnection {

    ///Creates UDPConnection
    fn new() -> UDPConnection {
        UDPConnection {
            name_server: String::from(""),
            bind_addr: None,
            timeout: Duration::from_secs(0),
        }
    }

    //TODO: funcion enviar
    fn send(){
        println!("impl send() for TCPConnection");
    }
}

//Getters
impl UDPConnection {

    fn get_name_server(&self)->String{
        return self.name_server.clone();    
    }

    fn get_bind_addr(&self)-> Option<SocketAddr> {
        return self.bind_addr.clone();
    }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }
}

//Setters
impl UDPConnection {
    
    fn set_name_server(&mut self, name_server: String){        
        self.name_server = name_server;
    }
    fn set_bind_addr(&mut self,addr :SocketAddr) {
        self.bind_addr = Some(addr) ;
    }
    fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;

    }


}



#[cfg(test)]
mod udp_connection_test{
    
    use super::*;


    #[test]
    fn create_tcp() {
        let _conn_new = UDPConnection::new();

    }
}