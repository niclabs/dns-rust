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
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

pub fn run_client() {
    //Start timestamp
    let now = Instant::now();

    // Create randon generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query msg
    let query_msg =
        DnsMessage::new_query_message(HOST_NAME.to_string(), QTYPE, QCLASS, 0, false, query_id);

    // Create response buffer
    let mut dns_message = DnsMessage::new();

    // Send query by UDP
    if TRANSPORT == "UDP" {
        let socket = UdpSocket::bind(CLIENT_IP_PORT).expect("No connection");
        let msg_to_bytes = query_msg.to_bytes();

        socket.send_to(&msg_to_bytes, RESOLVER_IP_PORT);

        // Hashmap to save incomplete messages
        let mut messages = HashMap::<u16, DnsMessage>::new();

        socket.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000)));

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
    if TRANSPORT == "TCP" {
        let mut stream = TcpStream::connect(RESOLVER_IP_PORT).expect("No connection");

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000)));

        stream.write(&full_msg);

        match Resolver::receive_tcp_msg(stream) {
            Some(val) => {
                dns_message = DnsMessage::from_bytes(&val).unwrap();
            }
            None => {
                panic!("Temporary Error");
            }
        }
    }

    // Get the message and print the information
    let header = dns_message.get_header();
    let answers = dns_message.get_answer();
    let authority = dns_message.get_authority();
    let additional = dns_message.get_additional();

    let answer_count = header.get_ancount();
    let authority_count = header.get_nscount();
    let additional_count = header.get_arcount();

    // Not data found error
    if answer_count == 0 && header.get_qr() == true && header.get_aa() == true {
        println!("Not data found");
    } else {
        println!("-------------------------------------");
        println!(
            "Answers: {} - Authority: {} - Additional: {}",
            answer_count, authority_count, additional_count
        );
        println!("-------------------------------------");

        for answer in answers {
            match answer.get_rdata() {
                Rdata::SomeARdata(val) => {
                    println!("Ip Address: {}", val.get_string_address())
                }
                Rdata::SomeNsRdata(val) => {
                    println!("Name Server: {}", val.get_nsdname().get_name())
                }
                Rdata::SomeCnameRdata(val) => {
                    println!("Cname: {}", val.get_cname().get_name())
                }
                Rdata::SomeHinfoRdata(val) => {
                    println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                }
                Rdata::SomeMxRdata(val) => {
                    println!(
                        "Preference: {} - Exchange: {}",
                        val.get_preference(),
                        val.get_exchange().get_name()
                    )
                }
                Rdata::SomePtrRdata(val) => {
                    println!("Ptr name: {}", val.get_ptrdname().get_name())
                }
                Rdata::SomeSoaRdata(val) => {
                    println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                }
                Rdata::SomeTxtRdata(val) => {
                    println!("Txt: {:#?}", val.get_text())
                }
            }
        }

        for answer in authority {
            match answer.get_rdata() {
                Rdata::SomeARdata(val) => {
                    println!("Ip Address: {}", val.get_string_address())
                }
                Rdata::SomeNsRdata(val) => {
                    println!("Name Server: {}", val.get_nsdname().get_name())
                }
                Rdata::SomeCnameRdata(val) => {
                    println!("Cname: {}", val.get_cname().get_name())
                }
                Rdata::SomeHinfoRdata(val) => {
                    println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                }
                Rdata::SomeMxRdata(val) => {
                    println!(
                        "Preference: {} - Exchange: {}",
                        val.get_preference(),
                        val.get_exchange().get_name()
                    )
                }
                Rdata::SomePtrRdata(val) => {
                    println!("Ptr name: {}", val.get_ptrdname().get_name())
                }
                Rdata::SomeSoaRdata(val) => {
                    println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                }
                Rdata::SomeTxtRdata(val) => {
                    println!("Txt: {:#?}", val.get_text())
                }
            }
        }

        for answer in additional {
            match answer.get_rdata() {
                Rdata::SomeARdata(val) => {
                    println!("Ip Address: {}", val.get_string_address())
                }
                Rdata::SomeNsRdata(val) => {
                    println!("Name Server: {}", val.get_nsdname().get_name())
                }
                Rdata::SomeCnameRdata(val) => {
                    println!("Cname: {}", val.get_cname().get_name())
                }
                Rdata::SomeHinfoRdata(val) => {
                    println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                }
                Rdata::SomeMxRdata(val) => {
                    println!(
                        "Preference: {} - Exchange: {}",
                        val.get_preference(),
                        val.get_exchange().get_name()
                    )
                }
                Rdata::SomePtrRdata(val) => {
                    println!("Ptr name: {}", val.get_ptrdname().get_name())
                }
                Rdata::SomeSoaRdata(val) => {
                    println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                }
                Rdata::SomeTxtRdata(val) => {
                    println!("Txt: {:#?}", val.get_text())
                }
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
