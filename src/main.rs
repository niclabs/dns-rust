use std::{time::Duration, net::IpAddr};

use dns_rust::{client::{Client, tcp_connection::ClientTCPConnection, client_connection::ClientConnection}, domain_name::DomainName};
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

pub fn main() {
    println!("Rustlang library for DNS");
    println!("Compatible with RFC 1034 and RFC 1035 only.");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Client(client_args) => {

            let addr = client_args.server.parse::<IpAddr>();
            let conn = ClientTCPConnection::new(addr.unwrap(), Duration::from_secs(10));
            let mut client = Client::new(conn);

            let mut response = client.query(DomainName::new_from_string(client_args.host_name.clone()), client_args.qtype.as_str(), client_args.qclass.as_str());

            response.print_dns_message()
        }
    }  
}
