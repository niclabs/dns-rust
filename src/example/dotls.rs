use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use crate::client::client_security::ClientSecurity;
use crate::client::tls_connection::ClientTLSConnection;
use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;
use crate::message::rrtype::Rrtype;
use crate::message::DnsMessage;
use crate::client::client_error::ClientError;
fn main() -> Result<(), ClientError> {
    let rt = Runtime::new().unwrap();

    let server_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));

    let timeout = Duration::from_secs(5);

    let tls_connection = ClientTLSConnection::new_default(server_ip, timeout);

    let dns_query = DnsMessage::new_query_message(DomainName::new_from_string("example.com".to_string()),
    Rrtype::A,
    Rclass::IN,
    0,
    false,
    1);

    rt.block_on(async {
        match tls_connection.send(dns_query).await {
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