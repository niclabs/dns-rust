use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use dns_rust::client::tls_connection::ClientTLSConnection;
use dns_rust::message::DnsMessage;
use dns_rust::client::client_error::ClientError;

fn main() -> Result<(), ClientError> {
    // Crear una instancia de Runtime para ejecutar tareas asincrónicas
    let rt = Runtime::new().unwrap();

    // Dirección IP del servidor DNS
    let server_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    // Tiempo de espera para la conexión
    let timeout = Duration::from_secs(5);

    // Crear una instancia de ClientTLSConnection
    let tls_connection = ClientTLSConnection::new(server_ip, timeout);

    // Crear una consulta DNS (esto es solo un ejemplo, ajusta según tu implementación)
    let dns_query = DnsMessage::new_query("example.com", crate::message::rrtype::Rrtype::A);

    // Ejecutar la tarea asincrónica en el Runtime
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