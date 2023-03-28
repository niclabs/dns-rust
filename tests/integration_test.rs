// use dns_rust::client::create_client_query;

// use dns_rust::message::rdata::Rdata;

// use dns_rust::resolver::Resolver;
// use dns_rust::{
//     config::RESOLVER_IP_PORT,
//     config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
//     name_server::{master_file::MasterFile, zone::NSZone},
//     resolver,
// };
// use std::sync::mpsc;
// use std::thread;



//-------Test practica juaquin -------
// // TEST EXAMPLE.COM
// #[test]
// fn test_qtype_a_example() {
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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
//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

//     thread::spawn(move || {
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
//         //let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
//         //client_query.print_dns_message();
//     });

//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;
//     let our_resolver = RESOLVER_IP_PORT;
//     // query to our resolver
//     let client_query_to_our_resolver =
//         create_client_query(host_name, transport, qtype, qclass, our_resolver);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();

//     if answer_count > 0 {
//         let answer = &answers[0];
//         let ip = match answer.get_rdata() {
//             Rdata::SomeARdata(val) => val.get_string_address(),
//             _ => "".to_string(),
//         };

//         assert_eq!(ip, "93.184.216.34");
//     } else {
//         println!("no answers")
//     }
// }

// #[test]
// fn test_qtype_ns_example() {
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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
//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

//     thread::spawn(move || {
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
//         //let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
//         //client_query.print_dns_message();
//     });
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 2;
//     let qclass = 1;
//     let our_resolver = RESOLVER_IP_PORT;
//     // query to our resolver
//     let client_query_to_our_resolver =
//         create_client_query(host_name, transport, qtype, qclass, our_resolver);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();

//     //let answer = &answers[0];
//     for answer in answers {
//         match answer.get_rdata() {
//             Rdata::SomeNsRdata(val) => {
//                 let name = val.get_nsdname().get_name();
//                 println!("{}", name);
//                 assert!(name == "a.iana-servers.net" || name == "b.iana-servers.net");
//             }
//             _ => {
//                 "".to_string();
//             }
//         };
//     }
// }

// #[test]
// //FIXME
// fn test_qtype_mx_example() {
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 15;
//     let qclass = 1;
//     let our_resolver = RESOLVER_IP_PORT;
//     // query to our resolver
//     let client_query_to_our_resolver =
//         create_client_query(host_name, transport, qtype, qclass, our_resolver);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();
// }

// #[test]
// fn test_invalid_domain(){
//     let host_name = "?test.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;
//     let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let header = client_query_to_our_resolver.get_header();
//     let aut = client_query_to_our_resolver.get_authority();
//     // FIXME: entrega 6 answers en vez de 3, pero es porque estan repetidas.
//     let answer_count = header.get_ancount();
//     //let uwu = header.get
//     //assert_eq!(0,answer_count);
//     println!("{}" , aut.len());
//     for autho in aut{
//         assert_eq!(autho.get_name().get_name() , "com.");
//         // TODO: verificar que sea un SOA
//         assert_eq!(autho.get_string_type() , "SOA");
//         match autho.get_rdata(){
//             Rdata::SomeSoaRdata(val) => {
//                 println!("dsfsdfdsfsd");
//                 assert_eq!("a.gtld-servers.net." , val.get_mname().get_name());
//                 assert_eq!("nstld.verisign-grs.com." , val.get_rname().get_name());
//             }
//             _ => {}
//         }

//     }
// }
// #[test]
// fn rfc1034_standard_queries_test_6_2_1() {

//     // Run resolver.
//     // thread::spawn(move || {
//     //     resolver::run_resolver(
//     //         RESOLVER_IP_PORT,
//     //         SBELT_ROOT_IPS,
//     //         HashMap::<u16, HashMap<String, NSZone>>::new(),
//     //     );
//     // });

//     // thread::sleep(time::Duration::from_millis(40));
//     // client::create_client_query("dcc.uchile.cl", "TCP", 1, 1);
// }

// #[test]
// fn test_6_2_1_AA() {
//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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
    
//     //vedsfsdfc!();
//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     // Create and send querie [otro threafs]
    
//     let mut dns_message = client::create_client_query("uchile.cl", "TCP", 1, 1);
//     dns_message.print_dns_message();

//     //Check answer
//     // let header = dns_message.get_header();
//     // let question = dns_message.get_question();
//     // let answers = dns_message.get_answer();
//     // let authority = dns_message.get_authority();
//     // let additional = dns_message.get_additional();

//     // let answer_count = header.get_ancount();
//     // let authority_count = header.get_nscount();
//     // let additional_count = header.get_arcount();

//     // // 6 . 2 . 1
//     // //test header
//     // /*
//     //            +---------------------------------------------------+
//     // Header     | OPCODE=SQUERY, RESPONSE, AA                       |
//     //            +---------------------------------------------------+
//     // */
//     // let qr = header.get_qr();
//     // let AA = header.get_aa();
//     // let op_code = header.get_op_code();

//     // assert_eq!(qr, true); // check if is a response
//     // assert_eq!(AA, true);
//     // assert_eq!(op_code, 0);

//     // //test question
//     // let qname = question.get_qname().get_name();
//     // let qtype = question.get_qtype();
//     // let qclass = question.get_qclass();

//     // assert_eq!(qname, "SRI-NIC.ARPA.");
//     // assert_eq!(qclass, 1);
//     // assert_eq!(qtype, 1);

//     // //test Answer
//     // assert_eq!(header.get_ancount(), 2);
//     // for answer in answers {
//     //     println!("sadfsfdsf")
//     //     //test array of resource records
//     // }

//     // //test Authority
//     // let authority_count = header.get_nscount();
//     // assert_eq!(authority_count, 0);

//     // //test Additional
//     // let additional_count = header.get_arcount();
//     // assert_eq!(additional_count, 0);
// }
// #[test]
// fn test_6_2_1_cache() {
//     //QNAME=SRI-NIC.ARPA, QTYPE=A, answer from local
    
//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
    

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     //Add queries informatios to Resolver cache
//     let mut dns_message = client::create_client_query("SRI-NIC.ARPA", "TCP", 1, 1);

//     // Create and send querie
//     let mut dns_message = client::create_client_query("SRI-NIC.ARPA", "TCP", 1, 1);
//     // dns_message.print_dns_message();

//     //dns message
//     // let header = dns_message.get_header();
//     // let question = dns_message.get_question();
//     // let answers = dns_message.get_answer();

//     // let qr = header.get_qr();
//     // let AA = header.get_aa();
//     // let op_code = header.get_op_code();
//     // let RD = header.get_rd();

//     // assert_eq!(RD, false); //recursion desire
//     // assert_eq!(qr, true); // check if is a response
//     // assert_eq!(AA, false); //no autoritative answer
//     // assert_eq!(op_code, 0);

//     //Question Section
//     // let qname = question.get_qname().get_name();
//     // let qtype = question.get_qtype();
//     // let qclass = question.get_qclass();

//     // assert_eq!(qname, "SRI-NIC.ARPA.");
//     // assert_eq!(qclass, 1);
//     // assert_eq!(qtype, 1);

//     //Answer Section
//     // assert_eq!(header.get_ancount(), 2);
//     // for answer in answers.iter() {
//     //     //TODO: assert answer 
//     // }

//     //Authority Section
//     // let authority_count = header.get_nscount();
//     // assert_eq!(authority_count, 0);

//     //Additional Section
//     // let additional_count = header.get_arcount();
//     // assert_eq!(additional_count, 0);
// }

// #[test]
// fn test_6_2_2(){
//     // QNAME=SRI-NIC.ARPA, QTYPE=*

//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
    

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     //Add queries informatios to Resolver cache
//     let mut dns_message = client::create_client_query("SRI-NIC.ARPA", "TCP",255 , 1);


//     //dns message
//     let header = dns_message.get_header();
//     let question = dns_message.get_question();
//     let answers = dns_message.get_answer();
//     let authority  = dns_message.get_authority();
//     let additional = dns_message.get_additional();

//     //header
//     let qr = header.get_qr();
//     let AA = header.get_aa();
//     let op_code = header.get_op_code();
//     let RD = header.get_rd();

//     // assert_eq!(RD, false); //recursion desire
//     // assert_eq!(qr, true); // check if is a response
//     // assert_eq!(AA, true); //no autoritative answer
//     // assert_eq!(op_code, 0);

//     //Question Section
//     // let qname = question.get_qname().get_name();
//     // let qtype = question.get_qtype();
//     // let qclass = question.get_qclass();

//     // assert_eq!(qname, "SRI-NIC.ARPA.");
//     // assert_eq!(qclass, 1);
//     // assert_eq!(qtype, 1);

//     //Answer Section
//     // assert_eq!(header.get_ancount(), 2);
//     // for answer in answers.iter() {
//     //     //TODO: assert answer 
//     // }

//     //Authority Section
//     // let authority_count = header.get_nscount();
//     // assert_eq!(authority_count, 0);

//     //Additional Section
//     // let additional_count = header.get_arcount();
//     // assert_eq!(additional_count, 0);
// }


// #[test]
// fn test_6_2_2_AA(){
//     // QNAME=SRI-NIC.ARPA, QTYPE=*

//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
    

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     //Add queries informatios to Resolver cache
//     let mut dns_message = client::create_client_query("SRI-NIC.ARPA", "TCP",255 , 1);


//     //dns message
//     let header = dns_message.get_header();
//     let question = dns_message.get_question();
//     let answers = dns_message.get_answer();
//     let authority  = dns_message.get_authority();
//     let additional = dns_message.get_additional();

//     //header
//     let qr = header.get_qr();
//     let AA = header.get_aa();
//     let op_code = header.get_op_code();
//     let RD = header.get_rd();

//     // assert_eq!(RD, false); //recursion desire
//     // assert_eq!(qr, true); // check if is a response
//     // assert_eq!(AA, true); //no autoritative answer
//     // assert_eq!(op_code, 0);

//     //Question Section
//     // let qname = question.get_qname().get_name();
//     // let qtype = question.get_qtype();
//     // let qclass = question.get_qclass();

//     // assert_eq!(qname, "SRI-NIC.ARPA.");
//     // assert_eq!(qclass, 1);
//     // assert_eq!(qtype, 1);

//     //Answer Section
//     // assert_eq!(header.get_ancount(), 2);
//     // for answer in answers.iter() {
//     //     //TODO: assert answer 
//     // }

//     //Authority Section
//     // let authority_count = header.get_nscount();
//     // assert_eq!(authority_count, 0);

//     //Additional Section
//     // let additional_count = header.get_arcount();
//     // assert_eq!(additional_count, 0);
// }

// fn test_6_2_3_MX(){
//     // QNAME=SRI-NIC.ARPA, QTYPE=* two diferent Name Servers

//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
    

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     //Add queries informatios to Resolver cache
//     let mut dns_message = client::create_client_query("SRI-NIC.ARPA", "TCP",15 , 1);


//     //dns message
//     let header = dns_message.get_header();
//     let question = dns_message.get_question();
//     let answers = dns_message.get_answer();
//     let authority  = dns_message.get_authority();
//     let additional = dns_message.get_additional();

//     //header
//     let qr = header.get_qr();
//     let AA = header.get_aa();
//     let op_code = header.get_op_code();
//     let RD = header.get_rd();

//     //TODO: terminar test 
//     // assert_eq!(RD, false); //recursion desire
//     // assert_eq!(qr, true); // check if is a response
//     //FIXME: 
//     // assert_eq!(AA, ???); //no autoritative answer  
//     // assert_eq!(op_code, 0);

//     //Question Section
//     // let qname = question.get_qname().get_name();
//     // let qtype = question.get_qtype();
//     // let qclass = question.get_qclass();

//     // assert_eq!(qname, "SRI-NIC.ARPA.");
//     // assert_eq!(qclass, 1);
//     // assert_eq!(qtype, 15);

//     //Answer Section
//     // assert_eq!(header.get_ancount(), ???);
//     // for answer in answers.iter() {
//     //     //TODO: assert answer 
//     // }

//     //Authority Section
//     // let authority_count = header.get_nscount();
//     // assert_eq!(authority_count, 0);

//     //Additional Section
//     // let additional_count = header.get_arcount();
//     // assert_eq!(additional_count, ????);
//     // for rr in additional.iter() {
//     //     //TODO: assert answer 
//     // }
// }


// fn test_6_2_4_NS(){
//     // QNAME=SRI-NIC.ARPA, QTYPE=* two diferent Name Servers

//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
    

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     //Add queries informatios to Resolver cache
//     let mut dns_message = client::create_client_query("SRI-NIC.ARPA", "TCP",2 , 1);


//     //dns message
//     let header = dns_message.get_header();
//     let question = dns_message.get_question();
//     let answers = dns_message.get_answer();
//     let authority  = dns_message.get_authority();
//     let additional = dns_message.get_additional();

//     //header
//     let qr = header.get_qr();
//     let AA = header.get_aa();
//     let op_code = header.get_op_code();
//     let RD = header.get_rd();

//     //TODO: terminar test 
//     // assert_eq!(RD, false); //recursion desire
//     // assert_eq!(qr, true); // check if is a response
//     //FIXME: 
//     // assert_eq!(AA, ???); //no autoritative answer  
//     // assert_eq!(op_code, 0);

//     //Question Section
//     // let qname = question.get_qname().get_name();
//     // let qtype = question.get_qtype();
//     // let qclass = question.get_qclass();

//     // assert_eq!(qname, "SRI-NIC.ARPA.");
//     // assert_eq!(qclass, 1);
//     // assert_eq!(qtype, 15);

//     //Answer Section
//     // assert_eq!(header.get_ancount(), ???);
//     // for answer in answers.iter() {
//     //     //TODO: assert answer 
//     // }

//     //Authority Section
//     // let authority_count = header.get_nscount();
//     // assert_eq!(authority_count, 0);

//     //Additional Section
//     // let additional_count = header.get_arcount();
//     // assert_eq!(additional_count, ????);
//     // for rr in additional.iter() {
//     //     //TODO: assert answer 
//     // }
// }
// fn test_6_2_5_A(){
//     //QNAME=SIR-NIC.ARPA, QTYPE=A  mistype
// }
// fn test_6_2_6_A(){
//     //QNAME=BRL.MIL, QTYPE=A  

// }
// fn test_6_2_7_A(){
//     // QNAME=USC-ISIC.ARPA, QTYPE=A

// }
// fn test_6_2_4_CNAME(){
//     //QNAME=USC-ISIC.ARPA, QTYPE=CNAME

// }
// #[test]
// fn sadsadsad(){
//     // Channels
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
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

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
    

//     //Run Resolver thread
//     thread::spawn(move || {
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

//     let mut dns_message = client::create_client_query("dcc.uchile.cl", "TCP",1 , 1 , RESOLVER_IP_PORT);

//     let a = dns_message.get_answer();
//     //let b = a[0];
//     dns_message.print_dns_message()


// }


// #[test]
// fn test_resolver(){

    

//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;
//     let google_resolver = "8.8.8.8:53";
//     let our_resolver = RESOLVER_IP_PORT;

//     // query to google resolver
//     let client_query_to_google: DnsMessage = create_client_query(host_name, transport, qtype, qclass, google_resolver);
//     let header = client_query_to_google.get_header();
//     let answers = client_query_to_google.get_answer();
//     let answer_count_a = header.get_ancount();

//     // query to our resolver
//     let client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , our_resolver);
//     let header_b = client_query_to_our_resolver.get_header();
//     let answers_b = client_query_to_our_resolver.get_answer();
//     let answer_count_b = header_b.get_ancount();

//     if answer_count_a > 0 && answer_count_b > 0{
//         let answer = &answers[0];
//         let ip = match answer.get_rdata(){
//             Rdata::SomeARdata(val) => {
//                 val.get_string_address()
//             }
//             _ => {"".to_string()}
//         };
//         println!("ip_1 = {}" , ip); 

//         let answer_b = &answers_b[0];
//         let ip_b = match answer_b.get_rdata(){
//             Rdata::SomeARdata(val) => {
//                 val.get_string_address()
//             }
//             _ => {"".to_string()}
//         };
//         println!("ip_2 = {}",ip_b);

//         //check if the two resolvers resolve the same ip
//         assert_eq!(ip , ip_b);

//     }
//     else {
//         println!("No answers");
//     } 
// }

// #[test]
// fn test_qtype_a(){
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
//         let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
//         let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
//         let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
//         let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
//         let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
//         let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
//         let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();
//         let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
//         let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
//         let (update_cache_sender_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
//         let (update_cache_sender_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

//         let (update_zone_udp, rx_update_zone_udp) = mpsc::channel();
//         let (update_zone_tcp, rx_update_zone_tcp) = mpsc::channel();
//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

//     thread::spawn(move || {
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
//         //let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
//     //client_query.print_dns_message();
//     });
    
//     let ten_millis = time::Duration::from_millis(1000);
//     thread::sleep(ten_millis);

//     let host_name = "test.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;

//     let client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();

//     if answer_count > 0 {
//         let answer = &answers[0];
//         println!("ttl = {}" , answer.get_ttl());
//         let ip = match answer.get_rdata(){
//             Rdata::SomeARdata(val) => {
//                 val.get_string_address()
//             }
//             _ => {"".to_string()}
//         };
//     println!("{}" , ip);
//     assert_eq!(ip,"67.225.146.248");
//     }

// }

// #[test]
// fn test_qtype_a_two_times(){
//     // FIXME   
//     let host_name = "test.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;

//     let client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();
//     println!("AA = {}" , header.get_aa());
//     let first_ttl = &answers[0].get_ttl();

//     if answer_count > 0 {
//         let answer = &answers[0];
//         println!("ttl = {}" , answer.get_ttl());
//         println!("id = {}" , client_query_to_our_resolver.get_query_id());

//         let ip = match answer.get_rdata(){
//             Rdata::SomeARdata(val) => {
//                 val.get_string_address()
//             }
//             _ => {"".to_string()}
//         };
//     println!("{}" , ip);
//     assert_eq!(ip,"67.225.146.248");
//     }
//     let ten_millis = time::Duration::from_millis(10000);
//     thread::sleep(ten_millis);


//     let client_query_to_our_resolver1 = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let header1 = client_query_to_our_resolver1.get_header();
//     let answers1 = client_query_to_our_resolver1.get_answer();
//     let answer_count = header1.get_ancount();
//     println!("AA = {}" , header1.get_aa());
//     let second_ttl = &answers1[0].get_ttl();
//     if answer_count > 0 {
//         let answer = &answers1[0];
//         println!("ttl = {}" , answer.get_ttl());
//         println!("id = {}" , client_query_to_our_resolver1.get_query_id());
//         let ip = match answer.get_rdata(){
//             Rdata::SomeARdata(val) => {
//                 val.get_string_address()
//             }
//             _ => {"".to_string()}
//         };
//     //println!("{}" , ip);
//     assert_eq!(ip,"67.225.146.248");
//     }
//     assert!( first_ttl != second_ttl); 


// }



// #[test]
// fn test_qtype_all(){
//     let host_name = "test.com";
//     let transport = "TCP";
//     let qtype = 255;
//     let qclass = 1;
//     let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     client_query_to_our_resolver.print_dns_message();
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();
//     println!("answer count = {}",answer_count);

//     let qr = header.get_qr();
//     let AA = header.get_aa();
//     let op_code = header.get_op_code();

//     assert_eq!(qr, true); // check if is a response
//     assert_eq!(AA, true);
//     assert_eq!(op_code, 0);


//     //let a = &answers[0];
//     for a in answers{
//     match a.get_rdata(){
//         Rdata::SomeARdata(val) => {
//             assert_eq!(val.get_string_address() , "67.225.146.248");
//         }
//         Rdata::SomeNsRdata(val) => {
//             assert!(val.get_nsdname().get_name() ==  "ns1.safesecureweb.com." ||
//                     val.get_nsdname().get_name() ==  "ns2.safesecureweb.com." || 
//                     val.get_nsdname().get_name() ==  "ns3.safesecureweb.com.");
//         }
//         Rdata::SomeTxtRdata(val) => {
//             //let texto = &val.get_text()[0];
//             assert!( val.get_text()[0] == "55d34914-636b-4a56-b349-fdb9f2c1eaca" || val.get_text()[0] == "google-site-verification=kW9t2V_S7WjOX57zq0tP8Ae_WJhRwUcZoqpdEkvuXJk");
//         }
//         Rdata::SomeSoaRdata(val) => {
//             assert_eq!(val.get_mname().get_name() , "ns1.safesecureweb.com.");
//             assert_eq!(val.get_rname().get_name() , "abuse.ntirety.com." );
//             assert_eq!(val.get_serial() , 212);
//             assert_eq!(val.get_refresh() , 10800);
//             assert_eq!(val.get_retry() , 3600);
//             assert_eq!(val.get_expire() , 604800);
//             assert_eq!(val.get_minimum() , 3600);
//         }
//         _ => {}
//     }
// }
//     /*
//     ;; ANSWER SECTION:
//     test.com.		3600	IN	SOA	ns1.safesecureweb.com. abuse.ntirety.com. 212 10800 3600 604800 3600
//     test.com.		3600	IN	NS	ns3.safesecureweb.com.
//     test.com.		3600	IN	NS	ns2.safesecureweb.com.
//     test.com.		3600	IN	NS	ns1.safesecureweb.com.
//     test.com.		3600	IN	A	67.225.146.248
//     test.com.		3600	IN	TXT	"55d34914-636b-4a56-b349-fdb9f2c1eaca"
//     test.com.		3600	IN	TXT	"google-site-verification=kW9t2V_S7WjOX57zq0tP8Ae_WJhRwUcZoqpdEkvuXJk"
//     */
// }

// #[test]
// fn test_qtype_mx(){
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 15;
//     let qclass = 1;
//     //let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);

//     let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     client_query_to_our_resolver.print_dns_message();
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();
//     println!("answer count = {}",answer_count);

//     //let qr = header.get_qr();
//     //let AA = header.get_aa();
//     //let op_code = header.get_op_code();

//     //assert_eq!(qr, true); // check if is a response
//     //assert_eq!(AA, true);
//     //assert_eq!(op_code, 0);


//     //let a = &answers[0];
//     /*
//     ;; ANSWER SECTION:
//     example.com.		21179	IN	MX	0 .

//     */
//     for a in answers{
//     match a.get_rdata(){
//         Rdata::SomeMxRdata(val) => {
//             assert_eq!(val.get_exchange().get_name() , ".");
//             assert_eq!(val.get_preference() , 0);
//         }
//         _ => {}
//     }


// }}

// #[test]
// fn test_qtype_mx_2(){
//     let host_name = "test.com";
//     let transport = "TCP";
//     let qtype = 15;
//     let qclass = 1;
//     //let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , "8.8.8.8:53");
//     client_query_to_our_resolver.print_dns_message();
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();
//     println!("answer count = {}",answer_count);
//     assert_eq!(answer_count,0);   

//     let aut = client_query_to_our_resolver.get_authority();
//     /*
//     ;; AUTHORITY SECTION:
//     test.com.		1402	IN	SOA	ns1.safesecureweb.com. abuse.ntirety.com. 212 10800 3600 604800 3600
//     */
//     for authority in aut{
//         match authority.get_rdata() {
//             Rdata::SomeSoaRdata(val) => {
//                 assert_eq!(val.get_mname().get_name(), "ns1.safesecurweb.com.");
//                 assert_eq!(val.get_rname().get_name() , "abuse.ntirety.com.");
//                 assert_eq!(val.get_serial() , 212);
//                 assert_eq!(val.get_refresh() , 10800);
//                 assert_eq!(val.get_retry(),3600);
//                 assert_eq!(val.get_expire() , 604800);
//                 assert_eq!(val.get_minimum() , 3600);
//             }
//             _ => {}
//         }
//     }

// }
// #[test]
// fn test_qtype_ns(){
//     let host_name = "test.com";
//     let transport = "TCP";
//     let qtype = 2;
//     let qclass = 1;
//     //let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     client_query_to_our_resolver.print_dns_message();
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     // FIXME: entrega 6 answers en vez de 3, pero es porque estan repetidas.
//     let answer_count = header.get_ancount();
//     for answer in answers{
//         match answer.get_rdata(){
//             Rdata::SomeNsRdata(val) => {
//                 let name = val.get_nsdname().get_name();
//                 assert!( name == "ns1.safesecureweb.com" || name == "ns2.safesecureweb.com" || name == "ns3.safesecureweb.com");}
//             _ => {}
//         }


//     }
// }

// #[test]
// fn test_invalid_domain(){
//     let host_name = "?test.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;
//     let mut client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , RESOLVER_IP_PORT);
//     let header = client_query_to_our_resolver.get_header();
//     let aut = client_query_to_our_resolver.get_authority();
//     // FIXME: entrega 6 answers en vez de 3, pero es porque estan repetidas.
//     let answer_count = header.get_ancount();
//     //let uwu = header.get
//     //assert_eq!(0,answer_count);
//     println!("{}" , aut.len());
//     for autho in aut{
//         assert_eq!(autho.get_name().get_name() , "com.");
//         // TODO: verificar que sea un SOA
//         assert_eq!(autho.get_string_type() , "SOA");
//         match autho.get_rdata(){
//             Rdata::SomeSoaRdata(val) => {
//                 println!("dsfsdfdsfsd");
//                 assert_eq!("a.gtld-servers.net." , val.get_mname().get_name());
//                 assert_eq!("nstld.verisign-grs.com." , val.get_rname().get_name());
//             }
//             _ => {}
//         }


//     }

// }

// #[test]
// fn test_udp_tcp(){
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 2;
//     let qclass = 1;
//     //let google_resolver = "8.8.8.8:53";
//     let our_resolver = RESOLVER_IP_PORT;

//     // query to google resolver
//     let mut client_query: DnsMessage = create_client_query(host_name, transport, qtype, qclass, our_resolver);
//     let header = client_query.get_header();
//     let answers = client_query.get_answer();
//     let answer_count_a = header.get_ancount();
//     client_query.print_dns_message();

//     let mut client_query_udp: DnsMessage = create_client_query(host_name, "UDP", qtype, qclass, our_resolver);
//     client_query_udp.print_dns_message();

// }

// #[test]
// fn resolver_thread(){
//     let (add_sender_udp, add_recv_udp) = mpsc::channel();
//         let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
//         let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
//         let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
//         let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
//         let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
//         let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
//         let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();
//         let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
//         let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
//         let (update_cache_sender_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
//         let (update_cache_sender_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

//         let (update_zone_udp, rx_update_zone_udp) = mpsc::channel();
//         let (update_zone_tcp, rx_update_zone_tcp) = mpsc::channel();
//     // Resolver Initialize
//     let mut resolver = Resolver::new(
//         add_sender_udp.clone(),
//         delete_sender_udp.clone(),
//         add_sender_tcp.clone(),
//         delete_sender_tcp.clone(),
//         add_sender_ns_udp.clone(),
//         delete_sender_ns_udp.clone(),
//         add_sender_ns_tcp.clone(),
//         delete_sender_ns_tcp.clone(),
//         update_cache_sender_udp.clone(),
//         update_cache_sender_tcp.clone(),
//         update_cache_sender_ns_udp.clone(),
//         update_cache_sender_ns_tcp.clone(),
//     );

//     resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

//     thread::spawn(move || {
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
//         //let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
//     //client_query.print_dns_message();
//     });

//     let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
//     client_query.print_dns_message();
//     let header = client_query.get_header();
//     let answers = client_query.get_answer();
//     let answer_count = header.get_ancount();

//     if answer_count > 0 {
//         let answer = &answers[0];
//         println!("ttl = {}" , answer.get_ttl());
//         let ip = match answer.get_rdata(){
//             Rdata::SomeARdata(val) => {
//                 val.get_string_address()
//             }
//             _ => {"".to_string()}
//         };
//     println!("{}" , ip);
//     assert_eq!(ip,"67.225.146.248");
//     }


// }



// // TEST EXAMPLE.COM
// #[test]
// fn test_qtype_a_example(){
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 1;
//     let qclass = 1;
//     let our_resolver = RESOLVER_IP_PORT;
//     // query to our resolver
//     let client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , our_resolver);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();

//     if answer_count > 0 {
//         let answer = &answers[0];
//     let ip = match answer.get_rdata(){
//         Rdata::SomeARdata(val) => {
//             val.get_string_address()
//         }
//         _ => {"".to_string()}
//     };

//     assert_eq!(ip, "93.184.216.34");
//     }
//     else{
//         println!("no answers")
//     }
// }


// #[test]
// fn test_qtype_ns_example(){
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 2;
//     let qclass = 1;
//     let our_resolver = RESOLVER_IP_PORT;
//     // query to our resolver
//     let client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , our_resolver);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();
    
//         //let answer = &answers[0];
//         for answer in answers{
//             match answer.get_rdata(){
//                 Rdata::SomeNsRdata(val) => {
//                         let name = val.get_nsdname().get_name();
//                         println!("{}" , name );
//                         assert!(name == "a.iana-servers.net" ||
//                                 name == "b.iana-servers.net");
//                 }
//                  _ => {"".to_string();}
//                 };
//         }
        
    
//         //assert_eq!(ip, "93.184.216.34");
        
        
// }

// #[test]
// fn test_qtype_mx_example(){
//     let host_name = "example.com";
//     let transport = "TCP";
//     let qtype = 15;
//     let qclass = 1;
//     let our_resolver = RESOLVER_IP_PORT;
//     // query to our resolver
//     let client_query_to_our_resolver = create_client_query(host_name, transport, qtype, qclass , our_resolver);
//     let header = client_query_to_our_resolver.get_header();
//     let answers = client_query_to_our_resolver.get_answer();
//     let answer_count = header.get_ancount();

// }
