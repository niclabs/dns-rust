use crate::client::ClientConnection;

use std::net::{IpAddr,Ipv4Addr};
use std::time::Duration;


pub struct TCPConnection {
    bind_addr: IpAddr,
    timeout: Duration,
}

impl ClientConnection for TCPConnection {

    ///Creates UDPConnection
    fn new(bind_addr:IpAddr, timeout:Duration) -> TCPConnection {
        TCPConnection {
            // domain_name: domain_name,
            bind_addr: bind_addr,
            timeout: timeout,
        }
    }

    //TODO: funcion enviar
    fn send(){
        println!("impl send() for TCPConnection");
    }
}

//Getters
impl TCPConnection {

    fn get_bind_addr(&self)-> IpAddr {
        return self.bind_addr.clone();
    }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl TCPConnection {

    fn set_bind_addr(&mut self,addr :IpAddr) {
        self.bind_addr = addr;
    }

    fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}



#[cfg(test)]
mod tcp_connection_test{
    
    use super::*;
    #[test]
    fn create_tcp() {

        let domain_name = String::from("uchile.cl");
        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);

        let _conn_new = TCPConnection::new(addr,timeout);

    }
}