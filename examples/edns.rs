use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::runtime::Runtime;
use dns_rust::client::Client;
use dns_rust::client::client_connection::ClientConnection;
use dns_rust::client::udp_connection::ClientUDPConnection;
use dns_rust::domain_name::DomainName;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rrtype::Rrtype;
use dns_rust::message::DnsMessage;
use dns_rust::client::client_error::ClientError;
use dns_rust::edns::opt_option::option_code::OptionCode;
use dns_rust::message::rcode::Rcode;

fn main() {

    let rt = Runtime::new().unwrap();

    let addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);

    // message
    let mut dns_query_message =
        DnsMessage::new_query_message(
            DomainName::new_from_string("nonexistent.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            true,
            1);

    dns_query_message.add_edns0(Option::from(4096), Rcode::NOERROR, 0, true, Some(vec![OptionCode::NSID, OptionCode::EDE]));

    client.set_dns_query(dns_query_message);

    rt.block_on(async {
        match client.send_query().await {
            Ok(response) => {
                println!("Respuesta recibida: {:?}", response);
            }
            Err(err) => {
                println!("Error al enviar: {:?}", err);
            }
        }
    });
}