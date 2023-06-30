use crate::client::ClientConnection;
use crate::message::{DnsMessage};

use std::io::{Write, Read};
use std::net::{TcpStream,SocketAddr,IpAddr};
use std::time::Duration;


pub struct ClientTCPConnection {
    //addr client
    server_addr: IpAddr,
    //timeout read time
    timeout: Duration,
}

impl ClientConnection for ClientTCPConnection {

    ///Creates UDPConnection
    fn new(server_addr:IpAddr, timeout:Duration) -> ClientTCPConnection {
        ClientTCPConnection {
            server_addr: server_addr,
            timeout: timeout,
        }
    }

    /// creates socket tcp, sends query and receive response
    fn send(&self, dns_query: DnsMessage) -> DnsMessage {
        println!("[SEND TCP]");
        let timeout: Duration = self.get_timeout();
        let bytes: Vec<u8> = dns_query.to_bytes();
        let server_addr:SocketAddr = SocketAddr::new(self.get_server_addr(), 53);

        let mut stream: TcpStream = TcpStream::connect(server_addr)
            .unwrap_or_else(|e| panic!("Error connect {}", e));
    
        //Add len of message len
        let msg_length: u16 = bytes.len() as u16;
        let tcp_bytes_length: [u8; 2] = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg: Vec<u8> = [&tcp_bytes_length, bytes.as_slice()].concat();
    
        //Set read timeout
        match stream.set_read_timeout(Some(timeout)) {
            Err(_) => panic!("Error setting read timeout for socket"),
            Ok(_) => (),
        }
    
        match stream.write(&full_msg) {
            Err(_) => panic!("Error: could not write to stream"),
            Ok(_) => (),
        }
        println!("[SEND TCP] query sent");
    
        //Read response
        let mut msg_size_response: [u8; 2] = [0; 2];
        stream.read_exact(&mut msg_size_response).expect("No data received");
    
        let tcp_msg_len: u16 = (msg_size_response[0] as u16) << 8 | msg_size_response[1] as u16;
        let mut vec_msg: Vec<u8> = Vec::new();
    
        while vec_msg.len() < tcp_msg_len as usize {
            let mut msg = [0; 512];
            let number_of_bytes_msg = stream.read(&mut msg).expect("No data received");
            vec_msg.extend_from_slice(&msg[..number_of_bytes_msg]);
        }

        let response_dns: DnsMessage = match DnsMessage::from_bytes(&vec_msg) {
            Ok(response) => response,
            Err(_) => panic!("Error parsing DNS query"),
        };
        // println!("[SEND TCP] {:?}", vec_msg);
    
        return response_dns;
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
    use std::net::{SocketAddr,IpAddr,Ipv4Addr};

    #[test]
    fn create_tcp() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port: u16 = 8088;
        let timeout = Duration::from_secs(100);

        let _conn_new = ClientTCPConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }
}