use std::{net::IpAddr, str::FromStr, thread, net::UdpSocket, time::Duration};
use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver}, client::client_error::ClientError, domain_name::DomainName, message::{rdata::Rdata,class_qclass::Qclass, type_qtype, resource_record::ResourceRecord, header::Header, DnsMessage},tsig::{self, TsigAlgorithm}};



// TODO: Change params type to intoDomainName
async fn query_response(domain_name: &str, protocol: &str, qtype: &str) -> Result<Vec<ResourceRecord>, ClientError> {

    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(
        domain_name,
        protocol,
        qtype,

        "IN").await;

    response.map(|lookup_response| lookup_response.to_vec_of_rr())
}

/// 6.2.1 Query test Qtype = A
#[tokio::test]
async fn query_a_type() {
    let response = query_response("example.com", "UDP", "A").await;

    if let Ok(rrs) = response {
        assert_eq!(rrs.iter().count(), 1);
        let rdata = rrs[0].get_rdata();
        if let Rdata::A(ip) = rdata {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.216.34").unwrap());
        } else {
            panic!("No ip address");
        }
    } 
}

/// 6.2.2 Query normal Qtype = *
#[tokio::test]
/// Ignored due to halting problem
#[ignore]
async fn query_any_type() {
    let udp_response = query_response("example.com", "UDP", "ANY").await;
    let tcp_response = query_response("example.com", "TCP", "ANY").await;
    assert!(udp_response.is_err());
    assert!(tcp_response.is_err());
}

/// 6.2.3 Query Qtype = MX
#[tokio::test]
async fn query_mx_type() {
    let response = query_response("example.com", "UDP", "MX").await;
    
    if let Ok(rrs) = response {
        assert_eq!(rrs.len(), 1);

        if let Rdata::MX(mxdata) = rrs[0].get_rdata() {
            assert_eq!(
                mxdata.get_exchange(),
                DomainName::new_from_str(""));

            assert_eq!(
                mxdata.get_preference(),
                0
            )
        } else { 
            panic!("Record is not MX type");
        }
    }
}


// 6.2.4 Query Qtype = NS
#[tokio::test]
async fn query_ns_type() {
    let response = query_response("example.com", "UDP", "NS").await;
    if let Ok(rrs) = response {
        assert_eq!(rrs.len(), 2);
        
        if let Rdata::NS(ns1) = rrs[0].get_rdata() {
            assert_eq!(
                ns1.get_nsdname(),
                DomainName::new_from_str("a.iana-servers.net"))
        } else { 
            panic!("First record is not NS");
        }
        
        if let Rdata::NS(ns) = rrs[1].get_rdata() {
            assert_eq!(
                ns.get_nsdname(),
                DomainName::new_from_str("b.iana-servers.net"))
        } else {
            panic!("Second record is not NS");
        }
    }
}

/// 6.2.5 Mistyped host name Qtype = A
#[tokio::test]
async fn mistyped_host_name() {
    let response = query_response("exampllee.com", "UDP", "A").await;
    assert!(response.is_err());
}

/// No record test
#[tokio::test]
async fn no_resource_available() {
    let response =  query_response("example.com", "UDP", "CNAME").await;
    println!("{:?}", response);
    assert!(response.is_err());
}


///RFC 8945 TSIG tests
#[tokio::test]
async fn tsig_signature() {
    // global test variables
    let key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha1;
    let fudge = 0;
    let time_signed = 0;
    let id = 6502; 
    let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("nictest.cl".to_string()),
                type_qtype::Qtype::A,
                Qclass::IN,
                0,
                false,
                id);
    let q_for_mac = dns_query_message.clone();
    //Lanzamiento de threads
    //Se lanza el servidor. Recibe un mensaje sin firmar, lo firma y lo reenvía
    fn host(){
        println!("I am a host");
        let udp_socket = UdpSocket::bind("127.0.0.1:8002").expect("Failed to bind to address");
        let mut buf = [0; 512];
        
        match udp_socket.recv_from(&mut buf) {
        
        Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut data = DnsMessage::from_bytes(&buf[0..size]).unwrap();
                println!("The data is {:?}", data);
                let key_name = "".to_string();
                tsig::sign_tsig(&mut data, b"1234567890",TsigAlgorithm::HmacSha1,0,0, key_name);
                let response = &DnsMessage::to_bytes(&data);
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
                
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                
            }
        }
        
    }
    println!("Starting server");
    let server_handle = thread::spawn(|| {
        host();  
        
    });
    thread::sleep(Duration::from_secs(2)); 
    // se instancia un socket cliente que enviará y  mensajes
    let client_sock = UdpSocket::bind("127.0.0.1:8001").expect("Nothing");
    let buf = dns_query_message.to_bytes();
    client_sock.send_to(&buf,"127.0.0.1:8002").unwrap();
    println!("Mensaje enviado");
    server_handle.join().unwrap();


}
   