use std::{time::Duration, net::IpAddr};
use std::io::Error as IoError;
use std::io::ErrorKind;
use dns_rust::{
    async_resolver::{
            config::ResolverConfig, AsyncResolver, server_info::ServerInfo
        }, client::{
        client_connection::ClientConnection, client_error::ClientError, tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection, Client}, domain_name::DomainName};

use clap::*;
use rand::{thread_rng, Rng};
use dns_rust::async_resolver::lookup_response::LookupResponse;
use dns_rust::client::client_connection::ConnectionProtocol;
use dns_rust::client::client_security::ClientSecurity;
use dns_rust::client::tls_connection::ClientTLSConnection;
use dns_rust::edns::opt_option::option_code::OptionCode;
use dns_rust::message::DnsMessage;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rcode::Rcode;
use dns_rust::message::rrtype::Rrtype;
use dns_rust::tsig::tsig_algorithm::TsigAlgorithm;

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
#[command(after_help = "\x1b[1m\x1b[4mEdns0 options:\x1b[0m\n  \
                        +nsid            NSID option code\n  \
                        +padding         PADDING option code \n  \
                        +ede             EDE option code  \n  \
                        +zoneversion     ZONEVERSION option code\n\
                        \x1b[1m\x1b[4mExamples:\x1b[0m\n  \
                        dns_rust client --qtype A --qclass IN 1.1.1.1 example.com +nsid +zoneversion")]
struct ClientArgs {
    /// DNS server ip
    server: String,

    /// Host name to query for IP
    domain_name: String,

    /// Query type
    #[arg(long, default_value = "A")]
    qtype: String,

    /// Query class
    #[arg(long, default_value = "IN")]
    qclass: String,

    /// Disables the use of recursion when specified
    #[arg(long, default_value = "false")]
    norecursive: bool,

    /// EDNS0 options in the format +option (e.g., +nsid, +ede, etc.)
    #[arg(trailing_var_arg = true, help = "EDNS0 options")]
    options: Vec<String>,

    /// Maximum payload for EDNS
    #[arg(long, default_value = "512")]
    payload: u16,

    /// Disables the use of EDNS when specified
    #[arg(long, default_value = "false")]
    noedns: bool,

    /// Transport protocol, options: "UDP", "TCP", "TLS".
    #[arg(long, default_value_t = String::from("UDP"))]
    protocol: String,

    /// TSIG arguments key, algorithm, fudge, time_signed, key_name, mac_request
    #[arg(long, value_parser = TsigArgs::from_str)]
    tsig: Option<TsigArgs>,
}

/// Represents the arguments required for TSIG.
#[derive(Debug, Clone)]
pub struct TsigArgs {
    pub key: Vec<u8>,
    pub alg_name: TsigAlgorithm,
    pub fudge: u16,
    pub time_signed: u64,
    pub key_name: String,
    pub mac_request: Vec<u8>,
}

impl TsigArgs {
    /// Parses a string into a `TsigArgs` instance.
    pub fn from_str(value: &str) -> Result<Self, String> {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() != 6 {
            return Err("Expected 6 values for TSIG args".to_string());
        }

        let key = hex::decode(parts[0].trim()).map_err(|e| e.to_string())?;
        let alg_name = TsigAlgorithm::from(parts[1].trim().to_string());
        let fudge = parts[2].trim().parse::<u16>().map_err(|e| e.to_string())?;
        let time_signed = parts[3].trim().parse::<u64>().map_err(|e| e.to_string())?;
        let key_name = parts[4].trim().to_string();
        let mac_request = hex::decode(parts[5].trim()).map_err(|e| e.to_string())?;

        Ok(Self {
            key,
            alg_name,
            fudge,
            time_signed,
            key_name,
            mac_request,
        })
    }
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

fn print_response_from_lookup(response: Result<LookupResponse, ClientError>) {
    match response {
        Ok(rrs) => {
            let bytes = rrs.get_bytes();
            let message = DnsMessage::from_bytes(bytes.as_slice());
            match message {
                Ok(mess) => println!("{}", mess),
                Err(e) => println!("{}", e),
            }
        },
        Err(e) => println!("{}", e),
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

            let mut dns_query_message =
                DnsMessage::new_query_message(
                    DomainName::new_from_string(client_args.domain_name.clone()),
                    Rrtype::from(client_args.qtype.as_str()),
                    Rclass::from(client_args.qclass.as_str()),
                    0,
                    !client_args.norecursive,
                    thread_rng().gen());

            // edns related
            if !client_args.noedns {
                let option_codes = parse_edns_options(client_args.options.clone());
                let mut some_options = None;
                if !option_codes.is_empty() { some_options = Some(option_codes); }
                let max_payload = Some(client_args.payload);
                dns_query_message.add_edns0(max_payload, Rcode::NOERROR, 0, false, some_options);
            }

            if !client_args.tsig.is_none() {
                if let Some(tsig_args) = &client_args.tsig {
                    dns_query_message.sign_message(&*tsig_args.key,
                                                   tsig_args.alg_name.clone(),
                                                   tsig_args.fudge,
                                                   tsig_args.time_signed,
                                                   tsig_args.key_name.clone(),
                                                   tsig_args.mac_request.clone());
                }
            }

            // match tcp to set a client
            let response = match client_args.protocol.as_str() {
                "UDP" => {
                    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
                    let mut client = Client::new(conn);
                    client.set_dns_query(dns_query_message);
                    client.send_query().await
                }
                "TCP" => {
                    let conn = ClientTCPConnection::new_default(addr, Duration::from_secs(10));
                    let mut client = Client::new(conn);
                    client.set_dns_query(dns_query_message);
                    client.send_query().await
                },
                "TLS" => {
                    let conn = ClientTLSConnection::new_default(addr, Duration::from_secs(10));
                    let mut client = Client::new(conn);
                    client.set_dns_query(dns_query_message);
                    client.send_query().await
                },
                _ => {
                    eprintln!{"{} is not a supported protocol", client_args.protocol.as_str()};
                    Err(ClientError::Io(IoError::new(
                    ErrorKind::InvalidInput,
                    format!("{} is not a supported protocol", client_args.protocol.as_str()),
                )).into())}
            };

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

            print_response_from_lookup(response);
        }
    }
}