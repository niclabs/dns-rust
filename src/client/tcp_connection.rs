use crate::client::ClientConnection;
use crate::message::DnsMessage;
use crate::message::rdata::Rdata;
use crate::message::rdata::a_rdata::ARdata;
use crate::message::resource_record::ResourceRecord;
use super::client_error::ClientError;
use async_trait::async_trait;
use futures_util::TryFutureExt;
use rustls::pki_types::ServerName;
use rustls::server;
use rustls::Stream;
use webpki::DnsNameRef;
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::iter::FromIterator;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use std::net::IpAddr;
use std::net::SocketAddr;
use tokio::time::Duration;
use tokio::time::timeout;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::TlsConnector;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClientTCPConnection {
    /// Client address
    server_addr: IpAddr,
    /// Read time timeout
    timeout: tokio::time::Duration,
}

#[async_trait]
impl ClientConnection for ClientTCPConnection {

    /// Creates TCPConnection
    fn new(server_addr:IpAddr, timeout: Duration) -> Self {
        ClientTCPConnection {
            server_addr: server_addr,
            timeout: timeout,
        }
    }

    ///implement get_ip
    /// returns IpAddr
    fn get_ip(&self) -> IpAddr {
        return self.server_addr.clone();
    }

    /// creates socket tcp, sends query and receive response
    async fn send(self, dns_query: DnsMessage) -> Result<Vec<u8>, ClientError> {
    // async fn send(self, dns_query: DnsMessage) -> Result<(Vec<u8>, IpAddr), ClientError> {
        
        
        let bytes: Vec<u8> = dns_query.to_bytes();
       
    
        // Add len of message len
        let msg_length: u16 = bytes.len() as u16;
        let tcp_bytes_length: [u8; 2] = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg: Vec<u8> = [&tcp_bytes_length, bytes.as_slice()].concat();
        
        //get domain name
        let domain_name = dns_query.get_question().get_qname().get_name();
        let dns_name = DnsNameRef::try_from_ascii_str(&domain_name);
        if dns_name.is_err() {
            return Err(ClientError::Io(IoError::new(ErrorKind::InvalidInput, format!("Error: invalid domain name"))).into());
        }

        let root_store = rustls::RootCertStore::from_iter(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .cloned(),
        );
        let config = rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth();
        let rc_config = Arc::new(config);
       

        let dns_name = domain_name;
        let server_name =ServerName::try_from(dns_name).expect("invalid DNS name");
        let connector = rustls::ClientConnection::new(rc_config, server_name).unwrap();
        let conn_timeout: Duration = self.get_timeout();
        let server_addr:SocketAddr = SocketAddr::new(self.get_server_addr(), 853);

        // let mut stream: TcpStream = TcpStream::connect_timeout(&server_addr,timeout)?;
        let conn_task = TcpStream::connect(&server_addr);
        let mut stream: TcpStream = TcpStream::connect(server_addr).await?;
        // stream.set_read_timeout(Some(timeout))?; //-> Se hace con tokio

        // stream.write(&full_msg)?;
        stream.write(&full_msg).await?;
        
        // Read response
        let mut msg_size_response: [u8; 2] = [0; 2];

        stream.read_exact(&mut msg_size_response).await?;
    
        let tcp_msg_len: u16 = (msg_size_response[0] as u16) << 8 | msg_size_response[1] as u16;
        let mut vec_msg: Vec<u8> = Vec::new();
        let ip = self.get_server_addr();
        let mut additionals = dns_query.get_additional();
        let mut ar = ARdata::new();
        ar.set_address(ip);
        let a_rdata = Rdata::A(ar);
        let rr = ResourceRecord::new(a_rdata);
        additionals.push(rr);
        
    
        while vec_msg.len() < tcp_msg_len as usize {
            let mut msg = [0; 512];
            let read_task = stream.read(&mut msg);
            let number_of_bytes_msg_result = match timeout(conn_timeout, read_task).await {
                Ok(n) => n,
                Err(_) => return Err(ClientError::Io(IoError::new(ErrorKind::TimedOut, format!("Error: timeout"))).into()),
            };

            let number_of_bytes_msg = match number_of_bytes_msg_result {
                Ok(n) if n > 0 => n,
                _ => return Err(IoError::new(ErrorKind::Other, format!("Error: no data received "))).map_err(Into::into),
                
            };

            vec_msg.extend_from_slice(&msg[..number_of_bytes_msg]);
        }

        return Ok(vec_msg);
    }
}

//Getters
impl ClientTCPConnection {

    pub fn get_server_addr(&self)-> IpAddr {
        return self.server_addr.clone();
    }

    pub fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl ClientTCPConnection {

    pub fn set_server_addr(&mut self,addr :IpAddr) {
        self.server_addr = addr;
    }

    pub fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}

#[cfg(test)]
mod tcp_connection_test{
    use super::*;
    use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
    use crate::domain_name::DomainName;
    use crate::message::rrtype::Rrtype;
    use crate::message::rclass::Rclass;

    #[test]
    fn create_tcp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let _port: u16 = 8088;
        let timeout = Duration::from_secs(100);

        let _conn_new = ClientTCPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    #[test]
    fn get_ip_v4(){
        let ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let connection = ClientTCPConnection::new(ip_address, timeout);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_ip_v6(){
        // ip in V6 version is the equivalent to (192, 168, 0, 1) in V4
        let ip_address = IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0));
        let timeout = Duration::from_secs(100);
        let connection = ClientTCPConnection::new(ip_address, timeout);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0)));
    }

    //Setters and Getters test
    #[test]
    fn get_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTCPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn set_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTCPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));

        _conn_new.set_server_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn get_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTCPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    #[test]
    fn set_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTCPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));

        _conn_new.set_timeout(Duration::from_secs(200));

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(200));
    }

    #[tokio::test]
    async fn send_query_tcp(){

        let ip_addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let _port: u16 = 8088;
        let timeout = Duration::from_secs(2);

        let conn_new = ClientTCPConnection::new(ip_addr,timeout);
        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_query =
        DnsMessage::new_query_message(
            domain_name,
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1);
        let response = conn_new.send(dns_query).await.unwrap();
        // let (response, _ip) = conn_new.send(dns_query).await.unwrap();
        
        assert!(DnsMessage::from_bytes(&response).unwrap().get_answer().len() > 0); 
        // FIXME:
    }

    #[tokio::test]
    async fn send_timeout() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let _port: u16 = 8088;
        let timeout = Duration::from_secs(2);

        let conn_new = ClientTCPConnection::new(ip_addr,timeout);
        let dns_query = DnsMessage::new();
        let response = conn_new.send(dns_query).await;

        assert!(response.is_err());
    }




}