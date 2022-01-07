use crate::dns_cache::DnsCache;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::name_server::zone::NSZone;
use crate::resolver::resolver_query::ResolverQuery;
use crate::resolver::slist::Slist;

use chrono::{DateTime, Utc};
use core::num;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::vec::Vec;

pub mod resolver_query;
pub mod slist;

#[derive(Clone)]
/// Struct that represents a dns resolver
pub struct Resolver {
    /// Ip address and port where the resolver will run
    ip_address: String,
    // Struct that contains a default server list to ask
    sbelt: Slist,
    // Cache for the resolver
    cache: DnsCache,
    // Name server data
    ns_data: HashMap<String, NSZone>,
    // Channel to share cache data between threads
    add_sender_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    delete_sender_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    add_sender_tcp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    delete_sender_tcp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between name server and resolver
    add_sender_ns_udp: Sender<(String, ResourceRecord)>,
    // Channel to delete cache data in name server and resolver
    delete_sender_ns_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between name server and resolver
    add_sender_ns_tcp: Sender<(String, ResourceRecord)>,
    // Channel to delete cache data in name server and resolver
    delete_sender_ns_tcp: Sender<(String, ResourceRecord)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_udp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_tcp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_ns_udp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_ns_tcp: Sender<(String, String, u32)>,
}

impl Resolver {
    /// Creates a new Resolver with default values
    pub fn new(
        add_sender_udp: Sender<(String, ResourceRecord)>,
        delete_sender_udp: Sender<(String, ResourceRecord)>,
        add_sender_tcp: Sender<(String, ResourceRecord)>,
        delete_sender_tcp: Sender<(String, ResourceRecord)>,
        add_sender_ns_udp: Sender<(String, ResourceRecord)>,
        delete_sender_ns_udp: Sender<(String, ResourceRecord)>,
        add_sender_ns_tcp: Sender<(String, ResourceRecord)>,
        delete_sender_ns_tcp: Sender<(String, ResourceRecord)>,
        update_cache_sender_udp: Sender<(String, String, u32)>,
        update_cache_sender_tcp: Sender<(String, String, u32)>,
        update_cache_sender_ns_udp: Sender<(String, String, u32)>,
        update_cache_sender_ns_tcp: Sender<(String, String, u32)>,
    ) -> Self {
        let mut cache = DnsCache::new();
        cache.set_max_size(100);

        let resolver = Resolver {
            ip_address: String::from(""),
            sbelt: Slist::new(),
            cache: cache,
            ns_data: HashMap::<String, NSZone>::new(),
            add_sender_udp: add_sender_udp,
            delete_sender_udp: delete_sender_udp,
            add_sender_tcp: add_sender_tcp,
            delete_sender_tcp: delete_sender_tcp,
            add_sender_ns_udp: add_sender_ns_udp,
            delete_sender_ns_udp: delete_sender_ns_udp,
            add_sender_ns_tcp: add_sender_ns_tcp,
            delete_sender_ns_tcp: delete_sender_ns_tcp,
            update_cache_sender_udp: update_cache_sender_udp,
            update_cache_sender_tcp: update_cache_sender_tcp,
            update_cache_sender_ns_udp: update_cache_sender_ns_udp,
            update_cache_sender_ns_tcp: update_cache_sender_ns_tcp,
        };

        resolver
    }

    pub fn run_resolver(
        &mut self,
        rx_add_udp: Receiver<(String, ResourceRecord)>,
        rx_delete_udp: Receiver<(String, ResourceRecord)>,
        rx_add_tcp: Receiver<(String, ResourceRecord)>,
        rx_delete_tcp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_udp: Receiver<(String, String, u32)>,
        rx_update_cache_tcp: Receiver<(String, String, u32)>,
    ) {
        let mut resolver_copy = self.clone();
        thread::spawn(move || {
            resolver_copy.run_resolver_udp(rx_add_udp, rx_delete_udp, rx_update_cache_udp);
        });

        self.run_resolver_tcp(rx_add_tcp, rx_delete_tcp, rx_update_cache_tcp);
    }

    // Runs a udp resolver
    pub fn run_resolver_udp(
        &mut self,
        rx_add_udp: Receiver<(String, ResourceRecord)>,
        rx_delete_udp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_udp: Receiver<(String, String, u32)>,
    ) {
        // Hashmap to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();

        // Hashmap to save incomplete messages
        let mut messages = HashMap::<u16, DnsMessage>::new();

        // Channels to send cache data between threads, resolvers and name server
        let tx_add_udp = self.get_add_sender_udp();
        let tx_delete_udp = self.get_delete_sender_udp();
        let tx_add_tcp = self.get_add_sender_tcp();
        let tx_delete_tcp = self.get_delete_sender_tcp();
        let tx_add_ns_udp = self.get_add_sender_ns_udp();
        let tx_delete_ns_udp = self.get_delete_sender_ns_udp();
        let tx_add_ns_tcp = self.get_add_sender_ns_tcp();
        let tx_delete_ns_tcp = self.get_delete_sender_ns_tcp();
        let tx_update_cache_udp = self.get_update_cache_udp();
        let tx_update_cache_tcp = self.get_update_cache_tcp();
        let tx_update_cache_ns_udp = self.get_update_cache_ns_udp();
        let tx_update_cache_ns_tcp = self.get_update_cache_ns_tcp();

        // Channel to delete queries ids from queries already response
        let (tx_delete_query, rx_delete_query): (Sender<ResolverQuery>, Receiver<ResolverQuery>) =
            mpsc::channel();

        // Channel to update resolver queries from queries in progress
        let (tx_update_query, rx_update_query): (Sender<ResolverQuery>, Receiver<ResolverQuery>) =
            mpsc::channel();

        // Create ip and port str
        let host_address_and_port = self.get_ip_address();

        // Creates an UDP socket
        let socket = UdpSocket::bind(&host_address_and_port).expect("Failed to bind host socket");
        println!("{}", "Socket Created");

        // Receives messages
        loop {
            println!("{}", "Waiting msg");

            // We receive the msg
            let mut dns_message_option =
                Resolver::receive_udp_msg(socket.try_clone().unwrap(), messages.clone());

            let (mut dns_message, mut src_address) = (DnsMessage::new(), "".to_string());

            println!("{}", "Message recv");

            match dns_message_option {
                Some(val) => {
                    dns_message = val.0;
                    src_address = val.1;
                }
                None => {
                    continue;
                }
            }

            // Delete queries already answered

            let mut queries_to_delete = rx_delete_query.try_iter();
            let mut next_query_value = queries_to_delete.next();

            while next_query_value.is_none() == false {
                let resolver_query_to_delete = next_query_value.unwrap();
                let id: u16 = resolver_query_to_delete.get_old_id();

                queries_hash_by_id.remove(&id);

                next_query_value = queries_to_delete.next();
            }

            //

            // Updates queries

            let mut queries_to_update = rx_update_query.try_iter();
            let mut next_query_to_update = queries_to_update.next();

            while next_query_to_update.is_none() == false {
                let resolver_query_to_update = next_query_to_update.unwrap();

                let id: u16 = resolver_query_to_update.get_main_query_id();

                queries_hash_by_id.insert(id, resolver_query_to_update);

                next_query_to_update = queries_to_update.next();
            }

            //

            // Delete from cache

            let mut received_delete = rx_delete_udp.try_iter();

            let mut next_value = received_delete.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (name, rr) = next_value.unwrap();
                let rr_type = rr.get_string_type();
                cache.remove(name, rr_type);
                next_value = received_delete.next();
            }

            self.set_cache(cache);

            //

            // Update response time cache

            let mut received_update = rx_update_cache_udp.try_iter();

            let mut next_value = received_update.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (host_name, address, response_time) = next_value.unwrap();
                cache.update_response_time(host_name, "A".to_string(), response_time, address);
                next_value = received_update.next();
            }

            self.set_cache(cache);

            //

            // Adding to Cache

            let mut received_add = rx_add_udp.try_iter();

            let mut next_value = received_add.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (name, rr) = next_value.unwrap();
                cache.add(name, rr);
                next_value = received_add.next();
            }

            self.set_cache(cache);
            //

            // Check queries for timeout

            for (key, val) in queries_hash_by_id.clone() {
                let mut query = val.clone();

                let timeout = query.get_timeout();
                let last_query_timestamp = query.get_last_query_timestamp();
                let now = Utc::now();
                let timestamp_ms = now.timestamp_millis() as u64;

                if timestamp_ms > (timeout as u64 + last_query_timestamp) {
                    query.step_3_udp(socket.try_clone().unwrap());
                }
            }

            ////////////////////////////////////////////////////////////////////

            let mut resolver = self.clone();

            println!("{}", "Message parsed");

            // We get the msg type, it can be query or answer
            let msg_type = dns_message.get_header().get_qr();

            println!("Msg type: {}", msg_type.clone());

            let tx_add_udp_copy = tx_add_udp.clone();
            let tx_delete_udp_copy = tx_delete_udp.clone();
            let tx_add_tcp_copy = tx_add_tcp.clone();
            let tx_delete_tcp_copy = tx_delete_tcp.clone();
            let tx_add_ns_udp_copy = tx_add_ns_udp.clone();
            let tx_delete_ns_udp_copy = tx_delete_ns_udp.clone();
            let tx_add_ns_tcp_copy = tx_add_ns_tcp.clone();
            let tx_delete_ns_tcp_copy = tx_delete_ns_tcp.clone();

            let tx_update_query_copy = tx_update_query.clone();
            let tx_delete_query_copy = tx_delete_query.clone();

            let tx_update_cache_udp_copy = tx_update_cache_udp.clone();
            let tx_update_cache_tcp_copy = tx_update_cache_tcp.clone();
            let tx_update_cache_ns_udp_copy = tx_update_cache_ns_udp.clone();
            let tx_update_cache_ns_tcp_copy = tx_update_cache_ns_tcp.clone();

            let src_address_copy = src_address.clone();

            // If it is query
            if msg_type == false {
                let sname = dns_message.get_question().get_qname().get_name();
                let stype = dns_message.get_question().get_qtype();
                let sclass = dns_message.get_question().get_qclass();
                let op_code = dns_message.get_header().get_op_code();
                let rd = dns_message.get_header().get_rd();
                let id = dns_message.get_query_id();

                let mut resolver_query = ResolverQuery::new(
                    tx_add_udp_copy,
                    tx_delete_udp_copy,
                    tx_add_tcp_copy,
                    tx_delete_tcp_copy,
                    tx_add_ns_udp_copy,
                    tx_delete_ns_udp_copy,
                    tx_add_ns_tcp_copy,
                    tx_delete_ns_tcp_copy,
                    tx_update_query_copy,
                    tx_delete_query_copy,
                    dns_message.clone(),
                    tx_update_cache_udp_copy.clone(),
                    tx_update_cache_tcp_copy.clone(),
                    tx_update_cache_ns_udp_copy.clone(),
                    tx_update_cache_ns_tcp_copy.clone(),
                );

                // Initializes the query data struct
                resolver_query.initialize(
                    sname,
                    stype,
                    sclass,
                    op_code,
                    rd,
                    resolver.get_sbelt(),
                    resolver.get_cache(),
                    resolver.get_ns_data(),
                    src_address.clone().to_string(),
                    id,
                );

                // Save the query info
                queries_hash_by_id
                    .insert(resolver_query.get_main_query_id(), resolver_query.clone());

                // Get copies from some data
                let socket_copy = socket.try_clone().unwrap();
                let dns_msg_copy = dns_message.clone();

                thread::spawn(move || {
                    let answer_local = resolver_query.step_1_udp(socket_copy.try_clone().unwrap());

                    match answer_local {
                        Some(val) => {
                            println!("Local info!");

                            let mut new_dns_msg = dns_msg_copy.clone();
                            new_dns_msg.set_answer(val.clone());
                            new_dns_msg.set_authority(Vec::new());
                            new_dns_msg.set_additional(Vec::new());

                            let mut header = new_dns_msg.get_header();
                            header.set_ancount(val.len() as u16);
                            header.set_nscount(0);
                            header.set_arcount(0);
                            header.set_id(resolver_query.get_old_id());
                            header.set_qr(true);

                            new_dns_msg.set_header(header);

                            Resolver::send_answer_by_udp(
                                new_dns_msg,
                                src_address.clone().to_string(),
                                &socket_copy,
                            );
                        }
                        None => {}
                    }

                    println!("{}", "Thread Finished")
                });
            }

            if msg_type == true {
                let socket_copy = socket.try_clone().unwrap();
                let answer_id = dns_message.get_query_id();
                let queries_hash_by_id_copy = queries_hash_by_id.clone();

                println!("Response ID: {}", answer_id);

                println!(
                    "AA: {}, NS: {}, AD: {}",
                    dns_message.get_answer().len(),
                    dns_message.get_authority().len(),
                    dns_message.get_additional().len()
                );

                if queries_hash_by_id_copy.contains_key(&answer_id) {
                    println!("Message answer ID checked");

                    let tx_query_delete_clone = tx_delete_query.clone();

                    thread::spawn(move || {
                        let mut resolver_query =
                            queries_hash_by_id_copy.get(&answer_id).unwrap().clone();

                        let last_query_timestamp = resolver_query.get_last_query_timestamp();
                        let now = Utc::now();
                        let timestamp_ms = now.timestamp_millis() as u64;

                        let response_time: u32 = (timestamp_ms - last_query_timestamp) as u32;

                        // Send request to update cache to resolver and name server
                        tx_update_cache_udp_copy.send((
                            resolver_query.get_last_query_hostname(),
                            src_address_copy.clone(),
                            response_time,
                        ));

                        tx_update_cache_tcp_copy.send((
                            resolver_query.get_last_query_hostname(),
                            src_address_copy.clone(),
                            response_time,
                        ));

                        tx_update_cache_ns_udp_copy.send((
                            resolver_query.get_last_query_hostname(),
                            src_address_copy.clone(),
                            response_time,
                        ));

                        tx_update_cache_ns_tcp_copy.send((
                            resolver_query.get_last_query_hostname(),
                            src_address_copy.clone(),
                            response_time,
                        ));
                        //

                        resolver_query.set_cache(resolver.get_cache());
                        let response = match resolver_query
                            .clone()
                            .step_4_udp(dns_message, socket_copy.try_clone().unwrap())
                        {
                            Some(val) => {
                                let mut msg = val.clone();
                                let mut header = msg.get_header();
                                let old_id = resolver_query.get_old_id();
                                let answer = msg.get_answer();
                                let authority = msg.get_authority();
                                let additional = msg.get_additional();

                                header.set_id(old_id);
                                header.set_ancount(answer.len() as u16);
                                header.set_nscount(authority.len() as u16);
                                header.set_arcount(additional.len() as u16);
                                msg.set_header(header);

                                tx_query_delete_clone.send(resolver_query.clone());

                                Resolver::send_answer_by_udp(
                                    msg,
                                    resolver_query.get_src_address(),
                                    &socket_copy,
                                );
                            }
                            None => {}
                        };
                    });
                }
            }
        }
    }

    // Runs a tcp resolver
    pub fn run_resolver_tcp(
        &mut self,
        rx_add_tcp: Receiver<(String, ResourceRecord)>,
        rx_delete_tcp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_tcp: Receiver<(String, String, u32)>,
    ) {
        // Vector to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();

        // Channels to send data between threads, resolvers and name server
        let tx_add_udp = self.get_add_sender_udp();
        let tx_delete_udp = self.get_delete_sender_udp();
        let tx_add_tcp = self.get_add_sender_tcp();
        let tx_delete_tcp = self.get_delete_sender_tcp();
        let tx_add_ns_udp = self.get_add_sender_ns_udp();
        let tx_delete_ns_udp = self.get_delete_sender_ns_udp();
        let tx_add_ns_tcp = self.get_add_sender_ns_tcp();
        let tx_delete_ns_tcp = self.get_delete_sender_ns_tcp();
        let tx_update_cache_udp = self.get_update_cache_udp();
        let tx_update_cache_tcp = self.get_update_cache_tcp();
        let tx_update_cache_ns_udp = self.get_update_cache_ns_udp();
        let tx_update_cache_ns_tcp = self.get_update_cache_ns_tcp();

        // Channel to delete queries ids from queries already response
        let (tx_delete_query, rx_delete_query) = mpsc::channel();

        // Channel to update resolver queries from queries in progress
        let (tx_update_query, rx_update_query) = mpsc::channel();

        // Gets ip and port str
        let mut host_address_and_port = self.get_ip_address();

        // Creates a TCP Listener
        let listener = TcpListener::bind(&host_address_and_port).expect("Could not bind");
        println!("{}", "TcpListener Created");

        // Receives messages
        loop {
            println!("{}", "Waiting msg");

            match listener.accept() {
                Ok((mut stream, src_address)) => {
                    // Delete from cache

                    let mut received_delete = rx_delete_tcp.try_iter();

                    let mut next_value = received_delete.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (name, rr) = next_value.unwrap();
                        let rr_type = rr.get_string_type();
                        cache.remove(name, rr_type);
                        next_value = received_delete.next();
                    }

                    self.set_cache(cache);

                    //

                    // Update response time cache

                    let mut received_update = rx_update_cache_tcp.try_iter();

                    let mut next_value = received_update.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (host_name, address, response_time) = next_value.unwrap();
                        cache.update_response_time(
                            host_name,
                            "A".to_string(),
                            response_time,
                            address,
                        );
                        next_value = received_update.next();
                    }

                    self.set_cache(cache);

                    //

                    // Adding to Cache

                    let mut received_add = rx_add_tcp.try_iter();

                    let mut next_value = received_add.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (name, rr) = next_value.unwrap();
                        cache.add(name, rr);
                        next_value = received_add.next();
                    }

                    self.set_cache(cache);

                    ////////////////////////////////////////////////////////////////////

                    println!("New connection: {}", stream.peer_addr().unwrap());

                    // We receive the msg
                    let received_msg =
                        Resolver::receive_tcp_msg(stream.try_clone().unwrap()).unwrap();

                    println!("{}", "Message recv");

                    let tx_add_udp_copy = tx_add_udp.clone();
                    let tx_delete_udp_copy = tx_delete_udp.clone();
                    let tx_add_tcp_copy = tx_add_tcp.clone();
                    let tx_delete_tcp_copy = tx_delete_tcp.clone();
                    let tx_add_ns_udp_copy = tx_add_ns_udp.clone();
                    let tx_delete_ns_udp_copy = tx_delete_ns_udp.clone();
                    let tx_add_ns_tcp_copy = tx_add_ns_tcp.clone();
                    let tx_delete_ns_tcp_copy = tx_delete_ns_tcp.clone();

                    let tx_update_cache_udp_copy = tx_update_cache_udp.clone();
                    let tx_update_cache_tcp_copy = tx_update_cache_tcp.clone();
                    let tx_update_cache_ns_udp_copy = tx_update_cache_ns_udp.clone();
                    let tx_update_cache_ns_tcp_copy = tx_update_cache_ns_tcp.clone();

                    let tx_update_query_copy = tx_update_query.clone();
                    let tx_delete_query_copy = tx_delete_query.clone();

                    let resolver = self.clone();

                    thread::spawn(move || {
                        let dns_message = DnsMessage::from_bytes(&received_msg);

                        println!("{}", "Query message parsed");

                        // We get the msg type, it can be query or answer
                        let msg_type = dns_message.get_header().get_qr();

                        if msg_type == false {
                            let sname = dns_message.get_question().get_qname().get_name();
                            let stype = dns_message.get_question().get_qtype();
                            let sclass = dns_message.get_question().get_qclass();
                            let op_code = dns_message.get_header().get_op_code();
                            let rd = dns_message.get_header().get_rd();
                            let id = dns_message.get_query_id();

                            let mut resolver_query = ResolverQuery::new(
                                tx_add_udp_copy,
                                tx_delete_udp_copy,
                                tx_add_tcp_copy,
                                tx_delete_tcp_copy,
                                tx_add_ns_udp_copy,
                                tx_delete_ns_udp_copy,
                                tx_add_ns_tcp_copy,
                                tx_delete_ns_tcp_copy,
                                tx_update_query_copy,
                                tx_delete_query_copy,
                                dns_message.clone(),
                                tx_update_cache_udp_copy,
                                tx_update_cache_tcp_copy,
                                tx_update_cache_ns_udp_copy,
                                tx_update_cache_ns_tcp_copy,
                            );

                            // Initializes the query data struct
                            resolver_query.initialize(
                                sname,
                                stype,
                                sclass,
                                op_code,
                                rd,
                                resolver.get_sbelt(),
                                resolver.get_cache(),
                                resolver.get_ns_data(),
                                src_address.clone().to_string(),
                                id,
                            );

                            let mut answer_msg = resolver_query.step_1_tcp(dns_message);

                            answer_msg.set_query_id(resolver_query.get_old_id());

                            Resolver::send_answer_by_tcp(
                                answer_msg,
                                stream.peer_addr().unwrap().to_string(),
                                stream,
                            );

                            println!("{}", "Thread Finished")
                        }
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

// Utils
impl Resolver {
    pub fn receive_udp_msg(
        mut socket: UdpSocket,
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

        let dns_msg_parsed = DnsMessage::from_bytes(&msg);

        let query_id = dns_msg_parsed.get_query_id();
        let trunc = dns_msg_parsed.get_header().get_tc();

        match messages.get(&query_id) {
            Some(mut val) => {
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
            let number_of_bytes_msg = stream.read(&mut msg).expect("No data received");
            tcp_msg_len = tcp_msg_len - number_of_bytes_msg as u16;
            vec_msg.append(&mut msg.to_vec());
        }

        return Some(vec_msg);
    }

    // Sends the response to the address by udp
    fn send_answer_by_udp(response: DnsMessage, src_address: String, socket: &UdpSocket) {
        let bytes = response.to_bytes();

        if bytes.len() <= 512 {
            socket
                .send_to(&bytes, src_address)
                .expect("failed to send message");
        } else {
            let ancount = response.get_header().get_ancount();
            let nscount = response.get_header().get_nscount();
            let arcount = response.get_header().get_arcount();
            let total_rrs = ancount + nscount + arcount;
            let half_rrs: f32 = (total_rrs / 2).into();
            let ceil_half_rrs = half_rrs.ceil() as u32;

            let mut answer = response.get_answer();
            let mut authority = response.get_authority();
            let mut additional = response.get_additional();

            let mut first_tc_msg = DnsMessage::new();
            let mut new_header = response.get_header();
            new_header.set_tc(true);
            first_tc_msg.set_header(new_header);

            for i in 1..ceil_half_rrs + 1 {
                if answer.len() > 0 {
                    let rr = answer.remove(0);
                    first_tc_msg.add_answers(vec![rr]);
                } else if authority.len() > 0 {
                    let rr = authority.remove(0);
                    first_tc_msg.add_authorities(vec![rr]);
                } else if additional.len() > 0 {
                    let rr = additional.remove(0);
                    first_tc_msg.add_additionals(vec![rr]);
                } else {
                    continue;
                }
            }

            first_tc_msg.update_header_counters();

            Resolver::send_answer_by_udp(
                first_tc_msg,
                src_address.clone(),
                &socket.try_clone().unwrap(),
            );

            let mut second_tc_msg = DnsMessage::new();
            let mut new_header = response.get_header();
            second_tc_msg.set_header(new_header);

            for i in 1..ceil_half_rrs + 1 {
                if answer.len() > 0 {
                    let rr = answer.remove(0);
                    second_tc_msg.add_answers(vec![rr]);
                } else if authority.len() > 0 {
                    let rr = authority.remove(0);
                    second_tc_msg.add_authorities(vec![rr]);
                } else if additional.len() > 0 {
                    let rr = additional.remove(0);
                    second_tc_msg.add_additionals(vec![rr]);
                } else {
                    continue;
                }
            }

            second_tc_msg.update_header_counters();

            Resolver::send_answer_by_udp(second_tc_msg, src_address, socket);
        }
    }

    // Sends the response to the address by tcp
    fn send_answer_by_tcp(response: DnsMessage, src_address: String, mut stream: TcpStream) {
        let bytes = response.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        stream.write(&full_msg);
    }
}

// Getters
impl Resolver {
    // Gets the ip address
    pub fn get_ip_address(&self) -> String {
        self.ip_address.clone()
    }

    // Gets the list of default servers to ask
    pub fn get_sbelt(&self) -> Slist {
        self.sbelt.clone()
    }

    // Gets the cache
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }

    // Gets the ns_data
    pub fn get_ns_data(&self) -> HashMap<String, NSZone> {
        self.ns_data.clone()
    }

    /// Get the owner's query address
    pub fn get_add_sender_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_sender_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_sender_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_sender_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_sender_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_sender_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_sender_ns_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_sender_ns_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_sender_ns_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_sender_ns_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_sender_ns_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_ns_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_sender_ns_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_ns_tcp.clone()
    }

    /// Gets the sender for updating cache
    pub fn get_update_cache_udp(&self) -> Sender<(String, String, u32)> {
        self.update_cache_sender_udp.clone()
    }

    /// Gets the sender for updating cache
    pub fn get_update_cache_tcp(&self) -> Sender<(String, String, u32)> {
        self.update_cache_sender_tcp.clone()
    }

    /// Gets the sender for updating cache
    pub fn get_update_cache_ns_udp(&self) -> Sender<(String, String, u32)> {
        self.update_cache_sender_ns_udp.clone()
    }

    /// Gets the sender for updating cache
    pub fn get_update_cache_ns_tcp(&self) -> Sender<(String, String, u32)> {
        self.update_cache_sender_ns_tcp.clone()
    }
}

//Setters
impl Resolver {
    // Sets the ip address attribute with a value
    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = ip_address;
    }

    // Sets the sbelt attribute with a value
    pub fn set_sbelt(&mut self, sbelt: Slist) {
        self.sbelt = sbelt;
    }

    // Sets the cache attribute with a value
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }

    // Sets the ns_data attribute with a new value
    pub fn set_ns_data(&mut self, ns_data: HashMap<String, NSZone>) {
        self.ns_data = ns_data;
    }
}

mod test {
    use crate::dns_cache::DnsCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;
    use crate::resolver::resolver_query::ResolverQuery;
    use crate::resolver::slist::Slist;
    use crate::resolver::Resolver;
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::vec::Vec;

    #[test]
    fn constructor_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
            tx_update_cache_ns_udp,
            tx_update_cache_ns_tcp,
        );

        assert_eq!(resolver.ip_address, "".to_string());
        assert_eq!(resolver.sbelt.get_ns_list().len(), 0);
        assert_eq!(resolver.cache.get_size(), 0);
    }

    #[test]
    fn set_and_get_ip_address() {
        /// Channels
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
            tx_update_cache_ns_udp,
            tx_update_cache_ns_tcp,
        );

        assert_eq!(resolver.get_ip_address(), "".to_string());

        resolver.set_ip_address("127.0.0.1".to_string());

        assert_eq!(resolver.get_ip_address(), "127.0.0.1".to_string());
    }

    #[test]
    fn set_and_get_sbelt() {
        /// Channels
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
            tx_update_cache_ns_udp,
            tx_update_cache_ns_tcp,
        );
        let mut sbelt_test = Slist::new();

        sbelt_test.insert("test.com".to_string(), "127.0.0.1".to_string(), 5000);

        resolver.set_sbelt(sbelt_test);

        assert_eq!(resolver.get_sbelt().get_ns_list().len(), 1);
    }

    #[test]
    fn set_and_get_cache() {
        /// Channels
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
            tx_update_cache_ns_udp,
            tx_update_cache_ns_tcp,
        );

        let mut cache_test = DnsCache::new();
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        cache_test.set_max_size(1);

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);

        cache_test.add("127.0.0.0".to_string(), resource_record);

        resolver.set_cache(cache_test);

        assert_eq!(resolver.get_cache().get_size(), 1);
    }

    #[test]
    fn set_and_get_ns_data_test() {
        let mut domain_name = DomainName::new();
        domain_name.set_name("test2.com".to_string());

        let mut ns_rdata = NsRdata::new();
        ns_rdata.set_nsdname(domain_name);

        let r_data = Rdata::SomeNsRdata(ns_rdata);
        let mut ns_resource_record = ResourceRecord::new(r_data);
        ns_resource_record.set_type_code(2);

        let mut resource_record_vec = Vec::<ResourceRecord>::new();

        resource_record_vec.push(ns_resource_record);

        let mut host_names_hash = HashMap::<String, Vec<ResourceRecord>>::new();

        host_names_hash.insert("test.com".to_string(), resource_record_vec);

        let mut rr_type_hash = HashMap::<String, HashMap<String, Vec<ResourceRecord>>>::new();

        rr_type_hash.insert("NS".to_string(), host_names_hash);

        /// Channels
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();

        let (tx_update_query, rx_update_query) = mpsc::channel();
        let (tx_delete_query, rx_delete_query) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let mut resolver_query_test = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
            tx_update_query,
            tx_delete_query,
            DnsMessage::new(),
            tx_update_cache_udp,
            tx_update_cache_tcp,
            tx_update_cache_ns_udp,
            tx_update_cache_ns_tcp,
        );

        assert_eq!(resolver_query_test.get_ns_data().len(), 0);

        //resolver_query_test.set_ns_data(rr_type_hash);

        //assert_eq!(resolver_query_test.get_ns_data().len(), 1);
    }
}
