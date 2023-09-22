use crate::client::ClientConnection;
use crate::message::DnsMessage;

use std::net::{UdpSocket,SocketAddr, IpAddr};
use std::time::Duration;
use std::io::Error as IoError;
use std::io::ErrorKind;

use super::client_error::ClientError;

#[derive(Clone,Copy)]
pub struct  ClientUDPConnection {
    /// addr to connect
    server_addr: IpAddr,
    /// read timeout
    timeout: Duration,
}

impl ClientConnection for ClientUDPConnection {

    /// Creates ClientUDPConnection
    fn new(server_addr:IpAddr, timeout:Duration) -> Self {
        
        ClientUDPConnection {
            server_addr: server_addr,
            timeout: timeout,
        }
    }

    /// TODO: funcion enviar
    fn send(self, dns_query:DnsMessage) -> Result<DnsMessage, ClientError> { 
        // TODO: Agregar resultado error 
        println!("[SEND UDP]");

        // let bind_addr = bind_addr.unwrap_or_else();
        let timeout:Duration = self.timeout;
        let server_addr = SocketAddr::new(self.get_server_addr(), 53);

        let dns_query_bytes = dns_query.to_bytes(); 

        let socket_udp:UdpSocket = match UdpSocket::bind("0.0.0.0:0"){ //FIXME:
            Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not bind socket {}", e))).map_err(Into::into),
            Ok(socket_udp) => socket_udp , 
        };                          
        
        //set read timeout
        match socket_udp.set_read_timeout(Some(timeout)) {
            Err(e) =>  return Err(IoError::new(ErrorKind::Other, format!("Error setting read timeout for socket {}", e))).map_err(Into::into),
            Ok(_) => (),
        }

        match socket_udp.send_to(&dns_query_bytes, server_addr){
            Err(e) =>return Err(IoError::new(ErrorKind::Other, format!("Error: could not send {}", e))).map_err(Into::into),
            Ok(_) => (),
        };
        
        println!("[SEND UDP] query sent");

        // TODO: caso en que se reciben truncados
        let mut msg: [u8;512] = [0;512];
        match socket_udp.recv_from(&mut msg){
            Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not read {}", e))).map_err(Into::into),
            Ok(_) => (),
        };
        

        let response_dns: DnsMessage = match DnsMessage::from_bytes(&msg) {
            Ok(response) => response,
            Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not create dns message {}", e))).map_err(Into::into),
        };
        // println!("[SEND UDP] {:?}", msg);
        
        drop(socket_udp);
        
        return Ok(response_dns);
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

    pub fn set_server_addr(&mut self, addr :IpAddr) {
        self.server_addr = addr;
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
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
        let timeout = Duration::from_secs(100);
        let _conn_new = ClientUDPConnection::new(ip_addr, timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    //Setters and Getters test
    #[test]
    fn get_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn set_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));

        _conn_new.set_server_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn get_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    #[test]
    fn set_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));

        _conn_new.set_timeout(Duration::from_secs(200));

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(200));
    }
}