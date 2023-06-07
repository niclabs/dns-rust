    
mod common;
// use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::net::{UdpSocket, TcpStream, SocketAddr};
use std::io::{Read, Write};
use hex;
use std::str::FromStr;



use dns_rust::{
    client::{config::TIMEOUT},
    // resolver
};

//config for resolver
use dns_rust::config::{
    RESOLVER_IP_PORT, SBELT_ROOT_IPS
};

use crate::common::run_resolver_for_testing;
use dns_rust::{self,};


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
fn qtype_a_example(){

    let string_hex_query = "861101200001000000000001076578616d706c6503636f6d0000010001000029100000000000000c000a000841d49cc746f76992".to_string();

    // run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));

    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_a_example_bytes(dns_response_udp);
    // common::qtype_a_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_a_example_bytes(dns_response_udp);
    common::qtype_a_example_bytes(dns_response_tcp); 
}

#[test]
fn qtype_ns_example(){

    let string_hex_query = "360f01200001000000000001076578616d706c6503636f6d0000020001000029100000000000000c000a000839f2559f0a6070a7".to_string();

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));

    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_ns_example_bytes(dns_response_udp);
    // common::qtype_ns_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_ns_example_bytes(dns_response_udp);
    common::qtype_ns_example_bytes(dns_response_tcp); 
}

#[test]
fn qtype_soa_example(){

    let string_hex_query = "861101200001000000000001076578616d706c6503636f6d0000060001000029100000000000000c000a0008970b6afc9f3385d2".to_string();

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));
  
    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_soa_example_bytes(dns_response_udp);
    // common::qtype_soa_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    //let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_soa_example_bytes(dns_response_udp);
    common::qtype_soa_example_bytes(dns_response_tcp); 
}

#[ignore]
#[test]
fn qtype_ptr(){
    //FIXME:
    let string_hex_query = "037801200001000000000001013801380138013807696e2d61646472046172706100000c000100002904d000000000000c000a00084c9879796ef71bec".to_string();

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));
  
    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_ptr_bytes(dns_response_udp);
    // common::qtype_ptr_bytes(dns_response_tcp); 

    //OUR RESOLVER
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    //let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_ptr_bytes(dns_response_udp);
    common::qtype_ptr_bytes(dns_response_tcp); 

}


#[ignore]
#[test]
fn qtype_hinfo_example(){

    // let string_hex_query = "b5bb01200001000000000001076578616d706c6503636f6d00000d0001000029100000000000000c000a00082ad20ef6d3683682".to_string();        
    let string_hex_query = "a8eb01200001000000000001076578616d706c6503636f6d00000d0001000029100000000000000c000a00084216f8e4db92ceea".to_string();

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));
  
    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_hinfo_example_bytes(dns_response_udp);
    // common::qtype_hinfo_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    //let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_hinfo_example_bytes(dns_response_udp);
    common::qtype_hinfo_example_bytes(dns_response_tcp); 

}

#[test]
#[ignore]
fn qtype_mx_example(){
    let string_hex_query = "ff6e01200001000000000001076578616d706c6503636f6d00000f0001000029100000000000000c000a00084ff21bafb4566efd".to_string();
    // let resolver = "8.8.8.8:53";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    });
    thread::sleep(Duration::from_secs(1));

    //GOOGLE RESOLVER
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_mx_example_bytes(dns_response_udp);
    // common::qtype_mx_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_mx_example_bytes(dns_response_udp);
    common::qtype_mx_example_bytes(dns_response_tcp); 
}

#[ignore]
#[test]
fn qtype_txt_example(){
    //FIXME: !!!!!!!!!!!!!!!!!!!!

    let string_hex_query = "861101200001000000000001076578616d706c6503636f6d0000100001000029100000000000000c000a000841d49cc746f76992".to_string();

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));  
    
    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // common::qtype_txt_example_bytes(dns_response_udp);
    // common::qtype_txt_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    //let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_txt_example_bytes(dns_response_udp);
    common::qtype_txt_example_bytes(dns_response_tcp); 
}

#[ignore]
#[test]
fn qtype_cname(){
    //FIXME: 

    let string_hex_query = "0cee01200001000000000001046d61696c057961686f6f03636f6d0000050001000029100000000000000c000a000803346ab484433bc3".to_string();
    let resolver = "8.8.8.8:53";

    //run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));  
    
    //GOOGLE RESOLVER
    let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    common::qtype_cname_bytes(dns_response_udp);
    common::qtype_cname_bytes(dns_response_tcp); 

    //OUR RESOLVER
    // let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query,RESOLVER_IP_PORT);
    //let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_cname_bytes(dns_response_udp);
    // common::qtype_cname_bytes(dns_response_tcp); 

}


#[ignore]
#[test]
fn answer_in_cache(){
    //FIXME: NO ESTA PONIENDO BIT AA cuando debería estar en cache

    let string_hex_query = "861101200001000000000001076578616d706c6503636f6d0000010001000029100000000000000c000a000841d49cc746f76992".to_string();

    // run resolver 
    thread::spawn(move || {
        run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);

    });
    thread::sleep(Duration::from_secs(1));

    //GOOGLE RESOLVER
    // let resolver = "8.8.8.8:53";
    // let _ = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let second_dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    // let _ = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    // let second_dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);

    // common::qtype_a_example_bytes_cache(second_dns_response_udp);
    // common::qtype_a_example_bytes_cache(second_dns_response_tcp); 

    //OUR RESOLVER
    // let _ = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // let dns_response_udp_cache = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    let _: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),RESOLVER_IP_PORT);
    let dns_response_tcp_cache: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),RESOLVER_IP_PORT);

    common::cache_answer(dns_response_tcp_cache);
    // common::cache_answer(dns_response_udp_cache);
}

#[test]
fn nonexistentdomain(){

        let string_hex_query = "eb7801200001000000000001116e6f6e6578697374656e74646f6d61696e0000010001000029100000000000000c000a0008f76b9ff5fb2cba0a".to_string();
    
        // run resolver 
        thread::spawn(move || {
            run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    
        });
        thread::sleep(Duration::from_secs(1));
    
        //GOOGLE RESOLVER
        // let resolver = "8.8.8.8:53";
        // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
        // let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);
    
        // common::nonexistentdomain_bytes(dns_response_udp);
        // common::nonexistentdomain_bytes(dns_response_tcp); 
    
        //OUR RESOLVER
        // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
        let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),RESOLVER_IP_PORT);
        // common::nonexistentdomain_bytes(dns_response_udp);
        common::nonexistentdomain_bytes(dns_response_tcp); 
}

#[test]
#[ignore]
fn qtype_any_example(){
    //FIXME:

    let string_hex_query ="003530880120000100000000000108657878616d706c6503636f6d0000ff0001000029100000000000000c000a0008011eb8ed12565cdf".to_string();
    let resolver = "8.8.8.8:53";
    //run resolver 
    // thread::spawn(move || {
    //     run_resolver_for_testing(RESOLVER_IP_PORT,SBELT_ROOT_IPS);
    // });

    // thread::sleep(Duration::from_secs(1));

    //GOOGLE RESOLVER
    let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),resolver);
    let dns_response_tcp = send_get_message_from_resolver_tcp(string_hex_query.clone(),resolver);

    common::qtype_any_example_bytes(dns_response_udp);
    common::qtype_any_example_bytes(dns_response_tcp); 

    //OUR RESOLVER
    // let dns_response_udp = send_get_message_from_resolver_udp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // let dns_response_tcp: Vec<u8> = send_get_message_from_resolver_tcp(string_hex_query.clone(),RESOLVER_IP_PORT);
    // common::qtype_any_example_bytes(dns_response_udp);
    // common::qtype_any_example_bytes(dns_response_tcp); 
}



///Sends DNS query by UDP to address of resolver given and reruns the response
fn send_get_message_from_resolver_udp(hex_string: String,resolver_addr:&str) -> Vec<u8> {
    let bytes = hex::decode(hex_string).unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("No connection");
    socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    let server_addr = SocketAddr::from_str(resolver_addr).expect("Invalid address");
    socket
        .send_to(&bytes, server_addr)
        .unwrap_or_else(|e| panic!("Error during send: {}", e));
    println!("Query Sent");


    let mut msg = [0; 512];
    socket
        .recv_from(&mut msg)
        .unwrap_or_else(|e| panic!("Error recv: {}", e));
    println!("Response Receive");

    drop(socket);

    msg.to_vec()
}


///Sends DNS query by TCP to address of resolver given and returns the response
fn send_get_message_from_resolver_tcp(hex_string: String, resolver_addr: &str) -> Vec<u8> {
    let bytes = hex::decode(hex_string).unwrap();
    
    let mut stream = TcpStream::connect(resolver_addr).expect("No connection");
    
    //add length of stream
    let msg_length: u16 = bytes.len() as u16;
    let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];
    let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

    match stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000))) {
        Err(_) => panic!("Error setting read timeout for socket"),
        Ok(_) => (),
    }

    match stream.write(&full_msg) {
        Err(_) => panic!("Error: could not write to stream"),
        Ok(_) => (),
    }

    let mut received_msg = [0; 2];
        let number_of_bytes = stream.read(&mut received_msg).expect("No data received");

        if number_of_bytes == 0 {
            return Vec::new();
        }

        let mut tcp_msg_len = (received_msg[0] as u16) << 8 | received_msg[1] as u16;
        let mut vec_msg: Vec<u8> = Vec::new();

        while tcp_msg_len > 0 {
            let mut msg = [0; 512];
            let number_of_bytes_msg = stream.read(&mut msg).expect("No data received");
            tcp_msg_len = tcp_msg_len - number_of_bytes_msg as u16;
            vec_msg.append(&mut msg.to_vec());
        }

        //FIXME: arreglar como esta esta funcion y cerar socket
        return Some(vec_msg).unwrap().to_vec();

}



//TODO: trunceted response
//TODO: queries any type (255)
//TODO: type WKS (deprecated)

//FIXME: UDP queries
//FIXME: AA answers 
