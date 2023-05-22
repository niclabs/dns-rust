use std::sync::mpsc;
use std::env;

use dns_rust::config::{
    RESOLVER_IP_PORT, SBELT_ROOT_IPS,
};
use dns_rust::resolver::Resolver;
use dns_rust::{self, client};

pub fn main() {
    println!("Rustlang library for DNS");
    println!("Compatible with RFC 1034 and RFC 1035 only.");

    let args: Vec<String> = env::args().collect();

    let index = 1;
    match args[index].as_str() {
        "-c" => {
            client::run_client();
        }
        "-r" => {
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

            resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

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
        _ => {
            eprintln!("Error: '{}' command not found", args[index]);
            return;
        }
    }  
}
