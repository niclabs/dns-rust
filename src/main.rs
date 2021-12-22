pub mod client;
pub mod dns_cache;
pub mod domain_name;
pub mod message;
pub mod resolver;
pub mod rr_cache;
pub mod server;
pub mod name_server;

use crate::message::rdata::Rdata;
use crate::message::DnsMessage;
use crate::resolver::slist::Slist;
use crate::resolver::Resolver;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::UdpSocket;
use crate::name_server::master_file::MasterFile;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn main() {
    /*
    test_udp();
    */

    /*
    let mut resolver = Resolver::new();

    resolver.set_ip_address("192.168.1.89".to_string());
    resolver.set_port("58396".to_string());

    let mut sbelt = Slist::new();
    sbelt.insert(".".to_string(), "8.8.8.8".to_string(), 5.0);

    resolver.set_sbelt(sbelt);

    resolver.run_resolver_udp();
    */

    MasterFile::from_file("test.txt".to_string());

}

fn test_tcp() {
    let mut stream = TcpStream::connect("8.8.8.8:53").expect("No conection");

    println!("{}", stream.peer_addr().unwrap().to_string());

    println!("{}", stream.local_addr().unwrap().to_string());

    let new_msg: [u8; 31] = [
        0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 0b01100100, 0b01100011, 0b01100011, 6, 0b01110101,
        0b01100011, 0b01101000, 0b01101001, 0b01101100, 0b01100101, 2, 0b01100011, 0b01101100, 0,
        0, 1, 0, 1,
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

    let dns_msg = DnsMessage::from_bytes(&received_msg);

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
