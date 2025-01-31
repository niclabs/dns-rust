use crate::client::ClientConnection;
use crate::message::DnsMessage;
use crate::message::rdata::Rdata;
use crate::message::rdata::a_rdata::ARdata;
use crate::message::resource_record::ResourceRecord;

use async_trait::async_trait;
use std::net::{SocketAddr, IpAddr};

use tokio::time::{Duration, timeout};
use std::io::Error as IoError;
use std::io::ErrorKind;
use tokio::net::UdpSocket;
use super::client_error::ClientError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct  ClientUDPConnection {
    /// addr to connect
    server_addr: IpAddr,
    /// read timeout
    timeout: tokio::time::Duration,
    /// payload size
    payload_size: usize,
}

#[async_trait]
impl ClientConnection for ClientUDPConnection {

    /// Creates ClientUDPConnection
    fn new(server_addr:IpAddr, timeout:Duration, payload_size: usize) -> Self {
        ClientUDPConnection {
            server_addr: server_addr,
            timeout: timeout,
            payload_size: payload_size,
        }
    }

    /// implement get_ip
    /// returns IpAddr
    fn get_ip(&self) -> IpAddr {
        self.server_addr.clone()
    }

    async fn send(self, dns_query:DnsMessage) -> Result<Vec<u8>, ClientError> { 
    // async fn send(self, dns_query:DnsMessage) -> Result<(Vec<u8>, IpAddr), ClientError> { 

        let conn_timeout:Duration = self.timeout;
        let server_addr = SocketAddr::new(self.get_server_addr(), 53);

        let dns_query_bytes = dns_query.to_bytes(); 

        //FIXME: chage port 
        let socket_udp = UdpSocket::bind("0.0.0.0:0").await?; //FIXME: type error

        // let socket_udp:UdpSocket = match UdpSocket::bind("0.0.0.0:0"){ //FIXME:
        //     Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not bind socket {}", e))).map_err(Into::into),
        //     Ok(socket_udp) => socket_udp , 
        // };                          
        
        // TODO: Set read timeout 
        // match socket_udp.set_read_timeout(Some(timeout)) {
        //     Err(e) =>  return Err(IoError::new(ErrorKind::Other, format!("Error setting read timeout for socket {}", e))).map_err(Into::into),
        //     Ok(_) => (),
        // }

        match socket_udp.send_to(&dns_query_bytes, server_addr).await {
            Err(e) =>return Err(IoError::new(ErrorKind::Other, format!("Error: could not send {}", e))).map_err(Into::into),
            Ok(_) => (),
        };
        
        let mut msg = vec![0;self.payload_size];
        //FIXME: not always is timeout error, since it doesn't have to be wait for the timeout return
        // and error, is just an IO error, the timeout error should come by itself from the timeout function
        let result = match timeout(conn_timeout, socket_udp.recv_from(&mut msg)).await {
            Ok(val) => val,
            Err(_) => return Err(ClientError::Io(IoError::new(ErrorKind::TimedOut, format!("Error: timeout"))).into()),
        };

        let (_, src_addr) = match result {
            Err(e) => return Err(IoError::new(ErrorKind::Other, format!("Error: could not read {}", e))).map_err(Into::into),
            Ok((n, addr)) => (n, addr),
        };

        // Verify that the response comes from the expected IP
        let expected_ip = self.get_server_addr();
        let actual_ip = src_addr.ip();
        if actual_ip != expected_ip {
            return Err(ClientError::Io(IoError::new(
                ErrorKind::PermissionDenied,
                format!("IP mismatch: expected {}, got {}", expected_ip, actual_ip),
            )).into());
        }
        let ip = self.get_server_addr();
        let mut additionals = dns_query.get_additional();
        let mut ar = ARdata::new();
        ar.set_address(ip);
        let a_rdata = Rdata::A(ar);
        let rr = ResourceRecord::new(a_rdata);
        additionals.push(rr);
       
        let (fin, _) = result?;
        drop(socket_udp);
        return Ok(msg[0..fin].to_vec());
    }
    // TODO create a global variable or remove new_default dependency
    fn new_default(server_addr: IpAddr, timeout: Duration) -> Self {
        const RECOMENDED_MAX_SIZE: usize = 4000;
        Self::new(server_addr, timeout, RECOMENDED_MAX_SIZE)
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
    use crate::message::rrtype::Rrtype;
    use crate::message::rclass::Rclass;
    use super::*;
    use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
    use tokio::time::sleep;
    use crate::message::rcode::Rcode;

    const DEFAULT_SIZE: usize = 512;
    #[test]
    fn create_udp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let _conn_new = ClientUDPConnection::new(ip_addr, timeout, DEFAULT_SIZE);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    // Setters and Getters test
    #[test]
    fn get_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout, DEFAULT_SIZE);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_ip_v4(){
        let ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let connection = ClientUDPConnection::new(ip_address, timeout, DEFAULT_SIZE);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_ip_v6(){
        // ip in V6 version is the equivalent to (192, 168, 0, 1) in V4
        let ip_address = IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0));
        let timeout = Duration::from_secs(100);
        let connection = ClientUDPConnection::new(ip_address, timeout, DEFAULT_SIZE);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0)));
    }

    #[test]
    fn set_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout, DEFAULT_SIZE);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));

        _conn_new.set_server_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn get_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout, DEFAULT_SIZE);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    #[test]
    fn set_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientUDPConnection::new(ip_addr,timeout, DEFAULT_SIZE);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));

        _conn_new.set_timeout(Duration::from_secs(200));

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(200));
    }

    #[tokio::test]
    async fn send_timeout(){

        let server_addr_non_existent = IpAddr::V4(Ipv4Addr::new(234,1 ,4, 44));
        let timeout = Duration::from_secs(2);

        let conn = ClientUDPConnection::new(server_addr_non_existent, timeout, DEFAULT_SIZE);

        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_query =
        DnsMessage::new_query_message(
            domain_name,
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1);
        
        let result = conn.send(dns_query).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn send_query_udp(){

        let server_addr_non_existent = IpAddr::V4(Ipv4Addr::new(8,8 ,8, 8));
        let timeout = Duration::from_secs(2);

        let conn = ClientUDPConnection::new(server_addr_non_existent, timeout, DEFAULT_SIZE);

        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_query =
        DnsMessage::new_query_message(
            domain_name,
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1);
        
        // let response = conn.send(dns_query).unwrap();
        let response = conn.send(dns_query).await;

        assert!(response.is_ok());

        // assert!(DnsMessage::from_bytes(&response).unwrap().get_answer().len() > 0); 

        // assert!(result.unwrap().get_answer().len() > 0); FIXME:
    }

    /*
    This tests the len of the buffer for udp and also the edns0 len sent and tests if the message
    sent is less than the edns0 len broadcasted
    */
    #[tokio::test]
    async fn test_buff_size() {
        const ROOTSV1: [u8; 4] = [199,7,83,42];
        const LENGTHS: [usize; 7] = [600,650,700,750,800,850,900];
        let ip2req = ROOTSV1.into();
        for size in LENGTHS {
            let timeout = Duration::from_secs(2u64);
            let conn = ClientUDPConnection::new(ip2req, timeout, size);
            let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
            let mut dns_query =
                DnsMessage::new_query_message(
                    domain_name,
                    Rrtype::A,
                    Rclass::IN,
                    0,
                    false,
                    1);

            dns_query.add_edns0(Some(size as u16), Rcode::NOERROR, 0, false, Some(vec![]));

            let response = conn.send(dns_query).await;
            match response {
                Ok(rrs) => {
                    let recv_size = rrs.len();
                    assert!(recv_size <= size)
                },
                Err(e) => panic!("{:?}", e),
            };
            sleep(Duration::from_secs(1)).await;
        }
    }
}