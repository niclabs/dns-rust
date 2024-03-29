use crate::client::ClientConnection;
use crate::message::DnsMessage;
use super::client_error::ClientError;


use std::io::{Write, Read};
use std::net::{TcpStream,SocketAddr,IpAddr};
use std::time::Duration;
use std::io::Error as IoError;
use std::io::ErrorKind;


#[derive(Clone,Copy, Debug, PartialEq, Eq)]
pub struct ClientTCPConnection {
    /// Client address
    server_addr: IpAddr,
    /// Read time timeout
    timeout: Duration,
}

impl ClientConnection for ClientTCPConnection {

    ///Creates UDPConnection
    fn new(server_addr:IpAddr, timeout:Duration) -> Self {
        ClientTCPConnection {
            server_addr: server_addr,
            timeout: timeout,
        }
    }

    /// creates socket tcp, sends query and receive response
    fn send(self, dns_query: DnsMessage) -> Result<Vec<u8>, ClientError>{
        
        let timeout: Duration = self.get_timeout();
        let bytes: Vec<u8> = dns_query.to_bytes();
        let server_addr:SocketAddr = SocketAddr::new(self.get_server_addr(), 53);

        let mut stream: TcpStream = TcpStream::connect_timeout(&server_addr,timeout)?;
    
        //Add len of message len
        let msg_length: u16 = bytes.len() as u16;
        let tcp_bytes_length: [u8; 2] = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg: Vec<u8> = [&tcp_bytes_length, bytes.as_slice()].concat();
        
        stream.set_read_timeout(Some(timeout))?; 

        stream.write(&full_msg)?;
    
        //Read response
        let mut msg_size_response: [u8; 2] = [0; 2];

        stream.read_exact(&mut msg_size_response)?;
    
        let tcp_msg_len: u16 = (msg_size_response[0] as u16) << 8 | msg_size_response[1] as u16;
        let mut vec_msg: Vec<u8> = Vec::new();
    
        while vec_msg.len() < tcp_msg_len as usize {
            let mut msg = [0; 512];
            let number_of_bytes_msg = match stream.read(&mut msg) {
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
    use std::net::{IpAddr,Ipv4Addr};
    use crate::domain_name::DomainName;
    use crate::message::type_qtype::Qtype;
    use crate::message::class_qclass::Qclass;
        

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

    #[test]
    fn send_query_tcp(){

        let ip_addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let _port: u16 = 8088;
        let timeout = Duration::from_secs(2);

        let conn_new = ClientTCPConnection::new(ip_addr,timeout);
        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_query =
        DnsMessage::new_query_message(
            domain_name,
            Qtype::A,
            Qclass::IN,
            0,
            false,
            1);
        let response = conn_new.send(dns_query).unwrap();
        
        assert!(DnsMessage::from_bytes(&response).unwrap().get_answer().len() > 0); 
        // FIXME:
    }

    #[test]
    fn send_timeout() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let _port: u16 = 8088;
        let timeout = Duration::from_secs(2);

        let conn_new = ClientTCPConnection::new(ip_addr,timeout);
        let dns_query = DnsMessage::new();
        let response = conn_new.send(dns_query);

        assert!(response.is_err());
    }




}