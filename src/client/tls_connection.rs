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
use rustls::RootCertStore;
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
pub struct ClientTLSConnection {
    /// Client address
    server_addr: IpAddr,
    /// Read time timeout
    timeout: tokio::time::Duration,
}

#[async_trait]
impl ClientConnection for ClientTLSConnection {

    /// Creates TCPConnection
    fn new(server_addr:IpAddr, timeout: Duration) -> Self {
        ClientTLSConnection {
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
            
            let conn_timeout: Duration = self.get_timeout();
            let bytes: Vec<u8> = dns_query.to_bytes();
            let server_addr:SocketAddr = SocketAddr::new(self.get_server_addr(), 53);
    
            // let mut stream: TcpStream = TcpStream::connect_timeout(&server_addr,timeout)?;
            let conn_task = TcpStream::connect(&server_addr);
            let mut stream: TcpStream = match timeout(conn_timeout, conn_task).await {
                Ok(stream_result) => stream_result?,
                Err(_) => return Err(ClientError::Io(IoError::new(ErrorKind::TimedOut, format!("Error: timeout"))).into()),
            };
        
            // Add len of message len
            let msg_length: u16 = bytes.len() as u16;
            let tcp_bytes_length: [u8; 2] = [(msg_length >> 8) as u8, msg_length as u8];
            let full_msg: Vec<u8> = [&tcp_bytes_length, bytes.as_slice()].concat();
            
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
impl ClientTLSConnection {

    pub fn get_server_addr(&self)-> IpAddr {
        return self.server_addr.clone();
    }

    pub fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }


}

//Setters
impl ClientTLSConnection {

    pub fn set_server_addr(&mut self,addr :IpAddr) {
        self.server_addr = addr;
    }

    pub fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}

#[cfg(test)]
mod tls_connection_test{
   

}