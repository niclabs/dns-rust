pub mod config;
pub mod dns_cache;
pub mod domain_name;
pub mod global_tests;
pub mod message;
pub mod name_server;
pub mod resolver;
pub mod rr_cache;

use crate::message::rdata::Rdata;
use crate::message::DnsMessage;
use crate::name_server::master_file::MasterFile;
use crate::name_server::zone::NSZone;
use crate::name_server::NameServer;
use crate::resolver::slist::Slist;
use crate::resolver::Resolver;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

pub fn main() {
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

    let mut resolver = Resolver::new(
        add_sender_udp,
        delete_sender_udp,
        add_sender_tcp,
        delete_sender_tcp,
        add_sender_ns_udp,
        delete_sender_ns_udp,
        add_sender_ns_tcp,
        delete_sender_ns_tcp,
        update_cache_sender_udp,
        update_cache_sender_tcp,
        update_cache_sender_ns_udp,
        update_cache_sender_ns_tcp,
    );

    resolver.set_ip_address("192.168.1.89:58396".to_string());

    let mut sbelt = Slist::new();
    sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5000);

    resolver.set_sbelt(sbelt);

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

    /*
    // Name Server initialization
    let mut name_server = NameServer::new(
        false,
        delete_sender_udp.clone(),
        delete_sender_tcp.clone(),
        add_sender_ns_udp.clone(),
        delete_sender_ns_udp.clone(),
        add_sender_ns_tcp.clone(),
        delete_sender_ns_tcp.clone(),
    );
    name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

    // Resolver Initialization
    let mut local_resolver = Resolver::new(
        add_sender_udp,
        delete_sender_udp,
        add_sender_tcp,
        delete_sender_tcp,
        add_sender_ns_udp,
        delete_sender_ns_udp,
        add_sender_ns_tcp,
        delete_sender_ns_tcp,
    );
    local_resolver.set_ip_address("192.168.1.89:58396".to_string());
    local_resolver.set_ns_data(name_server.get_zones());

    let mut sbelt = Slist::new();
    sbelt.insert(".".to_string(), "198.41.0.4".to_string(), 5000);

    local_resolver.set_sbelt(sbelt);

    let local_resolver_ip = local_resolver.get_ip_address();

    thread::spawn(move || {
        name_server.run_name_server_tcp(
            "192.168.1.89".to_string(),
            local_resolver_ip,
            add_recv_ns_tcp,
            delete_recv_ns_tcp,
        );
    });

    local_resolver.run_resolver_tcp(add_recv_tcp, delete_recv_tcp);

    */
}

fn test_tcp() {
    let mut stream = TcpStream::connect("8.8.8.8:53").expect("No conection");

    println!("{}", stream.peer_addr().unwrap().to_string());

    println!("{}", stream.local_addr().unwrap().to_string());

    let new_msg: [u8; 33] = [
        0, 31, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 0b01100100, 0b01100011, 0b01100011, 6,
        0b01110101, 0b01100011, 0b01101000, 0b01101001, 0b01101100, 0b01100101, 2, 0b01100011,
        0b01101100, 0, 0, 1, 0, 1,
    ];

    stream.write(&new_msg);

    println!("{}", "Enviado");

    let mut received_msg = [0; 512];
    let size = stream.read(&mut received_msg);

    println!("Recibidos {} bytes", size.unwrap());
}

fn test_udp() {
    let socket = UdpSocket::bind("192.168.1.89:58396").expect("Failed to bind host socket");

    let new_msg: [u8; 31] = [
        0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 0b01100100, 0b01100011, 0b01100011, 6, 0b01110101,
        0b01100011, 0b01101000, 0b01101001, 0b01101100, 0b01100101, 2, 0b01100011, 0b01101100, 0,
        0, 1, 0, 1,
    ];

    socket
        .send_to(&new_msg, "8.8.8.8:53")
        .expect("failed to send message");

    println!("{}", "Enviado");

    let mut received_msg = [0; 512];
    let (number_of_bytes, src_address) = socket
        .recv_from(&mut received_msg)
        .expect("No data received");

    let dns_msg = DnsMessage::from_bytes(&received_msg).unwrap();

    let answers = dns_msg.get_answer();

    for answer in answers {
        let name = answer.get_name();
        let r_data = answer.get_rdata();

        println!("Name: {}", name.get_name());

        let a_rdata = match r_data {
            Rdata::SomeARdata(val) => val,
            _ => unreachable!(),
        };

        let address = a_rdata.get_address();

        println!("Address:");
        for a in address {
            println!("{}", a);
        }
    }

    println!("Recibidos {} bytes", number_of_bytes);
}
