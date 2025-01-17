use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use dns_rust::client::client_connection::ClientConnection;
use dns_rust::client::udp_connection::ClientUDPConnection;
use dns_rust::domain_name::DomainName;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rrtype::Rrtype;
use dns_rust::message::DnsMessage;
use dns_rust::client::client_error::ClientError;

fn main() -> Result<(), ClientError> {
    // Crear una instancia de Runtime para ejecutar tareas asincrónicas
    let rt = Runtime::new().unwrap();

    // Dirección IP del servidor DNS
    let server_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    // Tiempo de espera para la conexión
    let timeout = Duration::from_secs(5);

    // Crear una instancia de ClientUDPConnection
    let udp_connection = ClientUDPConnection::new_default(server_ip, timeout);

    // Crear una consulta DNS (esto es solo un ejemplo, ajusta según tu implementación)
    let dns_query = DnsMessage::new_query_message(DomainName::new_from_string("example.com".to_string()),
    Rrtype::A,
    Rclass::IN,
    0,
    false,
    1);

    // Ejecutar la tarea asincrónica en el Runtime
    rt.block_on(async {
        match udp_connection.send(dns_query).await {
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