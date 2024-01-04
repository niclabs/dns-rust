use crate::client::ClientConnection;
use crate::message::DnsMessage;

use std::net::{UdpSocket,SocketAddr, IpAddr};
use std::time::Duration;
use std::io::Error as IoError;
use std::io::ErrorKind;

use super::client_error::ClientError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// implement get_ip
    /// returns IpAddr
    fn get_ip(&self) -> IpAddr {
        return self.server_addr.clone();
    }

    fn send(self, dns_query:DnsMessage) -> Result<Vec<u8>, ClientError> { 

        let timeout:Duration = self.timeout;
        let server_addr = SocketAddr::new(self.get_server_addr(), 53);

        let dns_query_bytes = dns_query.to_bytes(); 

        let socket_udp:UdpSocket = match UdpSocket::bind("0.0.0.0:0"){ //FIXME:
            Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not bind socket {}", e))).map_err(Into::into),
            Ok(socket_udp) => socket_udp , 
        };                          
        
        // Set read timeout
        match socket_udp.set_read_timeout(Some(timeout)) {
            Err(e) =>  return Err(IoError::new(ErrorKind::Other, format!("Error setting read timeout for socket {}", e))).map_err(Into::into),
            Ok(_) => (),
        }

        match socket_udp.send_to(&dns_query_bytes, server_addr){
            Err(e) =>return Err(IoError::new(ErrorKind::Other, format!("Error: could not send {}", e))).map_err(Into::into),
            Ok(_) => (),
        };

        let mut msg: [u8;512] = [0;512];
        match socket_udp.recv_from(&mut msg){
            Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not read {}", e))).map_err(Into::into),
            Ok(_) => (),
        };

        drop(socket_udp);
        return Ok(msg.to_vec());
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
    
    use crate::domain_name::DomainName;
    use crate::message::type_qtype::Qtype;
    use crate::message::class_qclass::Qclass;
    use super::*;
    use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
    #[test]
    fn create_udp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let _conn_new = ClientUDPConnection::new(ip_addr, timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    // Setters and Getters test
    #[test]
    fn get_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_ip_v4(){
        let ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let connection = ClientUDPConnection::new(ip_address, timeout);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_ip_v6(){
        // ip in V6 version is the equivalent to (192, 168, 0, 1) in V4
        let ip_address = IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0));
        let timeout = Duration::from_secs(100);
        let connection = ClientUDPConnection::new(ip_address, timeout);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0)));
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

    #[test]
    fn send_timeout(){

        let server_addr_non_existent = IpAddr::V4(Ipv4Addr::new(234,1 ,4, 44));
        let timeout = Duration::from_secs(2);

        let conn = ClientUDPConnection::new(server_addr_non_existent, timeout);

        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_query =
        DnsMessage::new_query_message(
            domain_name,
            Qtype::A,
            Qclass::IN,
            0,
            false,
            1);
        
        let result = conn.send(dns_query);

        assert!(result.is_err());
    }

    #[test]
    fn send_query_udp(){

        let server_addr_non_existent = IpAddr::V4(Ipv4Addr::new(8,8 ,8, 8));
        let timeout = Duration::from_secs(2);

        let conn = ClientUDPConnection::new(server_addr_non_existent, timeout);

        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_query =
        DnsMessage::new_query_message(
            domain_name,
            Qtype::A,
            Qclass::IN,
            0,
            false,
            1);
        
        let response = conn.send(dns_query).unwrap();

        // assert!(result.is_ok());

        assert!(DnsMessage::from_bytes(&response).unwrap().get_answer().len() > 0); 

        // assert!(result.unwrap().get_answer().len() > 0); FIXME:
    }

}