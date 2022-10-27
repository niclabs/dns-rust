pub mod client;
pub mod config;
pub mod dns_cache;
pub mod domain_name;
pub mod global_tests;
pub mod message;
pub mod name_server;
pub mod resolver;
pub mod rr_cache;
pub mod utils;

use crate::name_server::NameServer;
use crate::resolver::slist::Slist;
use crate::resolver::Resolver;
use crate::name_server::master_file::MasterFile;
use crate::name_server::zone::NSZone;


use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

use crate::config::MASTER_FILES;
use crate::config::NAME_SERVER_IP;
use crate::config::RESOLVER_IP_PORT;
use crate::config::SBELT_ROOT_IPS;
use crate::config::CHECK_MASTER_FILES;

pub fn main() {
    // Users input
    let mut input_line = String::new();
    println!("Rustlang library for DNS");
    println!("Name server compatible with RFC 1034 and RFC 1035 only.");
    println!("To only check the validity of a Master file, enter MF.");
    println!("For other services, enter program to run: \n[C] Client\n[R] Resolver\n[N] Nameserver\n[NR] Nameserver and Resolver");
    std::io::stdin().read_line(&mut input_line).unwrap();

    let trim_input_line = input_line.trim();

    if trim_input_line == "MF" {
        for master_file in MASTER_FILES {
            let _validated_mf = MasterFile::from_file(master_file.to_string(), true);
        }
        println!("All Master Files validated successfully.");
    }

    else if trim_input_line == "C" {
        client::run_client();
    } else {
        // Channels
        let (delete_sender_udp, _) = mpsc::channel();
        let (delete_sender_tcp, _) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();
        let (update_cache_sender_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (update_cache_sender_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let (update_zone_udp, _) = mpsc::channel();
        let (update_zone_tcp, _) = mpsc::channel();

        if trim_input_line == "R" {
            resolver::run_resolver(RESOLVER_IP_PORT, SBELT_ROOT_IPS, HashMap::<u16, HashMap<String, NSZone>>::new());
        } 
        else if trim_input_line == "N" {
            let (update_refresh_zone_udp, rx_update_refresh_zone_udp) = mpsc::channel();
            let (update_refresh_zone_tcp, rx_update_refresh_zone_tcp) = mpsc::channel();

            let mut name_server = NameServer::new(
                false,
                delete_sender_udp.clone(),
                delete_sender_tcp.clone(),
                add_sender_ns_udp.clone(),
                delete_sender_ns_udp.clone(),
                add_sender_ns_tcp.clone(),
                delete_sender_ns_tcp.clone(),
                update_refresh_zone_udp.clone(),
                update_refresh_zone_tcp.clone(),
                update_zone_udp.clone(),
                update_zone_tcp.clone(),
            );

            for master_file in MASTER_FILES {
                name_server.add_zone_from_master_file(master_file.to_string(), "".to_string(), CHECK_MASTER_FILES);
            }

            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                RESOLVER_IP_PORT.to_string(),
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
                rx_update_cache_ns_udp,
                rx_update_cache_ns_tcp,
                rx_update_refresh_zone_udp,
                rx_update_refresh_zone_tcp,
            );
        } else if trim_input_line == "NR" {
            // Name Server initialize
            let (update_refresh_zone_udp, rx_update_refresh_zone_udp) = mpsc::channel();
            let (update_refresh_zone_tcp, rx_update_refresh_zone_tcp) = mpsc::channel();

            let mut name_server = NameServer::new(
                false,
                delete_sender_udp.clone(),
                delete_sender_tcp.clone(),
                add_sender_ns_udp.clone(),
                delete_sender_ns_udp.clone(),
                add_sender_ns_tcp.clone(),
                delete_sender_ns_tcp.clone(),
                update_refresh_zone_udp,
                update_refresh_zone_tcp,
                update_zone_udp,
                update_zone_tcp,
            );

            for master_file in MASTER_FILES {
                name_server.add_zone_from_master_file(master_file.to_string(), "".to_string(), CHECK_MASTER_FILES);
            }

            let ns_data = name_server.get_zones();

            // Run Name server
            thread::spawn(move || {
                name_server.run_name_server(
                    NAME_SERVER_IP.to_string(),
                    RESOLVER_IP_PORT.to_string(),
                    add_recv_ns_udp,
                    delete_recv_ns_udp,
                    add_recv_ns_tcp,
                    delete_recv_ns_tcp,
                    rx_update_cache_ns_udp,
                    rx_update_cache_ns_tcp,
                    rx_update_refresh_zone_udp,
                    rx_update_refresh_zone_tcp,
                );
            });
            // Run Resolver
            resolver::run_resolver(RESOLVER_IP_PORT, SBELT_ROOT_IPS, ns_data);
            //
        }
    }
}
