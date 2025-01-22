use std::{time::Duration, net::IpAddr};

use dns_rust::{
    async_resolver::{
            config::ResolverConfig, AsyncResolver, server_info::ServerInfo
        }, client::{
        client_connection::ClientConnection, client_error::ClientError, tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection, Client}, domain_name::DomainName, message::resource_record::ResourceRecord};

use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use rand::{thread_rng, Rng};
use dns_rust::edns::opt_option::option_code::OptionCode;
use dns_rust::message::DnsMessage;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rcode::Rcode;
use dns_rust::message::rrtype::Rrtype;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs a client
    Client(ClientArgs),
    /// Runs a Stub Resolver
    Resolver(ResolverArgs),
}

/// Client Arguments
#[derive(Args, Debug)]
struct ClientArgs {
    /// Host name to query for IP
    #[arg(help = "The domain name to resolve.")]
    domain_name: String,

    /// DNS server ip
    #[arg(long)]
    server: String,

    /// Query type
    #[arg(long, default_value = "A")]
    qtype: String,

    /// Query class
    #[arg(long, default_value = "IN")]
    qclass: String,

    /// EDNS0 options in the format +option (e.g., +nsid, +ede, etc.)
    #[arg(trailing_var_arg = true, help_heading = "EDNS0 OPTIONS", help = "EDNS0 options available: +nsid +ede +padding +zoneversion")]
    options: Vec<String>,
}


/// Stub Resolver Arguments
#[derive(Args, Debug)]
struct ResolverArgs {
    /// Host name to query
    domain_name: String,
    /// Query type
    #[arg(long, default_value_t = String::from("A"))]
    qtype: String,
    /// Query class
    #[arg(long, default_value_t = String::from("IN"))]
    qclass: String,
    /// Protocol
    #[arg(long, default_value_t = String::from("UDP"))]
    protocol: String,
    /// Recursive Servers
    nameserver: Vec<IpAddr>,
}

/// Maps EDNS0 option strings to their corresponding `OptionCode`.
fn parse_edns_options(options: Vec<String>) -> Vec<OptionCode> {
    options
        .into_iter()
        .filter_map(|opt| {
            if opt.starts_with('+') {
                match opt.trim_start_matches('+').to_lowercase().as_str() {
                    "nsid" => Some(OptionCode::NSID),
                    "ede" => Some(OptionCode::EDE),
                    "padding" => Some(OptionCode::PADDING),
                    "zoneversion" => Some(OptionCode::ZONEVERSION),
                    _ => {
                        eprintln!("Unknown option: {}", opt);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect()
}
fn print_response(response: Result<Vec<ResourceRecord>, ClientError>) {
    match response {
        Ok(rrs) => println!("{:?}", rrs),
        Err(e) => println!("{:?}", e),
    }
}

#[tokio::main]
pub async fn main() {
    println!("Rustlang library for DNS");
    println!("Compatible with RFC 1034 and RFC 1035 only.");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Client(client_args) => {
            let addr = client_args.server.parse::<IpAddr>().expect("Invalid IP address");
            let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
            let mut client = Client::new(conn);

            let mut dns_query_message =
                DnsMessage::new_query_message(
                    DomainName::new_from_string(client_args.domain_name.clone()),
                    Rrtype::from(client_args.qtype.as_str()),
                    Rclass::from(client_args.qclass.as_str()),
                    0,
                    false,
                    thread_rng().gen());

            // Map EDNS0 options to OptionCodes
            let option_codes = parse_edns_options(client_args.options.clone());

            if !option_codes.is_empty() {
                dns_query_message.add_edns0(Some(512), Rcode::NOERROR, 0, false, Some(option_codes));
            }

            client.set_dns_query(dns_query_message);

            // Send the query and print the response
            let response = client.send_query().await;

            if let Ok(resp) = response {
                println!("{}", resp);
            }
        }

        Commands::Resolver(resolver_args) => {
            let mut config = ResolverConfig::os_config();

            let timeout = 2;
            if resolver_args.nameserver.len() > 0 {
                let mut nameservers = Vec::new();
                for ip_addr in resolver_args.nameserver.clone() {
                    let udp_conn = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(timeout));
                    let tcp_conn = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(timeout));
                    let server_info = ServerInfo::new_with_ip(ip_addr, udp_conn, tcp_conn);
                    nameservers.push(server_info);
                }
                config.set_name_servers(nameservers);
            }
            println!("Resolver pre loaded with nameservers: {:?}", config.get_name_servers().iter().map(|server| server.get_ip_addr()).collect::<Vec<IpAddr>>());
            let mut resolver = AsyncResolver::new(config);
            let response = resolver.lookup(
                resolver_args.domain_name.as_str(),
                resolver_args.protocol.as_str(),
                resolver_args.qtype.as_str(),
                resolver_args.qclass.as_str(),
            ).await;
            let rrs = response.map(|lookup_response| lookup_response.to_vec_of_rr());
            print_response(rrs);
        }
    }
}