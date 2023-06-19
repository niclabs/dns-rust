use crate::client::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};

use std::net::{IpAddr,Ipv4Addr,TcpStream,SocketAddr};
use std::time::Duration;


pub struct TCPConnection {
    //addr client
    bind_addr: SocketAddr,
    //timeout read time
    timeout: Duration,
}

impl ClientConnection for TCPConnection {

    ///Creates UDPConnection
    fn new(bind_addr:SocketAddr, timeout:Duration) -> TCPConnection {
        TCPConnection {
            bind_addr: bind_addr,
            timeout: timeout,
        }
    }

    //TODO: funcion enviar
    fn send(&self,server_addr: SocketAddr, dns_query:DnsMessage)-> DnsMessage{

        println!("[SEND TCP]");
        //FIXME: dummt for no warning
        let dns_query_dummy:DnsMessage = DnsMessage::new();
        return  dns_query_dummy;
    }
}

//Getters
impl TCPConnection {

    fn get_bind_addr(&self)-> SocketAddr {
        return self.bind_addr.clone();
    }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl TCPConnection {

    fn set_bind_addr(&mut self,addr :SocketAddr) {
        self.bind_addr = addr;
    }

    // fn set_bind_port(&mut self,port :u16) {
    //     self.bind_port = port;
    // }

    fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}



#[cfg(test)]
mod tcp_connection_test{
    
    use super::*;
    #[test]
    fn create_tcp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port:u16 = 8088;
        let bind_addr:SocketAddr  = SocketAddr::new(ip_addr, port);
        let timeout = Duration::from_secs(100);
        let bind_port = 8088;

        let _conn_new = TCPConnection::new(bind_addr,timeout);

    }
}