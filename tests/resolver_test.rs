    
mod common;
extern crate pnet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{transport_channel,tcp_packet_iter, self};

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use dns_rust::{
    client::{create_client_query,
            send_client_query},
    message::DnsMessage,
};

//config for resolver
use dns_rust::config::{
    RESOLVER_IP_PORT, SBELT_ROOT_IPS,
};

use crate::common::run_resolver_for_testing;
use dns_rust::{self, client, domain_name};


//FIXME: use client not from our library

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

#[test]
#[ignore]
fn qtype_a_example(){
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));

    // create client query
    let client_query: DnsMessage = create_client_query("example.com",
                                    1,
                                    1);

    //send query and get response
    let dns_response = send_client_query(transport_protocol,
                                        RESOLVER_IP_PORT,
                                        client_query);
    // dns_response.print_dns_message();

    //test
    common::qtype_a_example(dns_response);

}


#[test]
#[ignore]
fn non_existent_type(){
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });

    thread::sleep(Duration::from_secs(1));
    
    // create client query
    let client_query: DnsMessage = create_client_query("example.com",
                                    13,
                                    1);
    
    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                        RESOLVER_IP_PORT,
                                        client_query);
    
    common::qtype_hinfo_example_no_answer(dns_response);
    
}

#[test]
fn invalid_domain(){

    let transport_protocol = "TCP";
    let domain_name = "exam¿ple.com";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });

    thread::sleep(Duration::from_secs(1));


    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
        13,
        1);
    println!("query creada");

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query);
    
                                    println!("query response");

    
    
    //Header
    let header = dns_response.get_header();
    let rcode = header.get_rcode(); 
    
    //Format Error
    assert_eq!(rcode, 1);
    

}

#[test]
#[ignore]
fn query_answer_in_cache(){
    //FIXME: fails resolver in update of cache

    //query values
    let transport_protocol = "TCP";
    let domain_name = "example.com";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    thread::sleep(Duration::from_secs(1));


    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
        13,
        1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query.clone());
    thread::sleep(Duration::from_secs(1));    
    let second_dns_response = send_client_query(transport_protocol,
                                        RESOLVER_IP_PORT,
                                        client_query);
    
    common::qtype_a_example(second_dns_response);

    //Header
    let header = dns_response.get_header();
    let aa = header.get_aa(); 
    
    //Format Error
    assert_eq!(aa, true);

}

#[test]
#[should_panic]
#[ignore]
fn qtype_asterisk_example(){
    //Not implemented type RRSIG and is in answer 
    //se van a caer porq desde nuestro cliente ya se cae
    //revisar whireshark

    //values query
    let domain_name_example = "example.com";
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    thread::sleep(Duration::from_secs(1));


    // create client query
    let client_query: DnsMessage = create_client_query(domain_name_example,
        13,
        1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query);
    
    common::qtype_asterisk_example(dns_response); 

}

#[test]
#[ignore]
fn qtype_asterisk_test(){
    //se van a caer porq desde nuestro cliente ya se cae

    //values query
    let domain_name_test = "test";
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    thread::sleep(Duration::from_secs(1));

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name_test,
                                        13,
                                        1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query);

    common::qtype_asterisk_test(dns_response); 

}

#[test]
#[ignore]
fn qtype_ns_example(){
    //FIXME: resolver fails

    //values query
    let domain_name_test = "example.com";
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    thread::sleep(Duration::from_secs(1));

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name_test,
                                        2,
                                        1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query);

    common::qtype_ns_example(dns_response); 
    
}


#[test]
#[ignore]
fn qtype_mx_example(){
    //FIXME: fais but becouse of our client, see a library for clreate client query

    //values query
    let domain_name_test = "example.com";
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    thread::sleep(Duration::from_secs(1));

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name_test,
                                        15,
                                        1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query);

    common::qtype_ns_example(dns_response); 

    
}


#[test]
fn qtype_soa_example(){

    //values query
    let domain_name_test = "example.com";
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    thread::sleep(Duration::from_secs(1));

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name_test,
                                        6,
                                        1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                    RESOLVER_IP_PORT,
                                    client_query);

    common::qtype_soa_example(dns_response); 

    
}



#[test]
#[ignore]
fn qtype_txt_example(){
    let transport_protocol = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));

    // create client query
    let client_query: DnsMessage = create_client_query("example.com",
                                    16,
                                    1);

    //send query and get response
    let dns_response = send_client_query(transport_protocol,
                                        RESOLVER_IP_PORT,
                                        client_query);
    // dns_response.print_dns_message();

    //test
    common::qtype_txt_example(dns_response);

    

}


#[test]
#[ignore]
fn query_udp_tcp_to_same_resolver(){

    //values query
    let domain_name = "example.com";
    let transport_protocol_udp  = "UDP";
    let transport_protocol_tcp  = "TCP";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });

    // create client query udp
    let client_query_udp: DnsMessage = create_client_query(domain_name,
                                        2,
                                        1);

    // send query and get response udp
    let dns_response_udp = send_client_query(transport_protocol_udp,
                                        RESOLVER_IP_PORT,
                                        client_query_udp);

    // create client query tcp
    let client_query_tcp: DnsMessage = create_client_query(domain_name,
                                        2,
                                        1);

    // send query and get response tcp
    let dns_response_tcp = send_client_query(transport_protocol_tcp,
                                        RESOLVER_IP_PORT,
                                        client_query_tcp);

    common::qtype_a_example(dns_response_udp);
    common::qtype_a_example(dns_response_tcp);

}





#[test]
#[ignore]
fn get_resolver_packets_tcp(){
    //must be run with sudo privileges

    //Run Resolver
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS)
    });

    //config for catching packets
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Tcp));

    //create transport channel
    let (_,mut rx) = match transport_channel(4096, protocol) {
        Ok((tx,rx)) => (tx,rx),
        Err(e) => panic!("Error: creating the transport channel: {}",e),
    };

    let mut iter = tcp_packet_iter(&mut rx);

    //channel to stop test
    let (tx_stop,rx_stop) = channel();

    //Run Client
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        client::run_client();
        tx_stop.send(()).unwrap();        
    });
    
    //Loop for catchinng packets
    loop {
        match iter.next() {
            Ok((packet, _)) => {
                
                //
                let source = packet.get_source(); //caso sale del resolver
                let destination = packet.get_destination(); //caso llega respuesta al resolver
                let payload = packet.payload();
                

                match (source,destination){
                    (_,58396) => {
                        println!("\n DNS: Response to Resolver------------------------------------------------------------");
                        println!("payload: {:?}",payload);

                        },
                    (53,_) => {
                        println!("\n DNS: Sent Query by Resolver-----------------------------------------------------------");
                        println!("payload: {:?}",payload);
                        
                    },
                    
                    _  =>  {println!("\n Other TCP message------------------------------------------------------------------");
                }
                }



            }
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
        //finish loop if client is finish
        match rx_stop.try_recv() {
            Ok(_) => break,
            Err(_) => {},   
        }
    }

}






