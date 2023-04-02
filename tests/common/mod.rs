
use if_addrs::{get_if_addrs, Interface};
// use std::sync::mpsc;
// use dns_rust::{
//     config::RESOLVER_IP_PORT,
//     // config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
//     config::{ SBELT_ROOT_IPS},
//     // name_server::{master_file::MasterFile, zone::NSZone},
//     resolver::{Resolver},
// };



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


// pub fn run_resolver() {
//     // Channels
//     let add_udp_channel = mpsc::channel();
//     let delete_udp_channel = mpsc::channel();
//     let add_tcp_channel = mpsc::channel();
//     let delete_tcp_channel = mpsc::channel();
//     let add_ns_udp_channel = mpsc::channel();
//     let delete_ns_udp_channel = mpsc::channel();
//     let add_ns_tcp_channel = mpsc::channel();
//     let delete_ns_tcp_channel = mpsc::channel();
//     let update_cache_udp_channel = mpsc::channel();
//     let update_cache_tcp_channel = mpsc::channel();
//     let update_cache_ns_udp_channel = mpsc::channel();
//     let update_cache_ns_tcp_channel = mpsc::channel();

//     let update_zone_udp_channel = mpsc::channel();
//     let update_zone_tcp_channel = mpsc::channel();

//     //create obj resolver
//     let mut resolver = Resolver::new(
//         add_udp_channel.0.clone(),
//         delete_udp_channel.0.clone(),
//         add_tcp_channel.0.clone(),
//         delete_tcp_channel.0.clone(),
//         add_ns_udp_channel.0.clone(),
//         delete_ns_udp_channel.0.clone(),
//         add_ns_tcp_channel.0.clone(),
//         delete_ns_tcp_channel.0.clone(),
//         update_cache_udp_channel.0.clone(),
//         update_cache_tcp_channel.0.clone(),
//         update_cache_ns_udp_channel.0.clone(),
//         update_cache_ns_tcp_channel.0.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
//     // Run Resolver
//     resolver.run_resolver(
//         add_udp_channel.1,
//         delete_udp_channel.1,
//         add_tcp_channel.1,
//         delete_tcp_channel.1,
//         update_cache_udp_channel.1,
//         update_cache_tcp_channel.1,
//         update_zone_udp_channel.1,
//         update_zone_tcp_channel.1,
//     );


// }