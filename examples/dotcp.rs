
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use dns_rust::client::client_connection::ClientConnection;
use dns_rust::client::tcp_connection::ClientTCPConnection;
use dns_rust::domain_name::DomainName;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rrtype::Rrtype;
use dns_rust::message::DnsMessage;
use dns_rust::client::client_error::ClientError;

pub mod dotls;
pub mod doudp;

fn main() -> Result<(), ClientError> {
    let rt = Runtime::new().unwrap();

   
    let server_ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
   
    let timeout = Duration::from_secs(5);

  
    let tcp_connection = ClientTCPConnection::new_default(server_ip, timeout);

    
    let dns_query = DnsMessage::new_query_message(DomainName::new_from_string("example.com".to_string()),
    Rrtype::A,
    Rclass::IN,
    0,
    false,
    1);


    rt.block_on(async {
        match tcp_connection.send(dns_query).await {
            Ok(response) => {
                let message = DnsMessage::from_bytes(response.as_slice());
                match message {
                    Ok(mess) => {println!("Respuesta recibida: \n{}", mess);}
                    Err(e) => println!("Error resolving DNS message: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error al enviar la consulta DNS: {:?}", e);
            }
        }
    });

    Ok(())
}