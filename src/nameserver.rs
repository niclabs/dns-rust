use std::{io, vec};
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::net::UdpSocket;

use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;


/// Structure to represent a Name Server
#[derive (Debug)]
pub struct NameServer {
    zone: Vec<ResourceRecord>, // Each zone is associated with a domain.
    shared_sock: Arc<Mutex<UdpSocket>>,
}

impl NameServer {

    pub async fn init(&mut self ,addr: &str) -> io::Result<()> {
        self.zone = vec![];
        self.shared_sock = Arc::new(Mutex::new(UdpSocket::bind(addr).await?));
        Ok(())
    }

    pub async fn run(&mut self, addr: &str) -> io::Result<()> {
        let mut buf = vec![];
        self.init(addr).await?;
        loop {
            let (len, src) = self.shared_sock.lock().await.recv_from(&mut buf).await?;
            println!("Received {} bytes from {:?}", len, addr);

            // Spawn a new task to process the request
            let data = buf[..len].to_vec();
            let socket_clone = self.shared_sock.clone();
            tokio::spawn(async move {
                // Handle the request concurrently!!! Important
                NameServer::handle_request(socket_clone, data, src).await;
            });
        }
    }

    async fn handle_request(socket: Arc<Mutex<UdpSocket>>,
        data: Vec<u8>,
        addr: std::net::SocketAddr) {
            let message = DnsMessage::from_bytes(&data).unwrap();
            // respondemos lo mismo por el momento
            let response = message.to_bytes();

            // lock the socket and send the response
            let mut sock = socket.lock().await;
            if let Err(e) = sock.send_to(&response, addr).await {
                eprintln!("Failed to send response to {}: {}", addr, e);
            } else {
                println!("Sent response to {:?}", addr);
            }
        }
}


#[cfg(test)]
mod tests {
    use super::*;
    pub fn test_response() {
        
    }
}