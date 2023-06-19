use crate::client::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};

use std::net::{IpAddr,Ipv4Addr};
use std::time::Duration;


pub struct UDPConnection {
    bind_addr: IpAddr,
    timeout: Duration,
}

impl ClientConnection for UDPConnection {

    ///Creates UDPConnection
    fn new( bind_addr:IpAddr, timeout:Duration) -> UDPConnection {
        UDPConnection {
            bind_addr: bind_addr,
            timeout: timeout,
        }
    }

    //TODO: funcion enviar
    fn send(&self,dns_query:DnsMessage) -> DnsMessage{

        println!("impl send() for TCPConnection");

        //FIXME: dummt for no warning
        let dns_query_dummy:DnsMessage = DnsMessage::new();
        return  dns_query_dummy;
    }
}

//Getters
impl UDPConnection {

    fn get_bind_addr(&self)-> IpAddr {
        return self.bind_addr.clone();
    }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl UDPConnection {

    fn set_bind_addr(&mut self,addr :IpAddr) {
        self.bind_addr = addr;
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

        // let domain_name = String::from("uchile.cl");
        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);

        let _conn_new = UDPConnection::new(addr,timeout);

    }
}