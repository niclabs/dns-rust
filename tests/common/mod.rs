
use if_addrs::{get_if_addrs, Interface};
use std::sync::mpsc;
use dns_rust::{
//     config::RESOLVER_IP_PORT,
//     // config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
//     config::{ SBELT_ROOT_IPS},
//     // name_server::{master_file::MasterFile, zone::NSZone},
    resolver::{Resolver},
};

#[allow(dead_code)]
pub fn get_interface() -> Result<Interface,&'static str> {

    if let Ok(addrs) = get_if_addrs() {
        let default_interface = addrs
            .iter()
            .find(|&addr| !addr.is_loopback())
            .ok_or("No interface found")?;
        return Ok(default_interface.clone());
    }

    return Err("No interface found");
}

#[allow(dead_code)]
pub fn run_resolver_for_testing(resolver_ip_port: &str,sbelt_root_ips: [&str;3]) {
    // Channels
    let (add_sender_udp, add_recv_udp) = mpsc::channel();
    let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
    let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
    let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
    let (add_sender_ns_udp, _) = mpsc::channel();
    let (delete_sender_ns_udp, _) = mpsc::channel();
    let (add_sender_ns_tcp, _) = mpsc::channel();
    let (delete_sender_ns_tcp, _) = mpsc::channel();
    let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
    let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
    let (update_cache_sender_ns_udp, _) = mpsc::channel();
    let (update_cache_sender_ns_tcp, _) = mpsc::channel();

    let (_, rx_update_zone_udp) = mpsc::channel();
    let (_, rx_update_zone_tcp) = mpsc::channel();

    
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

        resolver.set_initial_configuration(resolver_ip_port, sbelt_root_ips);

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
   
}