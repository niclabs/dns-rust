use std::{time::Duration, net::IpAddr};

use dns_rust::{
    async_resolver::{
            config::ResolverConfig, AsyncResolver, server_info::ServerInfo
        }, client::{
        client_connection::ClientConnection, client_error::ClientError, tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection, Client}, domain_name::DomainName, message::resource_record::ResourceRecord};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
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
    domain_name: String,
    /// DNS server ip
    #[arg(long)]
    server: String,
    /// Query type
    #[arg(long, default_value_t = String::from("A"))]
    qtype: String,
    /// Query class
    #[arg(long, default_value_t = String::from("IN"))]
    qclass: String,
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

            let addr = client_args.server.parse::<IpAddr>();
            let conn = ClientTCPConnection::new_default(addr.unwrap(), Duration::from_secs(10));
            let mut client = Client::new(conn);

            let response = client.query(
                DomainName::new_from_string(client_args.domain_name.clone()), 
                client_args.qtype.as_str(), 
                client_args.qclass.as_str()
            );

            if let Ok(resp) = response.await {
                println!("{}", resp);
            }
        }

        Commands::Resolver(resolver_args) => {
            let mut config = ResolverConfig::default();

            let timeout = 2;
            for ip_addr in resolver_args.nameserver.clone() {
                let udp_conn = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(timeout));
                let tcp_conn = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(timeout));
                let server_info = ServerInfo::new_with_ip(ip_addr, udp_conn, tcp_conn);
                config.add_name_server(server_info);
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
