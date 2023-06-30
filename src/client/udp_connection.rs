use crate::client::ClientConnection;
use crate::message::DnsMessage;

use std::net::{UdpSocket,SocketAddr, IpAddr};
use std::time::Duration;


pub struct  ClientUDPConnection {
    /// addr to connect
    server_addr: IpAddr,
    /// read timeout
    timeout: Duration,
}

impl ClientConnection for ClientUDPConnection {

    /// Creates ClientUDPConnection
    fn new(server_addr:IpAddr, timeout:Duration) -> ClientUDPConnection {
        
        ClientUDPConnection {
            server_addr: server_addr,
            timeout: timeout,
        }
    }

    /// TODO: funcion enviar
    fn send(&self, dns_query:DnsMessage) -> DnsMessage { 
        // TODO: Agregar resultado error 
        println!("[SEND UDP]");

        // let bind_addr = bind_addr.unwrap_or_else();
        let timeout:Duration = self.timeout;
        let server_addr = SocketAddr::new(self.get_server_addr(), 53);

        let dns_query_bytes = dns_query.to_bytes(); 

        let socket_udp:UdpSocket = UdpSocket::bind("0.0.0.0:0") // FIXME:
                                    .unwrap_or_else(|error| {
                                        panic!("Problem Socket UDP {:?}", error);
                                    });
        
        // FIXME: pensar si timeout sea tipo Option<Duration>
        match socket_udp.set_read_timeout(Some(timeout)) {
            Err(_) => panic!("Error setting read timeout for socket"),
            Ok(_) => (),
        }

        socket_udp
            .send_to(&dns_query_bytes, server_addr)
            .unwrap_or_else(|e| panic!("Error send {}",e));
        
        println!("[SEND UDP] query sent");

        // TODO: caso en que se reciven truncados
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
        
        return response_dns;
    }

}

// Getters
impl ClientUDPConnection {

    pub fn get_server_addr(&self)-> IpAddr {
        return self.server_addr.clone();
    }

    pub fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

// Setters
impl ClientUDPConnection {

    fn set_server_addr(&mut self, addr :IpAddr) {
        self.server_addr = addr;
    }

    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}

#[cfg(test)]
mod udp_connection_test{
    
    use super::*;
    use std::net::{IpAddr,Ipv4Addr};
    #[test]
    fn create_udp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(2);
        let _conn_new = ClientUDPConnection::new(ip_addr, timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }
}