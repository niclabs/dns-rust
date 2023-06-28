use crate::client::ClientConnection;
use crate::message::{DnsMessage};

use std::io::{Write, Read};
use std::net::{TcpStream,SocketAddr,IpAddr};
use std::time::Duration;


pub struct TCPConnection {
    //addr client
    bind_addr: IpAddr,
    //timeout read time
    timeout: Duration,
}

impl ClientConnection for TCPConnection {

    ///Creates UDPConnection
    fn new(bind_addr:IpAddr, timeout:Duration) -> TCPConnection {
        TCPConnection {
            bind_addr: bind_addr,
            timeout: timeout,
        }
    }

    ///creates socket tcp, sends query and receive response
    fn send(&self, dns_query: DnsMessage) -> DnsMessage {
        println!("[SEND TCP]");
        let timeout: Duration = self.get_timeout();
        let bytes: Vec<u8> = dns_query.to_bytes();
        let bind_addr:SocketAddr = SocketAddr::new(self.get_bind_addr(), 53);

        let mut stream: TcpStream = TcpStream::connect(bind_addr)
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
    
        return  response_dns;
    }
}

//Getters
impl TCPConnection {

    fn get_bind_addr(&self)-> IpAddr {
        return self.bind_addr.clone();
    }

    fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl TCPConnection {

    fn set_bind_addr(&mut self,addr :IpAddr) {
        self.bind_addr = addr;
    }

    fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}



#[cfg(test)]
mod tcp_connection_test{
    
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
;       let conn_udp:TCPConnection = ClientConnection::new(ip_addr_to_connect,timeout);

        //Query
        let dns_query = DnsMessage::new_query_message("uchile.cl".to_string(),
                                                "A".to_string(),
                                                "IN".to_string(),
                                                0, false, 111);
        
        let mut response = conn_udp.send(dns_query);
        response.print_dns_message();

    }
}