// use std::sync::mpsc;
// use std::thread;
// mod common;

// use dns_rust::{
//     // config::RESOLVER_IP_PORT,
//     // config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
//     // config::{ SBELT_ROOT_IPS},
//     // name_server::{master_file::MasterFile, zone::NSZone},
//     client::{create_client_query,
//             send_client_query, config::RESOLVER_IP_PORT},
//     message::{DnsMessage,
//         rdata::Rdata},
// // use dns_rust::message::rdata::Rdata;
//     // utils::get_string_stype,
// };

//Thist client is tested with the google resolver -> 8.8.8.8:53



// fn test_query_udp() {
// }

// fn test_query_tcp() {
// }

// fn test_query_nonet() {
// }

// fn test_timeout_query_nonet() {
// }

// fn test_timeout_query_udp() {
// }

// fn test_timeout_query_tcp() {
// }

// #[test]
// fn test_qtype_a_example() {
//     //TODO: crear query que la envie a cualquier resolver pero la respuesta sea lo que e qiere
//     let google_resolver = "8.8.8.8:53";    
//     // println!("HOLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL");
    

//     //create client query
//     let mut client_query: DnsMessage = create_client_query("example.com",
//                                                             1,
//                                                         1);
//     client_query.print_dns_message();

//     // println!("HOLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL");
//     //send and receibe response
//     let response = send_client_query(
//                     "UDP",
//                     google_resolver,
//                     client_query);

                    // d send it to the resolver
                    // let dns_message_query = create_client_query(HOST_NAME, QTYPE, QCLASS );
                
                    // //send query and get response
                    // let mut dns_message = send_client_query(TRANSPORT,RESOLVER_IP_PORT,dns_message_query);
                
    
    
    // // println!("HOLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL");
    // // // query to our resolver
    // // let header = response.get_header();
    // // let answers = response.get_answer();
    // // let answer_count = header.get_ancount();

    // // if answer_count > 0 {
    // //     let answer = &answers[0];
    // //     let ip = match answer.get_rdata() {
    // //         Rdata::SomeARdata(val) => val.get_string_address(),
    // //         _ => "".to_string(),
    // //     };

    // //     assert_eq!(ip, "93.184.216.34");
    // // } else {
    // //     println!("no answers")
    // // }

    // // println!("HOLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL");

// }