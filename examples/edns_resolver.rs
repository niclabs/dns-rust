use tokio::runtime::Runtime;
use dns_rust::client::client_connection::ClientConnection;
use dns_rust::domain_name::DomainName;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rrtype::Rrtype;
use dns_rust::edns::opt_option::option_code::OptionCode;
use dns_rust::async_resolver::config::ResolverConfig;
use dns_rust::async_resolver::AsyncResolver;
use dns_rust::message::DnsMessage;

fn main() {
    let rt = Runtime::new().unwrap();

    let mut config = ResolverConfig::default();
    config.add_edns0(None, 0, true, Some(vec![OptionCode::NSID]));

    let resolver = AsyncResolver::new(config);

    let domain_name = DomainName::new_from_string("example.com".to_string());
    let rrtype = Rrtype::A;
    let record_class = Rclass::IN;

    rt.block_on(async {
        match resolver.inner_lookup(domain_name, rrtype, record_class).await {
            Ok(lookup) => {
                let message = DnsMessage::from_bytes(lookup.get_bytes().as_slice());
                match message {
                    Ok(mess) => {println!("Respuesta recibida: \n{}", mess);}
                    Err(e) => println!("Error resolving DNS message: {}", e),
                }
            }
            Err(e) => {
                println!("Error al enviar: {}", e)
            },
        }
    });
}