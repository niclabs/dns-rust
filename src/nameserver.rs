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
mod ns_tests {
    use super::*;
    use futures_util::future;
    use tokio::time::{timeout, Duration};
    use crate::message::rdata::mx_rdata::MxRdata;
    use crate::message::DnsMessage;
    use crate::domain_name::DomainName;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::rclass::Rclass;
    use crate::message::rrtype::Rrtype;
    use crate::message::rcode::Rcode;

    #[tokio::test]
    async fn test_name_server_init() {
        let mut server = NameServer {
            zone: vec![
                // Some sample RRs (will be cleared by init)
                {
                    let mut rr = ResourceRecord::new(Rdata::A(ARdata::new()));
                    rr.set_name(DomainName::new_from_string("example.com".to_string()));
                    rr.set_rclass(Rclass::IN);
                    rr.set_type_code(Rrtype::A);
                    rr
                }
            ],
            shared_sock: Arc::new(Mutex::new(
                UdpSocket::bind("127.0.0.1:0").await.unwrap()
            )),
        };

        server.init("127.0.0.1:0").await.unwrap();
        assert_eq!(server.zone.len(), 0, "Zone should be cleared on init");

        let socket = server.shared_sock.lock().await;
        assert_ne!(socket.local_addr().unwrap().port(), 0, "Should bind a valid port");
    }

    #[test]
    fn test_name_server_add_rrs() {
        let mut message = DnsMessage::new();
        let mut rr = ResourceRecord::new(Rdata::A(ARdata::new()));
        rr.set_name(DomainName::new_from_string("example.com".to_string()));
        rr.set_rclass(Rclass::IN);
        rr.set_type_code(Rrtype::A);

        let rrs = vec![rr.clone()];
        NameServer::add_rrs(&mut message, &rrs);

        assert_eq!(message.get_answer().len(), 1, "Should have one RR in answer");
        assert_eq!(message.get_answer()[0], rr);

        let header = message.get_header();
        assert!(header.get_aa(), "AA flag should be set");
        assert!(header.get_qr(), "QR flag should be set (response)");
        assert_eq!(header.get_ancount(), 1, "ANCOUT should be 1");
        assert_eq!(header.get_rcode(), Rcode::NOERROR, "Rcode should be NOERROR");
    }

    #[test]
    fn test_name_server_search_query() {
        let mut rr_a = ResourceRecord::new(Rdata::A(ARdata::new()));
        rr_a.set_name(DomainName::new_from_string("example.com".to_string()));
        rr_a.set_type_code(Rrtype::A);
        rr_a.set_rclass(Rclass::IN);

        let mut rr_mx = ResourceRecord::new(Rdata::MX(MxRdata::new())); // dummy Rdata, for example
        rr_mx.set_name(DomainName::new_from_string("example.com".to_string()));
        rr_mx.set_type_code(Rrtype::MX);
        rr_mx.set_rclass(Rclass::IN);

        let mut rr_other = ResourceRecord::new(Rdata::A(ARdata::new()));
        rr_other.set_name(DomainName::new_from_string("example.org".to_string()));
        rr_other.set_type_code(Rrtype::A);
        rr_other.set_rclass(Rclass::IN);

        let zone = vec![rr_a.clone(), rr_mx, rr_other];

        let mut query_message = DnsMessage::new();
        {
            let mut question = query_message.get_question();
            question.set_qname(DomainName::new_from_string("example.com".to_string()));
            question.set_rrtype(Rrtype::A);
            question.set_rclass(Rclass::IN);
            query_message.set_question(question);
        }

        let found = NameServer::search_query(&zone, &query_message);
        assert_eq!(found.len(), 1, "Should find exactly one matching RR");
        assert_eq!(found[0], rr_a, "Should match the A record for example.com");
    }

    #[tokio::test]
    async fn test_name_server_run() {
        let mut server = NameServer {
            zone: vec![],
            shared_sock: Arc::new(Mutex::new(
                UdpSocket::bind("127.0.0.1:0").await.unwrap()
            )),
        };

        let local_addr = server.shared_sock.lock().await.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            // This loop runs indefinitely, so we rely on a timeout in the test
            let _ = server.run(&local_addr.to_string()).await;
        });

        //time to bind/listen
        tokio::time::sleep(Duration::from_millis(100)).await;

        let test_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let query_bytes = DnsMessage::new().to_bytes(); 
        let _ = test_socket.send_to(&query_bytes, &local_addr).await;
        let _ = timeout(Duration::from_millis(300), handle).await;
    }
    #[ignore]
    #[tokio::test]
    async fn test_concurrency_with_timeout() {
        let mut server = NameServer {
            zone: vec![], // or some test zone RRs if needed
            shared_sock: Arc::new(Mutex::new(
                UdpSocket::bind("127.0.0.1:0").await.unwrap()
            )),
        };

        // Get the server's local address
        let local_addr = server.shared_sock.lock().await.local_addr().unwrap();

        // Run the server in a background task
        let server_task = tokio::spawn(async move {
            let _ = server.run(&local_addr.to_string()).await;
        });

        // We will spawn multiple parallel "clients"
        let num_clients = 5;
        let tasks = (0..num_clients).map(|i| {
            tokio::spawn(async move {

                let sock = UdpSocket::bind("127.0.0.1:0").await.unwrap();

                let query = DnsMessage::new().to_bytes();
                sock.send_to(&query, local_addr).await.unwrap();


                let mut buf = vec![0u8; 1024];
                let (bytes_received, from_addr) = sock.recv_from(&mut buf).await.unwrap();


                assert!(bytes_received > 0, "Should receive some data from the server");
                assert_eq!(from_addr.ip(), local_addr.ip(), "Response should come from the server IP");


                i
            })
        });

        // Wait for all client tasks to complete
        let results = futures_util::future::join_all(tasks).await;
        for (i, res) in results.into_iter().enumerate() {
            let val = res.expect("Task panicked");
            assert_eq!(val, i);
        }

        // After client tasks are done, we forcibly stop the server with a timeout
        match tokio::time::timeout(Duration::from_millis(300), server_task).await {
            Ok(_) => {
            }
            Err(_elapsed) => {
            }
        }
    }
   

}