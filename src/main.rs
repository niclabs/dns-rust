use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use dns_rust::name_server::NameServer;
use dns_rust::name_server::zone::NSZone;
use dns_rust::resolver::Resolver;
use dns_rust::resolver::slist::Slist;
use dns_rust::{self, client, resolver};
use dns_rust::client::config::{HOST_NAME, TRANSPORT};
use dns_rust::config::{MASTER_FILES, RESOLVER_IP_PORT, SBELT_ROOT_IPS, CHECK_MASTER_FILES, NAME_SERVER_IP};
use dns_rust::name_server::master_file::MasterFile;

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
        for (master_file_name, master_file_origin) in MASTER_FILES {
            let _validated_mf = MasterFile::from_file(master_file_name.to_string(), master_file_origin.to_string(),true);
        }
        println!("All Master Files validated successfully.");
    }

    else if trim_input_line == "C" {
        client::run_client(HOST_NAME, TRANSPORT);
    } else {
        // Channels
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();
        let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
        let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (update_cache_sender_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (update_cache_sender_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let (update_zone_udp, rx_update_zone_udp) = mpsc::channel();
        let (update_zone_tcp, rx_update_zone_tcp) = mpsc::channel();

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

            for (master_file_name, master_file_origin) in MASTER_FILES {
                name_server.add_zone_from_master_file(master_file_name.to_string(), master_file_origin.to_string(),"".to_string(), CHECK_MASTER_FILES);
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

            // Resolver Initialize
            let mut resolver = Resolver::new(
                add_sender_udp.clone(),
                delete_sender_udp.clone(),
                add_sender_tcp.clone(),
                delete_sender_tcp.clone(),
                add_sender_ns_udp.clone(),
                delete_sender_ns_udp.clone(),
                add_sender_ns_tcp.clone(),
                delete_sender_ns_tcp.clone(),
                update_cache_sender_udp.clone(),
                update_cache_sender_tcp.clone(),
                update_cache_sender_ns_udp.clone(),
                update_cache_sender_ns_tcp.clone(),
            );

            resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

            let mut sbelt = Slist::new();

            for ip in SBELT_ROOT_IPS {
                sbelt.insert(".".to_string(), ip.to_string(), 5000);
            }

            resolver.set_sbelt(sbelt);
            //

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

            for (master_file_name, master_file_origin) in MASTER_FILES {
                name_server.add_zone_from_master_file(master_file_name.to_string(), master_file_origin.to_string(),"".to_string(), CHECK_MASTER_FILES);
            }
            //

            // Set zones to resolver
            resolver.set_ns_data(name_server.get_zones());

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
            //

            // Run Resolver
            resolver.run_resolver(
                add_recv_udp,
                delete_recv_udp,
                add_recv_tcp,
                delete_recv_tcp,
                rx_update_cache_udp,
                rx_update_cache_tcp,
                rx_update_zone_udp,
                rx_update_zone_tcp,
            );
            //
        }
    }
}