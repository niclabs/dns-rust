pub mod config;

use crate::client::config::CLIENT_IP_PORT;
use crate::client::config::HOST_NAME;
use crate::client::config::QCLASS;
use crate::client::config::QTYPE;
use crate::client::config::RESOLVER_IP_PORT;
use crate::client::config::TIMEOUT;

use crate::client::config::TRANSPORT;
use crate::message::rdata::Rdata;
use crate::message::DnsMessage;
use crate::resolver::Resolver;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::{Write};
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

pub fn run_client() {

    let host_name: &str = HOST_NAME;

    let transport: &str = TRANSPORT;

    //Start timestamp
    let now = Instant::now();

    // Create randon generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    let mut dns_message = create_client_query(host_name,transport,QTYPE,QCLASS);
    dns_message.print_dns_message();

    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    

}

pub fn create_client_query(host_name: &str , transport: &str , qtype: u16 , qclass: u16) -> DnsMessage{
    //Start timestamp
    let now = Instant::now();

    // Create randon generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query msg
    let query_msg =
        DnsMessage::new_query_message(host_name.to_string(), QTYPE, QCLASS, 0, false, query_id);
    
    // Create response buffer
    let mut dns_message = DnsMessage::new();

    // Send query by UDP
    if transport == "UDP" {
        let socket = UdpSocket::bind(CLIENT_IP_PORT).expect("No connection");
        let msg_to_bytes = query_msg.to_bytes();

        match socket.send_to(&msg_to_bytes, RESOLVER_IP_PORT){
            Err(_) => panic!("Error sending query"),
            Ok(_) => (),
        }

        // Hashmap to save incomplete messages
        let messages = HashMap::<u16, DnsMessage>::new();

        match socket.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000))){
            Err(_) => panic!("Error setting read timeout for socket"),
            Ok(_) => (),
        }

        loop {
            let response_result =
                Resolver::receive_udp_msg(socket.try_clone().unwrap(), messages.clone());
            let messages_len = messages.len();

            match response_result {
                Some(val) => {
                    dns_message = val.0;

                    break;
                }
                None => {
                    if messages_len == messages.len() {
                        panic!("Temporary Error");
                    }
                    continue;
                }
            }
        }
    }

    // Send query by TCP
    if transport == "TCP" {
        let mut stream = TcpStream::connect(RESOLVER_IP_PORT).expect("No connection");

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        match stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000))){
            Err(_) => panic!("Error setting read timeout for socket"),
            Ok(_) => (),
        }

        match stream.write(&full_msg){
            Err(_) => panic!("Error: could not write to stream"),
            Ok(_) => (),
        }

        match Resolver::receive_tcp_msg(stream) {
            Some(val) => {
                dns_message = match DnsMessage::from_bytes(&val) {
                    Ok(msg) => {
                        msg
                    },
                    Err(_) => {DnsMessage::format_error_msg()},
                };
            }
            None => {
                panic!("Temporary Error");
            }
        }
    }
    dns_message
}