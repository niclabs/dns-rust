use std::sync::mpsc;

use dns_rust::config::{
    RESOLVER_IP_PORT, SBELT_ROOT_IPS,
};
use dns_rust::resolver::Resolver;
use dns_rust::{self, client};

pub fn main() {
    // Users input
    let mut input_line = String::new();
    println!("Rustlang library for DNS");
    println!("Name server compatible with RFC 1034 and RFC 1035 only.");
    println!("To only check the validity of a Master file, enter MF.");
    println!("For other services, enter program to run: \n[C] Client\n[R] Resolver\n[N] Nameserver\n[NR] Nameserver and Resolver");
    std::io::stdin().read_line(&mut input_line).unwrap();

    let trim_input_line = input_line.trim();

    if trim_input_line == "C" {
        client::run_client();
    } else {
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

        if trim_input_line == "R" {
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
    }
}
