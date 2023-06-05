use std::net::Ipv4Addr;
use std::sync::mpsc;

use dns_rust::client;
use dns_rust::config::{
    SBELT_ROOT_IPS,
};
use dns_rust::resolver::Resolver;
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
    /// Runs a resolver
    Resolver(ResolverArgs),
}

/// Client Arguments
#[derive(Args, Debug)]
struct ClientArgs {
    /// Host name to query for IP
    #[arg(long)]
    host_name: String,
    /// Query type
    #[arg(long, default_value_t = 1)]
    qtype: u16,
    /// Query class
    #[arg(long, default_value_t = 1)]
    qclass: u16,
    /// Network Protocol to use
    #[arg(long, default_value_t = String::from("TCP"))]
    protocol: String
}

/// Resolver Arguments
#[derive(Args, Debug)]
struct ResolverArgs {
    /// Resolver Ip
    #[arg(long, default_value_t = Ipv4Addr::LOCALHOST)]
    ip: Ipv4Addr,
    /// Resolver Port
    #[arg(short, long, default_value_t = 58396)]
    port: u16,
}

pub fn main() {
    println!("Rustlang library for DNS");
    println!("Compatible with RFC 1034 and RFC 1035 only.");
    let cli = Cli::parse();


    match &cli.command {
        Commands::Client(client_args) => {
            client::run_client(&client_args.host_name);
        }
        Commands::Resolver(resolver_args) => {

            let ip_port: String = resolver_args.ip.to_string() + &":".to_string() + &resolver_args.port.to_string();

            // Channels
            let (add_sender_udp, add_recv_udp) = mpsc::channel();
            let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
            let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
            let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
            // let (add_sender_ns_udp, _) = mpsc::channel();
            // let (delete_sender_ns_udp, _) = mpsc::channel();
            // let (add_sender_ns_tcp, _) = mpsc::channel();
            // let (delete_sender_ns_tcp, _) = mpsc::channel();
            let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
            let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
            // let (update_cache_sender_ns_udp, _) = mpsc::channel();
            // let (update_cache_sender_ns_tcp, _) = mpsc::channel();

            // let (update_zone_udp, rx_update_zone_udp) = mpsc::channel();
            // let (update_zone_tcp, rx_update_zone_tcp) = mpsc::channel();

            // Resolver Initialize
            let mut resolver = Resolver::new(
                add_sender_udp.clone(),
                delete_sender_udp.clone(),
                add_sender_tcp.clone(),
                delete_sender_tcp.clone(),
                // add_sender_ns_udp.clone(),
                // delete_sender_ns_udp.clone(),
                // add_sender_ns_tcp.clone(),
                // delete_sender_ns_tcp.clone(),
                update_cache_sender_udp.clone(),
                update_cache_sender_tcp.clone(),
                // update_cache_sender_ns_udp.clone(),
                // update_cache_sender_ns_tcp.clone(),
            );

            resolver.set_initial_configuration(ip_port.as_str(), SBELT_ROOT_IPS);

            // Run Resolver
            resolver.run_resolver(
                add_recv_udp,
                delete_recv_udp,
                add_recv_tcp,
                delete_recv_tcp,
                rx_update_cache_udp,
                rx_update_cache_tcp,
                // rx_update_zone_udp,
                // rx_update_zone_tcp,
            );
        }
    }  
}
