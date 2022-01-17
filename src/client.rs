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

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::UdpSocket;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};


pub fn run_client() {
    //Start timestamp
    let now = Instant::now();

    // Create randon generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query msg
    let query_msg = DnsMessage::new_query_message(
        HOST_NAME.to_string(),
        QTYPE,
        QCLASS,
        0,
        false,
        query_id,
    );

    // Create response buffer
    let mut dns_message = DnsMessage::new();

    // Send query by UDP
    if TRANSPORT == "UDP" {
        let socket = UdpSocket::bind(CLIENT_IP_PORT).expect("No connection");
        let msg_to_bytes = query_msg.to_bytes();

        socket.send_to(&msg_to_bytes, RESOLVER_IP_PORT);

        // Hashmap to save incomplete messages
        let mut messages = HashMap::<u16, DnsMessage>::new();

        socket.set_read_timeout(Some(Duration::from_millis(TIMEOUT*1000)));

        loop {
            let response_result = Resolver::receive_udp_msg(socket.try_clone().unwrap(), messages.clone());
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

        stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT*1000)));

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
                    println!("Txt: {}", val.get_text())
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
                    println!("Txt: {}", val.get_text())
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
                    println!("Txt: {}", val.get_text())
                }
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

pub fn receive_udp_msg(
    socket: UdpSocket,
    mut messages: HashMap<u16, DnsMessage>,
) -> Option<(DnsMessage, String)> {
    let mut msg = [0; 512];
    let (number_of_bytes_msg, address) = match socket.recv_from(&mut msg) {
        Ok((bytes, addr)) => (bytes, addr.to_string()),
        Err(e) => (0, "".to_string()),
    };

    if number_of_bytes_msg == 0 {
        return None;
    }

    let dns_msg_parsed_result = DnsMessage::from_bytes(&msg);

    match dns_msg_parsed_result {
        Ok(_) => {}
        Err(_) => {
            return Some((DnsMessage::format_error_msg(), address));
        }
    };

    let dns_msg_parsed = dns_msg_parsed_result.unwrap();

    let query_id = dns_msg_parsed.get_query_id();
    let trunc = dns_msg_parsed.get_header().get_tc();

    match messages.get(&query_id) {
        Some(val) => {
            let mut msg = val.clone();
            msg.add_answers(dns_msg_parsed.get_answer());
            msg.add_authorities(dns_msg_parsed.get_authority());
            msg.add_additionals(dns_msg_parsed.get_additional());

            if trunc == true {
                messages.insert(query_id, msg.clone());

                return None;
            } else {
                msg.update_header_counters();
                let mut header = msg.get_header();
                header.set_tc(false);

                msg.set_header(header);
                messages.remove(&query_id);

                return Some((msg.clone(), address));
            }
        }
        None => {
            if trunc == true {
                messages.insert(query_id, dns_msg_parsed);
                return None;
            } else {
                return Some((dns_msg_parsed, address));
            }
        }
    }
}

pub fn receive_tcp_msg(mut stream: TcpStream) -> Option<Vec<u8>> {
    let mut received_msg = [0; 2];
    let number_of_bytes = stream.read(&mut received_msg).expect("No data received");

    if number_of_bytes == 0 {
        return None;
    }

    let mut tcp_msg_len = (received_msg[0] as u16) << 8 | received_msg[1] as u16;
    let mut vec_msg: Vec<u8> = Vec::new();

    while tcp_msg_len > 0 {
        let mut msg = [0; 512];
        let number_of_bytes_msg = stream.read(&mut msg).expect("Temporary Error");
        tcp_msg_len = tcp_msg_len - number_of_bytes_msg as u16;
        vec_msg.append(&mut msg.to_vec());
    }

    return Some(vec_msg);
}
