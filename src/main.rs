pub mod client;
pub mod dns_cache;
pub mod domain_name;
pub mod message;
pub mod resolver;
pub mod rr_cache;
pub mod server;

use crate::resolver::slist::Slist;
use crate::resolver::Resolver;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub fn main() {
    /*
    let mut stream = TcpStream::connect("8.8.8.8:53").unwrap();

    println!("{}", stream.local_addr().unwrap().ip().to_string());

    let new_msg: [u8; 10] = [0, 1, 0, 0, 0, 1, 0, 0, 0, 0];

    stream.write(&new_msg);

    println!("{}", "Enviado");

    let mut received_msg = [0; 512];
    stream.read(&mut received_msg);

    println!("{}", "Recibido");

    for i in received_msg {
        println!("{}", i);
    }
    */

    let mut resolver = Resolver::new();

    resolver.set_ip_address("192.168.1.89".to_string());
    resolver.set_port("53".to_string());

    let mut sbelt = Slist::new();
    sbelt.insert(".".to_string(), "199.7.83.42".to_string(), 5.0);

    resolver.set_sbelt(sbelt);

    resolver.run_resolver();
}
