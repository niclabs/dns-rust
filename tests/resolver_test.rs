// use dns_rust::client::config::CLIENT_IP_PORT;
// use dns_rust::client::create_client_query;
// use dns_rust::message::question;
// use dns_rust::message::rdata::Rdata;
// use dns_rust::name_server::NameServer;
// use dns_rust::resolver::slist::Slist;
// use dns_rust::resolver::Resolver;
// use dns_rust::{
//     client,
//     config::RESOLVER_IP_PORT,
//     config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
//     name_server::{master_file::MasterFile, zone::NSZone},
//     resolver,
// };
// use std::net::UdpSocket;
// use std::sync::mpsc;
//use std::{collections::HashMap, fs, thread, time};
// use dns_rust::message::DnsMessage;
// use std::vec::Vec;



// Gets a Vec of host names from a external file
// fn get_host_names_from_zone_file(path: &str) -> Vec<String> {
//     // Read file content
//     let contents = fs::read_to_string(path).expect("Should have been able to read the file");

//     // Split file content
//     let splitted_content: Vec<&str> = contents.split("\n").collect();

//     // Create a vec of host names
//     let mut host_names_vec: Vec<String> = Vec::new();
//     // Extract host names from file
//     for host_name in splitted_content {
//         host_names_vec.push(host_name.to_string())
//     }

//     // Return all host names from file
//     return host_names_vec;
// }

// #[test]
// fn validate_rfc_master_files() {
//     for (master_file, master_file_origin) in MASTER_FILES {
//         let _validated_mf = MasterFile::from_file(
//             master_file.to_string(),
//             master_file_origin.to_string(),
//             true,
//         );
//     }
// }

// /// Robustness test
// #[test]
// fn test_500000_cl_domains() {
//     //let (add_sender_udp, add_recv_udp) = mpsc::channel();
//     owo!(add_sender_udp,add_recv_udp);
//     let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
//     let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
//     let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
//     let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
//     let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
//     let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
//     let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();
//     let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
//     let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
//     let (update_cache_sender_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
//     let (update_cache_sender_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();
//     let (update_zone_udp, rx_update_zone_udp) = mpsc::channel();
//     let (update_zone_tcp, rx_update_zone_tcp) = mpsc::channel();

//     // Run resolver.
//     thread::spawn(move || {
//         // Resolver Initialize
//         let mut resolver = Resolver::new(
//             add_sender_udp.clone(),
//             delete_sender_udp.clone(),
//             add_sender_tcp.clone(),
//             delete_sender_tcp.clone(),
//             add_sender_ns_udp.clone(),
//             delete_sender_ns_udp.clone(),
//             add_sender_ns_tcp.clone(),
//             delete_sender_ns_tcp.clone(),
//             update_cache_sender_udp.clone(),
//             update_cache_sender_tcp.clone(),
//             update_cache_sender_ns_udp.clone(),
//             update_cache_sender_ns_tcp.clone(),
//         );

//         resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

//         // Run Resolver
//         resolver.run_resolver(
//             add_recv_udp,
//             delete_recv_udp,
//             add_recv_tcp,
//             delete_recv_tcp,
//             rx_update_cache_udp,
//             rx_update_cache_tcp,
//             rx_update_zone_udp,
//             rx_update_zone_tcp,
//         );
//     });

//     // Get all host names from a file
//     let host_names_vec: Vec<String> =
//         get_host_names_from_zone_file("tests/test_files/test_domains_names.txt");
//     for host_name in host_names_vec {
//         println!("Domain name: {}", host_name);
//         let mut dnsmessage = client::create_client_query(host_name.as_str(), "TCP", 1, 1);
//         dnsmessage.print_dns_message()
//     }
// }

