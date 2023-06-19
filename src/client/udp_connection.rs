use crate::client::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};

use std::net::{UdpSocket,SocketAddr};
use std::time::Duration;
use std::collections::HashMap;


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
        
        socket_udp.set_read_timeout(Some(timeout)).unwrap(); //FIXME: pensar si timeout sea tipo Opcton<Duration>

        socket_udp
            .send_to(&dns_query_bytes ,server_addr)
            .unwrap_or_else(|e| panic!("Error send {}",e));
        
        println!("[SEND UDP] query sent");

        //TODO: caso en que se reciven trunncados
        let mut msg: [u8;512] = [0;512];
        socket_udp
            .recv_from(&mut msg)
            .unwrap_or_else(|e| panic!("Error recv {}",e));

        println!("{:?}",msg);
        let response_dns: DnsMessage = match DnsMessage::from_bytes(&msg) {
            Ok(response) => response,
            Err(_) => panic!("Error parsing DNS query"),
        };
        
        return  response_dns;
    }
}

//Getters
impl UDPConnection {

    fn get_bind_addr(&self)-> SocketAddr {
        return self.bind_addr.clone();
    }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl UDPConnection {

    fn set_bind_addr(&mut self,addr :SocketAddr) {
        self.bind_addr = addr;
    }

    fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}



#[cfg(test)]
mod udp_connection_test{
    
    use super::*;
    use std::net::{SocketAddr,IpAddr,Ipv4Addr};
    #[test]
    fn create_tcp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port:u16 = 8088;
        let bind_addr:SocketAddr  = SocketAddr::new(ip_addr, port);
        let timeout = Duration::from_secs(2);
        let bind_port = 8088;

        let _conn_new = UDPConnection::new(bind_addr,timeout);

    }
}