use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use crate::client::client_connection::ClientConnection;
use crate::client::udp_connection::ClientUDPConnection;
use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;
use crate::message::rrtype::Rrtype;
use crate::message::DnsMessage;
use crate::client::client_error::ClientError;

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