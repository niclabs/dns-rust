use crate::client::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};

use std::net::{IpAddr,Ipv4Addr,UdpSocket,SocketAddr};
use std::str::FromStr;
use std::time::Duration;


pub struct UDPConnection {
    //addr client
    bind_addr: SocketAddr,
    //timeout read time
    timeout: Duration,
}

impl ClientConnection for UDPConnection {

    ///Creates UDPConnection
    fn new( bind_addr:SocketAddr,timeout:Duration) -> UDPConnection {
        UDPConnection {
            bind_addr: bind_addr,
            timeout: timeout,


        }
    }

    //TODO: funcion enviar
    fn send(&self, server_addr: SocketAddr, dns_query:DnsMessage) -> DnsMessage{
        println!("[SEND UDP]");
        let bind_addr:SocketAddr = self.get_bind_addr();
        let timeout:Duration = self.get_timeout();


        let dns_query_bytes = dns_query.to_bytes(); 

        let socket_udp:UdpSocket = UdpSocket::bind(bind_addr)
                                    .unwrap_or_else(|error| {
                                        panic!("Problem Socket UDP {:?}", error);
                                    });

        socket_udp
            .send_to(&dns_query_bytes ,server_addr)
            .unwrap_or_else(|e| panic!("Error durent send {}",e));
        println!("[SEND UDP] mensaje sent");


        // socket_udp.set_read_timeout(Some(timeout)).unwrap(); //FIXME: pensar mejor si timeout debe guardarse como Option<Duration>
        
        //addr where the query is sent
        // let server_addr:SocketAddr =         



        //FIXME: dummt for no warning
        let dns_query_dummy:DnsMessage = DnsMessage::new();
        return  dns_query_dummy;
    }
}

//Getters
impl UDPConnection {

    fn get_bind_addr(&self)-> SocketAddr {
        return self.bind_addr.clone();
    }

    // fn get_bind_port(&self)-> u16 {
    //     return  self.bind_port;
    // }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl UDPConnection {

    fn set_bind_addr(&mut self,addr :SocketAddr) {
        self.bind_addr = addr;
    }

    // fn set_bind_port(&mut self,bind_port: u16){
    //     self.bind_port = bind_port;
    // }

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
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port:u16 = 8088;
        let bind_addr:SocketAddr  = SocketAddr::new(ip_addr, port);
        let timeout = Duration::from_secs(100);
        let bind_port = 8088;

        let _conn_new = UDPConnection::new(bind_addr,timeout);

    }
}