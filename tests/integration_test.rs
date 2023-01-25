use dns_rust::client::create_client_query;

use dns_rust::message::rdata::Rdata;

use dns_rust::resolver::Resolver;
use dns_rust::{
    config::RESOLVER_IP_PORT,
    config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
    name_server::{master_file::MasterFile, zone::NSZone},
    resolver,
};
use std::sync::mpsc;
use std::thread;

// TEST EXAMPLE.COM
#[test]
fn test_qtype_a_example() {
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

    resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

    thread::spawn(move || {
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
        //let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
        //client_query.print_dns_message();
    });

    let host_name = "example.com";
    let transport = "TCP";
    let qtype = 1;
    let qclass = 1;
    let our_resolver = RESOLVER_IP_PORT;
    // query to our resolver
    let client_query_to_our_resolver =
        create_client_query(host_name, transport, qtype, qclass, our_resolver);
    let header = client_query_to_our_resolver.get_header();
    let answers = client_query_to_our_resolver.get_answer();
    let answer_count = header.get_ancount();

    if answer_count > 0 {
        let answer = &answers[0];
        let ip = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_string_address(),
            _ => "".to_string(),
        };

        assert_eq!(ip, "93.184.216.34");
    } else {
        println!("no answers")
    }
}

#[test]
fn test_qtype_ns_example() {
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

    resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

    thread::spawn(move || {
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
        //let mut client_query: DnsMessage = create_client_query("test.com", "TCP", 1 , 1, RESOLVER_IP_PORT);
        //client_query.print_dns_message();
    });
    let host_name = "example.com";
    let transport = "TCP";
    let qtype = 2;
    let qclass = 1;
    let our_resolver = RESOLVER_IP_PORT;
    // query to our resolver
    let client_query_to_our_resolver =
        create_client_query(host_name, transport, qtype, qclass, our_resolver);
    let header = client_query_to_our_resolver.get_header();
    let answers = client_query_to_our_resolver.get_answer();
    let answer_count = header.get_ancount();

    //let answer = &answers[0];
    for answer in answers {
        match answer.get_rdata() {
            Rdata::SomeNsRdata(val) => {
                let name = val.get_nsdname().get_name();
                println!("{}", name);
                assert!(name == "a.iana-servers.net" || name == "b.iana-servers.net");
            }
            _ => {
                "".to_string();
            }
        };
    }
}

#[test]
//FIXME
fn test_qtype_mx_example() {
    let host_name = "example.com";
    let transport = "TCP";
    let qtype = 15;
    let qclass = 1;
    let our_resolver = RESOLVER_IP_PORT;
    // query to our resolver
    let client_query_to_our_resolver =
        create_client_query(host_name, transport, qtype, qclass, our_resolver);
    let header = client_query_to_our_resolver.get_header();
    let answers = client_query_to_our_resolver.get_answer();
    let answer_count = header.get_ancount();
}
