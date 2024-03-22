use std::{time::Duration, net::IpAddr};

use dns_rust::{
    client::{
        Client, 
        tcp_connection::ClientTCPConnection, 
        client_connection::ClientConnection, 
        udp_connection::ClientUDPConnection}, 
        domain_name::DomainName, 
        async_resolver::{
            config::ResolverConfig, 
            AsyncResolver, 
            resolver_error::ResolverError
        }, message::resource_record::ResourceRecord};
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
    host_name: String,
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
    host_name: String,
    /// Resolver bind address
    #[arg(long)]
    bind_addr: Option<IpAddr>,
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


async fn query(
    mut resolver: AsyncResolver,
    domain_name: String,
    qtype: String,
    qclass: String,
    protocol: String,
) -> Result<Vec<ResourceRecord>, ResolverError> {
    let response = resolver.lookup(domain_name.as_str(),protocol.as_str(), qtype.as_str(),qclass.as_str()).await;

    response.map(|lookup_response| lookup_response.to_vec_of_rr())
}

fn print_response(response: Result<Vec<ResourceRecord>, ResolverError>) {
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
            let conn = ClientTCPConnection::new(addr.unwrap(), Duration::from_secs(10));
            let mut client = Client::new(conn);

            let response = client.query(
                DomainName::new_from_string(client_args.host_name.clone()), 
                client_args.qtype.as_str(), 
                client_args.qclass.as_str()
            );

            if let Ok(mut resp) = response.await {
                resp.print_dns_message()
            }
        }

        Commands::Resolver(resolver_args) => {
            let mut config = ResolverConfig::default();

            let mut nameservers: Vec<(ClientUDPConnection, ClientTCPConnection)> = Vec::new();
            let timeout = 2;
            for ip_addr in resolver_args.nameserver.clone() {
                let udp_conn = ClientUDPConnection::new(ip_addr, Duration::from_secs(timeout));
                let tcp_conn = ClientTCPConnection::new(ip_addr, Duration::from_secs(timeout));
                nameservers.push((udp_conn, tcp_conn));

            }
            config.set_name_servers(nameservers);

            let resolver = AsyncResolver::new(config);
            let response = query(
                resolver,
                 resolver_args.host_name.clone(),
                  resolver_args.qtype.clone(),
                  resolver_args.qclass.clone(),
                   resolver_args.protocol.clone()).await;
            
            print_response(response);
        }
    }  
}
