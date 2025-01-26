use std::{io, vec};
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::net::UdpSocket;

use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::message::rcode::Rcode;

/// Structure to represent a Name Server
#[derive (Debug, Clone)]
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
        let mut buf = vec![0u8; 1024]; // Specify a buffer size for receiving data.
        self.init(addr).await?;
    
        let shared_sock = self.shared_sock.clone();
        let zone = self.zone.clone(); // Clone the zone for use in the spawned tasks.
    
        loop {
            let (len, src) = shared_sock.lock().await.recv_from(&mut buf).await?;
            println!("Received {} bytes from {:?}", len, addr);
    
            let data = buf[..len].to_vec();
            let zone_clone = zone.clone();
            let socket_clone = shared_sock.clone();
            // Handle the request concurrently
    
            tokio::spawn(async move {
                let message = DnsMessage::from_bytes(&data).expect("Error parsing the message");
    
                // Search for resource records in the cloned zone
                let rrs_to_add = NameServer::search_query(&zone_clone, &message);
    
                if !rrs_to_add.is_empty() {
                    let mut message = message;
                    NameServer::add_rrs(&mut message, &rrs_to_add);
    
                    let response = message.to_bytes();
                    let mut sock = socket_clone.lock().await;
    
                    if let Err(e) = sock.send_to(&response, src).await {
                        eprintln!("Failed to send response to {}: {}", src, e);
                    } else {
                        println!("Sent response to {:?}", src);
                    }
                }
            });
        }
    }

    /*async fn handle_request(&self, socket: Arc<Mutex<UdpSocket>>,
        data: Vec<u8>,
        addr: std::net::SocketAddr) {
            let mut message = DnsMessage::from_bytes(&data).expect("Error al parsear el mensaje");

            let rrs_to_add = NameServer::search_query(&self.zone, &message);

            if rrs_to_add.len() > 0 {
                NameServer::add_rrs(&mut message, &rrs_to_add)
            }
            let response = message.to_bytes();
            // lock the socket and send the response
            let mut sock = socket.lock().await;
            if let Err(e) = sock.send_to(&response, addr).await {
                eprintln!("Failed to send response to {}: {}", addr, e);
            } else {
                println!("Sent response to {:?}", addr);
            }
    }*/

    fn add_rrs(msg :&mut DnsMessage, rrs: &Vec<ResourceRecord>) {
        msg.set_answer(rrs.clone());
        let mut header = msg.get_header();
        header.set_aa(true);
        header.set_rcode(Rcode::NOERROR);
        header.set_qr(true);
        header.set_ancount(rrs.len() as u16);
        msg.set_header(header);
    }

    fn search_query(zone: &Vec<ResourceRecord>, msg: &DnsMessage) -> Vec<ResourceRecord> {
        let qclass = msg.get_question().get_rclass();
        let qtype = msg.get_question().get_rrtype();
        let qname = msg.get_question().get_qname();
        zone.iter()
            .filter(|rr|
                qclass == rr.get_rclass()
                    && qtype == rr.get_rtype()
                    && qname == rr.get_name())
            .cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    pub fn test_response() {
        
    }
}