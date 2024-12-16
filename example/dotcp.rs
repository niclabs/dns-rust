
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use crate::client::tcp_connection::ClientTCPConnection;
use crate::message::DnsMessage;
use crate::client::client_error::ClientError;

fn main() -> Result<(), ClientError> {
    let rt = Runtime::new().unwrap();

   
    let server_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
   
    let timeout = Duration::from_secs(5);

  
    let tcp_connection = ClientTCPConnection::new(server_ip, timeout);

    
    let dns_query = DnsMessage::new_query("example.com", crate::message::rrtype::Rrtype::A);


    rt.block_on(async {
        match tcp_connection.send(dns_query).await {
            Ok(response) => {
                println!("Respuesta recibida: {:?}", response);
            }
            Err(e) => {
                eprintln!("Error al enviar la consulta DNS: {:?}", e);
            }
        }
    });

    Ok(())
}