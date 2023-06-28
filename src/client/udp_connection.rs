use crate::client::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};

use std::net::{UdpSocket,SocketAddr,IpAddr};
use std::time::Duration;
use std::collections::HashMap;


pub struct UDPConnection {
    //addr that client will connect
    bind_addr: IpAddr,
    //timeout read time
    timeout: Duration,
}

impl ClientConnection for UDPConnection {

    ///Creates UDPConnection
    fn new( bind_addr:IpAddr,timeout:Duration) -> UDPConnection {
        UDPConnection {
            bind_addr: bind_addr,
            timeout: timeout,


        }
    }

    //TODO: funcion enviar
    fn send(&self,dns_query:DnsMessage) -> DnsMessage{
        println!("[SEND UDP]");
        let bind_addr:SocketAddr = SocketAddr::new(self.get_bind_addr(), 53);
        let timeout:Duration = self.get_timeout();
        let my_add = "127.0.0.1:3400";


        let dns_query_bytes = dns_query.to_bytes(); 

        let socket_udp:UdpSocket = UdpSocket::bind("127.0.0.1:3400")
                                    .unwrap_or_else(|error| {
                                        panic!("Problem Socket UDP {:?}", error);
                                    });
        
        //FIXME: pensar si timeout sea tipo Opcton<Duration>
        match socket_udp.set_read_timeout(Some(timeout)) {
            Err(_) => panic!("Error setting read timeout for socket"),
            Ok(_) => (),
        }

        socket_udp
            .send_to(&dns_query_bytes ,bind_addr)
            .unwrap_or_else(|e| panic!("Error send {}",e));
        
        println!("[SEND UDP] query sent");

        //TODO: caso en que se reciven trunncados
        let mut msg: [u8;512] = [0;512];
        socket_udp
            .recv_from(&mut msg)
            .unwrap_or_else(|e| panic!("Error recv {}",e));

        let response_dns: DnsMessage = match DnsMessage::from_bytes(&msg) {
            Ok(response) => response,
            Err(_) => panic!("Error parsing DNS query"),
        };
        // println!("[SEND UDP] {:?}", msg);
        
        drop(socket_udp);
        
        return  response_dns;
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
    use std::net::{SocketAddr,IpAddr,Ipv4Addr};
    #[test]
    fn create_tcp() {        
        //create connection
        let port: u16 = 8089;
        let ip_addr_to_connect:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));

        // let addr: SocketAddr = SocketAddr::new(ip_addr, port);
        let timeout: Duration = Duration::from_secs(2);
        let addr_cloudfare = SocketAddr::new(ip_addr_to_connect, port)
;       let conn_udp:UDPConnection = ClientConnection::new(ip_addr_to_connect,timeout);

        //Query
        let dns_query = DnsMessage::new_query_message("uchile.cl".to_string(),
                                                "A".to_string(),
                                                "IN".to_string(),
                                                0, false, 111);
        
        let mut response = conn_udp.send(dns_query);
        response.print_dns_message();




    }
}