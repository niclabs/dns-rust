use std::{net::IpAddr, str::FromStr, thread, net::UdpSocket, time::Duration};
use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver}, client::client_error::ClientError, domain_name::DomainName, message::{rdata::Rdata,class_qclass::Qclass, type_qtype, resource_record::ResourceRecord, header::Header, DnsMessage},tsig::{self, TsigAlgorithm}};

///RFC 8945 TSIG tests
/*This tests verifies section 5.3:
   When a server has generated a response to a signed request, it signs
   the response using the same algorithm and key.  The server MUST NOT
   generate a signed response to a request if either the key is invalid
   (e.g., key name or algorithm name are unknown) or the MAC fails
   validation; see Section 5.3.2 for details of responding in these
   cases.

   It also MUST NOT generate a signed response to an unsigned request,
   except in the case of a response to a client's unsigned TKEY request
   if the secret key is established on the server side after the server
   processed the client's request.  Signing responses to unsigned TKEY
   requests MUST be explicitly specified in the description of an
   individual secret key establishment algorithm [RFC3645].

   The digest components used to generate a TSIG on a response are:

      Request MAC
      DNS Message (response)
      TSIG Variables (response) */
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
   