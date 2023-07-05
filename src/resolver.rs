use crate::dns_cache::DnsCache;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::resolver::resolver_query::ResolverQuery;
use crate::resolver::slist::Slist;

use chrono::Utc;
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
// Struct that represents a dns resolver
pub struct Resolver {
    /// Ip address and port where the resolver will run
    ip_address: String,
    // Struct that contains a default server list to ask
    sbelt: Slist,
    // Cache for the resolver
    cache: DnsCache,
    // Channel to share cache data between threads
    tx_add_cache_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    tx_delete_cache_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    tx_add_cache_tcp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    tx_delete_cache_tcp: Sender<(String, ResourceRecord)>,
    // Channel to update response time in cache data in name server and resolver
    tx_update_cache_time_udp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    tx_update_cache_time_tcp: Sender<(String, String, u32)>,
}

impl Resolver {
    // Creates a new Resolver with default values
    pub fn new(
        tx_add_cache_udp: Sender<(String, ResourceRecord)>,
        tx_delete_cache_udp: Sender<(String, ResourceRecord)>,
        tx_add_cache_tcp: Sender<(String, ResourceRecord)>,
        tx_delete_cache_tcp: Sender<(String, ResourceRecord)>,
        tx_update_cache_time_udp: Sender<(String, String, u32)>,
        tx_update_cache_time_tcp: Sender<(String, String, u32)>,
    ) -> Self {
        let mut cache = DnsCache::new();
        cache.set_max_size(1000);

        let resolver = Resolver {
            ip_address: String::from(""),
            sbelt: Slist::new(),
            cache: cache,
            tx_add_cache_udp: tx_add_cache_udp,
            tx_delete_cache_udp: tx_delete_cache_udp,
            tx_add_cache_tcp: tx_add_cache_tcp,
            tx_delete_cache_tcp: tx_delete_cache_tcp,
            tx_update_cache_time_udp: tx_update_cache_time_udp,
            tx_update_cache_time_tcp: tx_update_cache_time_tcp,
        };

        resolver
    }

    /// Sets the initial IP, PORT and SBELT values.
    pub fn set_initial_configuration(&mut self, resolver_ip_port: &str, sbelt_root_ips: &'static [&'static str]) {
        self.set_ip_address(resolver_ip_port.to_string());

        //set sbelt
        let mut sbelt = Slist::new();
        for &ip in sbelt_root_ips {
            sbelt.insert(".".to_string(), ip.to_string(), 5000);
        }
        self.set_sbelt(sbelt);
    }

    pub fn run_resolver(
        &mut self,
        rx_add_cache_udp: Receiver<(String, ResourceRecord)>,
        rx_delete_cache_udp: Receiver<(String, ResourceRecord)>,
        rx_add_cache_tcp: Receiver<(String, ResourceRecord)>,
        rx_delete_cache_tcp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_time_udp: Receiver<(String, String, u32)>,
        rx_update_cache_time_tcp: Receiver<(String, String, u32)>,
    ) {
        let mut resolver_copy = self.clone();
        thread::spawn(move || {
            resolver_copy.run_resolver_udp(
                rx_add_cache_udp,
                rx_delete_cache_udp,
                rx_update_cache_time_udp
            );
        });

        self.run_resolver_tcp(
            rx_add_cache_tcp,
            rx_delete_cache_tcp,
            rx_update_cache_time_tcp
        );
    }

    // Runs a udp resolver
    fn run_resolver_udp(
        &mut self,
        rx_add_cache_udp: Receiver<(String, ResourceRecord)>,
        rx_delete_cache_udp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_time_udp: Receiver<(String, String, u32)>
    ) {
        // Hashmap to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();

        // Hashmap to save incomplete messages
        let messages = HashMap::<u16, DnsMessage>::new();

        // Channels to send cache data between threads, resolvers and name server
        let tx_update_cache_udp = self.get_tx_update_cache_time_udp();
        let tx_update_cache_tcp = self.get_tx_update_cache_time_tcp();

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
            let (dns_message, src_address) = self.receive_udp_msg_value(
                socket.try_clone().unwrap(), 
                messages.clone()
            );
            self.update_queries(&rx_update_query, &mut queries_hash_by_id);
            self.delete_answered_queries(&rx_delete_query, &mut queries_hash_by_id);
            self.delete_from_cache(&rx_delete_cache_udp);
            self.update_cache_response_time_udp(&rx_update_cache_time_udp);
            self.add_to_cache_upd(&rx_add_cache_udp);
            self.check_queries_timeout(queries_hash_by_id.clone(), socket.try_clone().unwrap());

            let resolver = self.clone();

            println!("{}", "Message parsed");

            // Message type: QR field in Header that specifies whether this message is a 
            // query (0), or a response/answer (1).
            let msg_type = dns_message.get_header().get_qr();

            println!("Msg type: {}", msg_type.clone());

            let tx_update_query_copy = tx_update_query.clone();
            let tx_delete_query_copy = tx_delete_query.clone();

            let tx_update_cache_udp_copy = tx_update_cache_udp.clone();
            let tx_update_cache_tcp_copy = tx_update_cache_tcp.clone();

            let src_address_copy = src_address.clone();

            // If it is query
            if msg_type == false {
                let (
                    mut resolver_query, 
                    _rx_update_slist_tcp, 
                    rx_update_self_slist
                ) = self.new_query_from_msg(
                    dns_message.clone(), 
                    src_address.clone().to_string(), 
                    tx_update_query_copy, 
                    tx_delete_query_copy.clone());

                // Save the query info
                queries_hash_by_id
                    .insert(resolver_query.get_main_query_id(), resolver_query.clone());

                // Get copies from some data
                let socket_copy = socket.try_clone().unwrap();
                let dns_msg_copy = dns_message.clone();
                let tx_query_delete_clone = tx_delete_query_copy.clone();

                thread::spawn(move || {
                    let answer_local = resolver_query
                        .step_1_udp(socket_copy.try_clone().unwrap(), rx_update_self_slist);

                    match answer_local {
                        (Some(val), None) => {
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

                            tx_query_delete_clone
                                .send(resolver_query.clone())
                                .expect("Couldn't send the resolver query through the channel");

                            Resolver::send_answer_by_udp(
                                new_dns_msg,
                                src_address.clone().to_string(),
                                &socket_copy,
                            );
                        }
                        (None, Some(msg)) => {
                            tx_query_delete_clone
                                .send(resolver_query.clone())
                                .expect("Couldn't send the resolver query through the channel");
                            Resolver::send_answer_by_udp(
                                msg,
                                src_address.clone().to_string(),
                                &socket_copy,
                            );
                        }
                        (_, _) => {}
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
                    "qname: {}",
                    dns_message.get_question().get_qname().get_name()
                );
                println!(
                    "AA: {}, NS: {}, AD: {}",
                    dns_message.get_answer().len(),
                    dns_message.get_authority().len(),
                    dns_message.get_additional().len()
                );

                if queries_hash_by_id_copy.contains_key(&answer_id) {
                    println!("Message answer ID checked");

                    let tx_query_delete_clone = tx_delete_query.clone();
                    let tx_query_update_clone = tx_update_query.clone();

                    thread::spawn(move || {
                        let mut resolver_query =
                            queries_hash_by_id_copy.get(&answer_id).unwrap().clone();

                        let last_query_timestamp = resolver_query.get_last_query_timestamp();
                        let now = Utc::now();
                        let timestamp_ms = now.timestamp_millis() as u64;

                        let response_time: u32 = (timestamp_ms - last_query_timestamp) as u32;

                        // Send request to update cache to resolver and name server
                        tx_update_cache_udp_copy
                            .send((
                                resolver_query.get_last_query_hostname(),
                                src_address_copy.clone(),
                                response_time,
                            ))
                            .expect("Couldn't send request using UDP to update cache to resolver");

                        tx_update_cache_tcp_copy
                            .send((
                                resolver_query.get_last_query_hostname(),
                                src_address_copy.clone(),
                                response_time,
                            ))
                            .expect("Couldn't send request using TCP to update cache to resolver");

                        resolver_query.set_cache(resolver.get_cache());

                        let (tx_update_self_slist, rx_update_self_slist) = mpsc::channel();

                        resolver_query.set_tx_update_self_slist(tx_update_self_slist);

                        let _response = match resolver_query.clone().step_4_udp(
                            dns_message,
                            socket_copy.try_clone().unwrap(),
                            rx_update_self_slist,
                        ) {
                            Some(val) => {
                                let is_internal_query = resolver_query.get_internal_query();

                                if is_internal_query == false {
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

                                    tx_query_delete_clone.send(resolver_query.clone()).expect(
                                        "Couldn't send the resolver query through the channel",
                                    );

                                    Resolver::send_answer_by_udp(
                                        msg,
                                        resolver_query.get_src_address(),
                                        &socket_copy,
                                    );
                                } else {
                                    let msg = val.clone();
                                    let answers = msg.get_answer();
                                    let host_name = answers[0].clone().get_name().get_name();
                                    let resolver_query_id_to_update =
                                        resolver_query.get_query_id_update_slist();

                                    let resolver_query_to_update_result =
                                        queries_hash_by_id_copy.get(&resolver_query_id_to_update);

                                    match resolver_query_to_update_result {
                                        Some(val) => {
                                            let mut resolver_query_to_update = val.clone();
                                            let mut slist_to_update =
                                                resolver_query_to_update.get_slist();
                                            let mut ns_list_to_update =
                                                slist_to_update.get_ns_list();
                                            let mut ns_index = 0;

                                            for ns in ns_list_to_update.clone() {
                                                let answers_copy = answers.clone();
                                                let ns_name = ns
                                                    .get(&"name".to_string())
                                                    .unwrap()
                                                    .to_string();

                                                println!(
                                                    "ns name: {} - host name: {}",
                                                    ns_name.clone(),
                                                    host_name.clone()
                                                );

                                                if ns_name == host_name {
                                                    ns_list_to_update.remove(ns_index);

                                                    for answer in answers_copy {
                                                        let ip = match answer.get_rdata() {
                                                            Rdata::SomeARdata(val) => {
                                                                val.get_string_address()
                                                            }
                                                            _ => unreachable!(),
                                                        };

                                                        let mut new_ns_to_ask = HashMap::new();

                                                        new_ns_to_ask.insert(
                                                            "name".to_string(),
                                                            host_name.clone(),
                                                        );
                                                        new_ns_to_ask
                                                            .insert("ip_address".to_string(), ip);
                                                        new_ns_to_ask.insert(
                                                            "response_time".to_string(),
                                                            "5000".to_string(),
                                                        );

                                                        ns_list_to_update.push(new_ns_to_ask);
                                                    }
                                                }
                                                ns_index = ns_index + 1;
                                            }

                                            slist_to_update.set_ns_list(ns_list_to_update);
                                            resolver_query_to_update
                                                .set_slist(slist_to_update.clone());

                                            resolver_query_to_update
                                                .get_tx_update_self_slist()
                                                .send(slist_to_update)
                                                .expect("Couldn't send the slist to update through the channel");
                                            tx_query_update_clone.send(resolver_query_to_update)
                                                                 .expect("Couldn't send the resolver query through the channel");
                                            tx_query_delete_clone.send(resolver_query.clone())
                                                                 .expect("Couldn't send the resolver query through the channel");
                                        }
                                        None => {}
                                    }
                                }
                            }
                            None => {}
                        };
                    });
                }
            }
        }
    }

    /// Retrieves DNS messages received from an UDP socket and their respective origin addresses.
    ///
    /// Given an UDP socket and a HashMap of DNS messages, this function returns the DNS messages
    /// that were received along with their address of origin.
    /// 
    /// In case of a format error in the DNS message, a response is sent.
    fn receive_udp_msg_value(
        &mut self, 
        socket: UdpSocket, 
        messages: HashMap<u16, DnsMessage>) -> (DnsMessage, String) {
        println!("{}", "Waiting msg");

        // We receive the msg
        let dns_message_option =
            Resolver::receive_udp_msg(socket.try_clone().unwrap(), messages.clone());
        let dns_message;
        let src_address;

        println!("{}", "Message recv");

        match dns_message_option {
            Some(val) => {
                dns_message = val.0;
                src_address = val.1;
            }
            None => {
                (dns_message, src_address) = (DnsMessage::new(), "".to_string());
            }
        }

        // Format Error
        if dns_message.get_header().get_rcode() == 1 {
            Resolver::send_answer_by_udp(
                dns_message.clone(),
                src_address.clone(),
                &socket.try_clone().unwrap(),
            );
        }
        return (dns_message, src_address);
    }

    /// Updates the queries in the resolver.
    ///
    /// Given a references to a Receiver with the queries to update and a HashMap which saves 
    /// the queries in process, this function iterates over each query that needs to be 
    /// updated and updates the corresponding entries in the `queries_hash_by_id` HashMap.
    fn update_queries(
        &mut self, 
        rx_update_query: & Receiver<ResolverQuery>, 
        queries_hash_by_id: &mut HashMap<u16, ResolverQuery>) {
        // Iterate in each query which needs to be updated
        let mut queries_to_update = rx_update_query.try_iter();
        let mut next_query_to_update = queries_to_update.next();

        println!("Queries before update len: {}", queries_hash_by_id.len());

        while next_query_to_update.is_none() == false {
            println!("Queries to update");
            let resolver_query_to_update = next_query_to_update.unwrap();
            let id: u16 = resolver_query_to_update.get_main_query_id();
            println!("Queries to update: {}", id);
            queries_hash_by_id.insert(id, resolver_query_to_update);

            next_query_to_update = queries_to_update.next();
        }
        println!("Queries len: {}", queries_hash_by_id.len());
    }

    /// Deletes the queries that have been answered from the HashMap of queries.
    ///
    /// Given a reference to a Receiver of `ResolverQuery`(`rx_delete_query`), and a mutable 
    /// reference to a HashMap of queries (`queries_hash_by_id`), this method iterates over 
    /// the received queries and removes the ones that have been answered from the HashMap 
    /// based on their ID.
    fn delete_answered_queries(&mut self, 
        rx_delete_query: & Receiver<ResolverQuery>, 
        queries_hash_by_id: &mut HashMap<u16, ResolverQuery>) {
        // Delete queries already answered
        let mut queries_to_delete = rx_delete_query.try_iter();
        let mut next_query_value = queries_to_delete.next();

        while next_query_value.is_none() == false {
            let resolver_query_to_delete = next_query_value.unwrap();
            let id: u16 = resolver_query_to_delete.get_main_query_id();
            println!("Queries to delete: {}", id);
            queries_hash_by_id.remove(&id);

            next_query_value = queries_to_delete.next();
        }
        println!("Queries length after delete: {}", queries_hash_by_id.len());
    }

    /// Deletes entries from the cache based on received Resouce Record.
    ///
    /// This function takes a reference to a Receiver of tuples `(String, ResourceRecord)`. 
    /// It iterates over the received queries (UDP messages) and removes the 
    /// corresponding entries from the cache.
    fn delete_from_cache(
        &mut self, 
        rx_delete_cache_udp: & Receiver<(String, ResourceRecord)>) {
        let mut received_delete = rx_delete_cache_udp.try_iter();
        let mut next_value = received_delete.next();
        let mut cache = self.get_cache();

        while next_value.is_none() == false {
            let (name, rr) = next_value.unwrap();
            let rr_type = rr.get_string_type();
            cache.remove(name, rr_type);
            next_value = received_delete.next();
        }
        self.set_cache(cache);
    }

    /// Updates the response time in the cache for the given host names received over UDP.
    ///
    /// This function takes a reference to a Receiver of tuples `(String, String, u32)` 
    /// (`rx_update_cache_time_udp`). It iterates over the received host names, addresses, 
    /// and response times and updates the corresponding entries in the cache.
    fn update_cache_response_time_udp(
        &mut self, 
        rx_update_cache_time_udp: & Receiver<(String, String, u32)>) {
        let mut received_update = rx_update_cache_time_udp.try_iter();
        let mut next_value = received_update.next();
        let mut cache = self.get_cache();

        while next_value.is_none() == false {
            let (host_name, address, response_time) = next_value.unwrap();
            cache.update_response_time(
                host_name, 
                "A".to_string(), 
                response_time, 
                address);
            next_value = received_update.next();
        }
        self.set_cache(cache);
    }
    
    /// Adds received domain name and Resource Records to the cache from a UDP source.
    ///
    /// This function takes a reference to a Receiver of tuples (String, ResourceRecord)` 
    /// (`rx_add_cache_udp`). It iterates over the received Resource Records and adds them to
    /// the cache.
    fn add_to_cache_upd(
        &mut self, 
        rx_add_cache_udp: & Receiver<(String, ResourceRecord)>) {
        let mut received_add = rx_add_cache_udp.try_iter();
        let mut next_value = received_add.next();
        let mut cache = self.get_cache();

        while next_value.is_none() == false {
            let (name, rr) = next_value.unwrap();
            cache.add(name, rr);
            next_value = received_add.next();
        }
        self.set_cache(cache);
    }

    /// Check queries for a timeout.
    ///
    /// This function iterates over the provided `queries_hash_by_id` HashMap<u16, ResolverQuery> 
    /// (where `u16` corresonds to the query's ID, and `ResolverQuery` contains the corresonding 
    /// query) and checks each query for a possible timeout. If a query has timed out, 
    /// it performs the necessary steps to handle the timeout.
    fn check_queries_timeout(
        &mut self, 
        queries_hash_by_id: HashMap<u16, ResolverQuery>, 
        socket: UdpSocket) {
        for (_key, value) in queries_hash_by_id {
            let mut query = value.clone();

            let timeout = query.get_timeout();
            let last_query_timestamp = query.get_last_query_timestamp();
            let time_now = Utc::now();
            let timestamp_ms = time_now.timestamp_millis() as u64;

            println!("Query to {}", query.get_sname());

            let (_tx_update_self_slist, rx_update_self_slist) = mpsc::channel();

            if timestamp_ms > (timeout as u64 + last_query_timestamp) {
                println!("Timeout!!!!!!!!");
                query.step_3_udp(socket.try_clone().unwrap(), rx_update_self_slist);
            }
        }
    }

    /// Creates a new ResolverQuery based on a DNS message and other parameters.
    /// 
    /// This function takes the Sender channels to update and delete queries (`tx_update_query` 
    /// and `tx_delete_query`) to create a new ResolverQuery. Uses the values provided by 
    /// the `dns_message: DnsMessage` and its source address `src_address` to initialize the query. 
    /// 
    /// Returns the ResolverQuery and the Receiver channels required to update the query's
    ///  SLIST, `rx_update_slist_tcp` and `rx_update_slist_tcp`.
    fn new_query_from_msg(
        &mut self, 
        dns_message: DnsMessage,
        src_address: String,
        tx_update_query: Sender<ResolverQuery>,
        tx_delete_query: Sender<ResolverQuery>) -> (
            ResolverQuery, 
            Receiver<(String, Vec<ResourceRecord>)>, 
            Receiver<Slist>) {
        // DNS message's data
        let sname = dns_message.get_question().get_qname().get_name();
        let stype = dns_message.get_question().get_qtype();
        let sclass = dns_message.get_question().get_qclass();
        let op_code = dns_message.get_header().get_op_code();
        let rd = dns_message.get_header().get_rd();
        let id = dns_message.get_query_id();

        // Channels needed for ResolverQuery
        let (tx_update_slist_tcp, 
            rx_update_slist_tcp) = mpsc::channel();

        let (tx_update_self_slist, 
            rx_update_self_slist) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
            self.get_tx_add_cache_udp(),
            self.get_tx_delete_cache_udp(),
            self.get_tx_add_cache_tcp(),
            self.get_tx_delete_cache_tcp(),
            tx_update_query,
            tx_delete_query,
            dns_message,
            self.get_tx_update_cache_time_udp(),
            self.get_tx_update_cache_time_tcp(),
            tx_update_slist_tcp,
            tx_update_self_slist,
        );

        // Initializes ResolverQuery data struct with message's data
        resolver_query.initialize(
            sname,
            Qtype::from_qtype_to_str(stype).as_str(),
            Qclass::from_qclass_to_str(sclass).as_str(),
            op_code,
            rd,
            self.get_sbelt(),
            self.get_cache(),
            src_address.to_string(),
            id,
        );
        return (resolver_query, rx_update_slist_tcp, rx_update_self_slist);
    }

    /// Matches the answer for the first step of resolution for a given resolver query.
    /// 
    /// This method performs the necessary steps to handle the local answer for a resolver query in the first step of resolution. 
    /// It takes references to the required data structures and performs the following actions:
    /// - Calls the `step_1_udp` method on the `resolver_query` to obtain the local answer.
    /// - Matches the local answer and performs the corresponding actions:
    ///   - If there is a local answer available (`Some(vec_rr)`), it modifies the `dns_message` to include the answer, updates the header information, sends the resolver query for deletion through the `tx_delete_query` channel, and sends the modified `new_dns_msg` as the answer by UDP to `src_address` using the provided `socket`.
    ///   - If there is an error message available (`None` local answer, but `Some(msg)` error message), it sends the resolver query for deletion through the `tx_delete_query` channel and sends the `msg` as the answer by UDP to `src_address` using the provided `socket`.
    ///   - For any other cases, no action is taken.
    /// 
    /// # Arguments
    /// * `resolver_query` - A mutable reference to the ResolverQuery struct representing the query being resolved.
    /// * `socket` - A reference to the UdpSocket used for communication.
    /// * `rx_update_self_slist` - A Receiver for updating the self Slist.
    /// * `tx_delete_query` - A reference to the Sender for deleting the resolver query.
    /// * `dns_message` - A reference to the DnsMessage associated with the resolver query.
    /// * `src_address` - A reference to the source address where the answer will be sent.
    fn match_step_1_answer(
        &mut self, 
        resolver_query: &mut ResolverQuery, 
        socket: & UdpSocket,
        rx_update_self_slist: Receiver<Slist>,
        tx_delete_query: & Sender<ResolverQuery>,
        dns_message: & DnsMessage,
        src_address: & String) {
        let local_answer = resolver_query
        .step_1_udp(socket.try_clone().unwrap(), rx_update_self_slist);

        match local_answer {
            (Some(vec_rr), None) => {
                println!("Local info!");

                let mut new_dns_msg = dns_message.clone();
                new_dns_msg.set_answer(vec_rr.clone());
                new_dns_msg.set_authority(Vec::new());
                new_dns_msg.set_additional(Vec::new());

                let mut header = new_dns_msg.get_header();
                header.set_ancount(vec_rr.len() as u16);
                header.set_nscount(0);
                header.set_arcount(0);
                header.set_id(resolver_query.get_old_id());
                header.set_qr(true);

                new_dns_msg.set_header(header);

                tx_delete_query
                    .send(resolver_query.clone())
                    .expect("Couldn't send the resolver query through the channel");

                Resolver::send_answer_by_udp(
                    new_dns_msg,
                    src_address.clone().to_string(),
                    socket,
                );
            }
            (None, Some(msg)) => {
                tx_delete_query
                    .send(resolver_query.clone())
                    .expect("Couldn't send the resolver query through the channel");
                Resolver::send_answer_by_udp(
                    msg,
                    src_address.clone().to_string(),
                    socket,
                );
            }
            (_, _) => {}
        }
        println!("{}", "Thread Finished")
    }

    // Runs a tcp resolver
    fn run_resolver_tcp(
        &mut self,
        rx_add_cache_tcp: Receiver<(String, ResourceRecord)>,
        rx_delete_cache_tcp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_time_tcp: Receiver<(String, String, u32)>
    ) {
        // Vector to save the queries in process
        // let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();

        // Channels to send data between threads, resolvers and name server
        let tx_add_udp = self.get_tx_add_cache_udp();
        let tx_delete_udp = self.get_tx_delete_cache_udp();
        let tx_add_tcp = self.get_tx_add_cache_tcp();
        let tx_delete_tcp = self.get_tx_delete_cache_tcp();
        let tx_update_cache_udp = self.get_tx_update_cache_time_udp();
        let tx_update_cache_tcp = self.get_tx_update_cache_time_tcp();

        // Channel to delete queries ids from queries already response
        let (tx_delete_query, _rx_delete_query) = mpsc::channel();

        // Channel to update resolver queries from queries in progress
        let (tx_update_query, _rx_update_query) = mpsc::channel();

        // Gets ip and port str
        let host_address_and_port = self.get_ip_address();

        // Creates a TCP Listener
        let listener = TcpListener::bind(&host_address_and_port).expect("Could not bind");
        println!("{}", "TcpListener Created");

        // Receives messages
        loop {
            println!("{}", "Waiting msg");

            match listener.accept() {
                Ok((stream, src_address)) => {
                    
                    // Delete from cache

                    let mut received_delete = rx_delete_cache_tcp.try_iter();

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

                    let mut received_update = rx_update_cache_time_tcp.try_iter();

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

                    let mut received_add = rx_add_cache_tcp.try_iter();

                    let mut next_value = received_add.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (name, rr) = next_value.unwrap();
                        cache.add(name, rr);
                        next_value = received_add.next();
                    }

                    self.set_cache(cache);

                    println!("New connection: {}", stream.peer_addr().unwrap());

                    // We receive the msg
                    let received_msg =
                        Resolver::receive_tcp_msg(stream.try_clone().unwrap()).unwrap();

                    println!("{}", "Message recv");

                    let tx_add_udp_copy = tx_add_udp.clone();
                    let tx_delete_udp_copy = tx_delete_udp.clone();
                    let tx_add_tcp_copy = tx_add_tcp.clone();
                    let tx_delete_tcp_copy = tx_delete_tcp.clone();

                    let tx_update_cache_udp_copy = tx_update_cache_udp.clone();
                    let tx_update_cache_tcp_copy = tx_update_cache_tcp.clone();

                    let tx_update_query_copy = tx_update_query.clone();
                    let tx_delete_query_copy = tx_delete_query.clone();

                    let resolver = self.clone();

                    let dns_message_parse_result = DnsMessage::from_bytes(&received_msg);

                    match dns_message_parse_result {
                        Ok(_) => {}
                        Err(_) => {
                            let dns_format_error_msg = DnsMessage::format_error_msg();

                            Resolver::send_answer_by_tcp(
                                dns_format_error_msg,
                                stream.peer_addr().unwrap().to_string(),
                                stream,
                            );

                            continue;
                        }
                    }

                    thread::spawn(move || {
                        let dns_message = dns_message_parse_result.unwrap();

                        println!("{}", "Query message parsed");

                        // We get the msg type, it can be query or answer
                        let msg_type = dns_message.get_header().get_qr();

                        // if it is a query
                        if msg_type == false {
                            let sname = dns_message.get_question().get_qname().get_name();
                            let stype = dns_message.get_question().get_qtype();
                            let sclass = dns_message.get_question().get_qclass();
                            let op_code = dns_message.get_header().get_op_code();
                            let rd = dns_message.get_header().get_rd();
                            let id = dns_message.get_query_id();

                            let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();
                            let (tx_update_self_slist, _rx_update_self_slist) = mpsc::channel();

                            let mut resolver_query = ResolverQuery::new(
                                tx_add_udp_copy,
                                tx_delete_udp_copy,
                                tx_add_tcp_copy,
                                tx_delete_tcp_copy,
                                tx_update_query_copy,
                                tx_delete_query_copy,
                                dns_message.clone(),
                                tx_update_cache_udp_copy,
                                tx_update_cache_tcp_copy,
                                tx_update_slist_tcp,
                                tx_update_self_slist,
                            );

                            // Initializes the query data struct
                            resolver_query.initialize(
                                sname,
                                Qtype::from_qtype_to_str(stype).as_str(),
                                Qclass::from_qclass_to_str(sclass).as_str(),
                                op_code,
                                rd,
                                resolver.get_sbelt(),
                                resolver.get_cache(),
                                // resolver.get_ns_data(),
                                src_address.clone().to_string(),
                                id,
                            );

                            let mut answer_msg =
                                resolver_query.step_1_tcp(dns_message, rx_update_slist_tcp);

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
        socket: UdpSocket,
        mut messages: HashMap<u16, DnsMessage>,
    ) -> Option<(DnsMessage, String)> {
        let mut msg = [0; 512];
        let (number_of_bytes_msg, address) = match socket.recv_from(&mut msg) {
            Ok((bytes, addr)) => (bytes, addr.to_string()),
            Err(_) => (0, "".to_string()),
        };
        
        println!("msg len: {}", number_of_bytes_msg);

        if number_of_bytes_msg == 0 {
            return None;
        }

        let dns_msg_parsed_result = DnsMessage::from_bytes(&msg);

        match dns_msg_parsed_result {
            Ok(_) => {}
            Err(_) => {
                return Some((DnsMessage::format_error_msg(), address));
            }
        }

        let dns_msg_parsed = dns_msg_parsed_result.unwrap();

        let query_id = dns_msg_parsed.get_query_id();
        //let trunc = dns_msg_parsed.get_header().get_tc();

        let trunc = false;

        println!("Truncado: {}", trunc);

        println!(
            "AA: {} - NS: {} - AD: {}",
            dns_msg_parsed.get_answer().len(),
            dns_msg_parsed.get_authority().len(),
            dns_msg_parsed.get_additional().len()
        );

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

            for _i in 1..ceil_half_rrs + 1 {
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
            let new_header = response.get_header();
            second_tc_msg.set_header(new_header);

            for _i in 1..ceil_half_rrs + 1 {
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
    fn send_answer_by_tcp(response: DnsMessage, _src_address: String, mut stream: TcpStream) {
        let bytes = response.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        stream.write(&full_msg).expect("Couldn't write the message");
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

    // Get the owner's query address
    pub fn get_tx_add_cache_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.tx_add_cache_udp.clone()
    }

    // Get the owner's query address
    pub fn get_tx_add_cache_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.tx_add_cache_tcp.clone()
    }

    // Get the owner's query address
    pub fn get_tx_delete_cache_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.tx_delete_cache_udp.clone()
    }

    // Get the owner's query address
    pub fn get_tx_delete_cache_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.tx_delete_cache_tcp.clone()
    }

    // Gets the sender for updating cache
    pub fn get_tx_update_cache_time_udp(&self) -> Sender<(String, String, u32)> {
        self.tx_update_cache_time_udp.clone()
    }

    // Gets the sender for updating cache
    pub fn get_tx_update_cache_time_tcp(&self) -> Sender<(String, String, u32)> {
        self.tx_update_cache_time_tcp.clone()
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

}

 #[cfg(test)]
mod resolver_test {
    use crate::config::{RESOLVER_IP_PORT, SBELT_ROOT_IPS};
    use crate::dns_cache::DnsCache;
    use crate::message::DnsMessage;
    use crate::message::class_qclass::Qclass;
    use crate::message::type_rtype::Rtype;
    use crate::message::type_qtype::Qtype;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::resolver::slist::Slist;
    use crate::resolver::Resolver;
    use std::collections::HashMap;
    use std::net::UdpSocket;
    use std::sync::mpsc::{self, Sender, Receiver};

    use super::resolver_query::ResolverQuery;

    #[test]
    fn constructor() {
        // Channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        assert_eq!(resolver.ip_address, "".to_string());
        assert_eq!(resolver.sbelt.get_ns_list().len(), 0);
        assert_eq!(resolver.cache.get_size(), 0);
    }

    #[test]
    fn set_and_get_ip_address() {
        //Channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );
        
        assert_eq!(resolver.get_ip_address(), "".to_string());
        resolver.set_ip_address("127.0.0.1".to_string());
        assert_eq!(resolver.get_ip_address(), "127.0.0.1".to_string());
    }

    #[test]
    fn set_and_get_sbelt() {
        // Channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        assert_eq!(resolver.get_sbelt().get_ns_list().len(), 0);

        let mut sbelt = Slist::new();        
        sbelt.insert("test.com".to_string(), 
        "127.0.0.1".to_string(), 
        5000);
        resolver.set_sbelt(sbelt);

        assert_eq!(resolver.get_sbelt().get_ns_list().len(), 1);
    }

    #[test]
    fn set_and_get_cache() {
        // Channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        assert_eq!(resolver.get_cache().get_size(), 0);

        let mut cache_test = DnsCache::new();
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();
        cache_test.set_max_size(1);

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rtype::A);

        cache_test.add("127.0.0.0".to_string(), resource_record);
        resolver.set_cache(cache_test);

        assert_eq!(resolver.get_cache().get_size(), 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_tx_add_cache_tcp() {
        // Channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        let tx_add_cache_tcp_test = resolver.get_tx_add_cache_tcp();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);

        tx_add_cache_tcp_test
            .send((String::from("test"), rr.clone()))
            .unwrap();
        let (name, rr_result) = rx_add_cache_tcp.recv().unwrap();

        /* if the message was correctly sent it should work with the variable
        created with the get fn used */
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_tx_delete_cache_udp() {
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        let tx_delete_cache_udp_test = resolver.get_tx_delete_cache_udp();
        let delete_rcv_upd = _rx_delete_cache_udp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        tx_delete_cache_udp_test.send(msg).unwrap();
        let (name, rr_result) = delete_rcv_upd.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_tx_delete_cache_tcp() {
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        let tx_delete_cache_tcp_test = resolver.get_tx_delete_cache_tcp();
        let delete_rcv_tcp = _rx_delete_cache_tcp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        tx_delete_cache_tcp_test.send(msg).unwrap();
        let (name, rr_result) = delete_rcv_tcp.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    // ToDo: Revisar Práctica 1
    #[test]
    fn get_tx_update_cache_time_udp() {
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        let update_cache_udp_test = resolver.get_tx_update_cache_time_udp();
        let rcv_update_cache_udp = _rx_update_cache_time_udp;
        let msg = (String::from("test1"), String::from("test2"), 1);

        update_cache_udp_test.send(msg.clone()).unwrap();
        let msg_result = rcv_update_cache_udp.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/
        assert_eq!(msg_result, msg.clone());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_tx_update_cache_time_tcp() {
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        let update_cache_tcp_test = resolver.get_tx_update_cache_time_tcp();
        let rcv_update_cache_tcp = _rx_update_cache_time_tcp;
        let msg = (String::from("test1"), String::from("test2"), 1);
        update_cache_tcp_test.send(msg.clone()).unwrap();

        let msg_result = rcv_update_cache_tcp.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/
        assert_eq!(msg_result, msg.clone());
    }

    #[test]
    fn receive_udp_msg_value() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();
            
        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        let host_address_and_port = "127.0.0.1:34254";
        let host_socket = UdpSocket::bind(host_address_and_port).expect("Failed to bind host socket");
        let messages = HashMap::<u16, DnsMessage>::new();

        // Send a message to the origin host socket
        let dns_query_message = 
            DnsMessage::new_query_message(String::from("test.com"), "A", "IN", 0, false, 1);
        let socket_to_send_msg = UdpSocket::bind("127.0.0.1:4242").expect("Failed to bind host socket");
        let _result = socket_to_send_msg.connect(host_address_and_port);
        socket_to_send_msg.send_to(&dns_query_message.to_bytes(), host_address_and_port).expect("couldn't send data");

        let (dns_message, src_address) = resolver.receive_udp_msg_value(host_socket.try_clone().unwrap(), messages.clone());
        
        assert_eq!(dns_message.get_answer().len(), 0);
        assert_eq!(dns_message.get_query_id(), 1);
        assert_eq!(dns_message.get_question().get_qname().to_string(), String::from("test.com"));
        assert_eq!(src_address, "127.0.0.1:4242");
    }

    #[test]
    fn receive_udp_msg_empty_messages() {
        let messages = HashMap::<u16, DnsMessage>::new();
        let origin_port_address = "127.0.0.1:34254";
        let socket_origin = UdpSocket::bind(origin_port_address).expect("Failed to bind host socket");
        
        // Send a message to the origin socket
        let dns_query_message =
        DnsMessage::new_query_message(String::from("test.com"), "A", "IN", 0, false, 1);
        let socket_to_send_msg = UdpSocket::bind("127.0.0.1:4242").expect("Failed to bind host socket");
        let _result = socket_to_send_msg.connect(origin_port_address);
        socket_to_send_msg.send_to(&dns_query_message.to_bytes(), origin_port_address).expect("couldn't send data");
   
        let dns_message_option = Resolver::receive_udp_msg(socket_origin, messages);
        
        let dns_message;
        let msg_origin_address;
        match dns_message_option {
            Some(value) => {
                dns_message = value.0;
                msg_origin_address = value.1;
            }
            None => {panic!("No message received")}

        };

        assert_eq!(dns_message.get_query_id(), 1);
        assert_eq!(dns_message.get_question().get_qname().to_string(), String::from("test.com"));
        assert_eq!(msg_origin_address, "127.0.0.1:4242");
    }

    #[test]
    fn receive_udp_msg_with_existing_msg() {
        let mut messages = HashMap::<u16, DnsMessage>::new();
        let dns_message_to_send =
            DnsMessage::new_query_message(String::from("test.com"), "A", "IN", 0, false, 1);
        messages.insert(1, dns_message_to_send.clone()); 

        let origin_port_address = "127.0.0.1:34254";
        let socket_origin = UdpSocket::bind(origin_port_address).expect("Failed to bind host socket");
        
        // Send a message to the origin socket
        let socket_to_send_msg = UdpSocket::bind("127.0.0.1:4242").expect("Failed to bind host socket");
        let _result = socket_to_send_msg.connect(origin_port_address);
        socket_to_send_msg.send_to(&dns_message_to_send.to_bytes(), origin_port_address).expect("couldn't send data");
   
        let dns_message_option = Resolver::receive_udp_msg(socket_origin, messages);
        
        let dns_message;
        let msg_origin_address;
        match dns_message_option {
            Some(value) => {
                dns_message = value.0;
                msg_origin_address = value.1;
            }
            None => {panic!("No message received")}

        };

        assert_eq!(dns_message.get_query_id(), 1);
        assert_eq!(dns_message.get_question().get_qname().to_string(), String::from("test.com"));
        assert_eq!(msg_origin_address, "127.0.0.1:4242");
    }

    #[test]
    fn update_queries_empy() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        // Hashmap to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();
    
        // Channel to update resolver queries from queries in progress
        let (_tx_update_query, rx_update_query): 
        (Sender<ResolverQuery>, Receiver<ResolverQuery>) = mpsc::channel();
        resolver.update_queries(&rx_update_query, &mut queries_hash_by_id);
        
        // It should be empty since no query was given
        assert!(queries_hash_by_id.is_empty());
    }

    #[test]
    fn update_queries() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        // Channel to update resolver queries
        let (tx_update_query, 
            rx_update_query) = mpsc::channel();
        let (tx_delete_query, 
            _rx_delete_query) = mpsc::channel();
        let (tx_update_slist_tcp, 
            _rx_update_slist_tcp) = mpsc::channel();
        let (tx_update_self_slist, 
            _rx_update_self_slist) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
            resolver.get_tx_add_cache_udp(),
            resolver.get_tx_delete_cache_udp(),
            resolver.get_tx_add_cache_tcp(),
            resolver.get_tx_delete_cache_tcp(),
            tx_update_query.clone(),
            tx_delete_query,
            DnsMessage::new(),
            resolver.get_tx_update_cache_time_tcp(),
            resolver.get_tx_update_cache_time_tcp(),
            tx_update_slist_tcp,
            tx_update_self_slist,
        );

        let host_name = String::from("test.com");
        let query_id = resolver_query.get_main_query_id();
        resolver_query.set_last_query_hostname(host_name.clone());

        let _result = tx_update_query.send(resolver_query);

        // Hashmap to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();
        assert_eq!(queries_hash_by_id.len(), 0);   

        resolver.update_queries(&rx_update_query, &mut queries_hash_by_id);
        assert_eq!(queries_hash_by_id.len(), 1);

        // Query added with correct ID
        for (id, query) in queries_hash_by_id {
            assert_eq!(id, query_id);
            assert_eq!(host_name, query.get_last_query_hostname());
        }
    }

    #[test]
    fn delete_single_answered_query() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );

        // Channel to delete queries
        let (tx_update_query, 
            _rx_update_query) = mpsc::channel();
        let (tx_delete_query, 
            rx_delete_query) = mpsc::channel();
        let (tx_update_slist_tcp, 
            _rx_update_slist_tcp) = mpsc::channel();
        let (tx_update_self_slist, 
            _rx_update_self_slist) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
            resolver.get_tx_add_cache_udp(),
            resolver.get_tx_delete_cache_udp(),
            resolver.get_tx_add_cache_tcp(),
            resolver.get_tx_delete_cache_tcp(),
            tx_update_query.clone(),
            tx_delete_query.clone(),
            DnsMessage::new(),
            resolver.get_tx_update_cache_time_tcp(),
            resolver.get_tx_update_cache_time_tcp(),
            tx_update_slist_tcp,
            tx_update_self_slist,
        );

        let host_name = String::from("test.com");
        let query_id = resolver_query.get_main_query_id();
        resolver_query.set_last_query_hostname(host_name.clone());

        // Hashmap which saves the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();
        queries_hash_by_id.insert(query_id.clone(), resolver_query.clone());

        let _result = tx_delete_query.send(resolver_query);
        
        assert_eq!(queries_hash_by_id.len(), 1);   
        resolver.delete_answered_queries(&rx_delete_query, &mut queries_hash_by_id);
        assert_eq!(queries_hash_by_id.len(), 0);
    }

    #[test]
    fn delete_from_cache() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp.clone(),
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_udp,
            tx_update_cache_tcp,
        );
        // Set the Resource Records to the Resolver's cache
        let domain_name_1 = "dcc.uchile.cl.".to_string();
        let mut a_rdata_1 = ARdata::new();
        let ip_address_1: [u8; 4] = [127, 0, 0, 0];
        a_rdata_1.set_address(ip_address_1);
        let rdata_1 = Rdata::SomeARdata(a_rdata_1);
        let mut resource_record_1 = ResourceRecord::new(rdata_1);
        resource_record_1.set_type_code(Rtype::A);

        let domain_name_2 = "example.com.".to_string();
        let mut a_rdata_2 = ARdata::new();
        let ip_address_2: [u8; 4] = [127, 0, 0, 0];
        a_rdata_2.set_address(ip_address_2);
        let rdata_2 = Rdata::SomeARdata(a_rdata_2);
        let mut resource_record_2 = ResourceRecord::new(rdata_2);
        resource_record_2.set_type_code(Rtype::A);

        let mut cache = DnsCache::new();
        cache.set_max_size(5);
        cache.add(domain_name_1.clone(), resource_record_1.clone());
        cache.add(domain_name_2.clone(), resource_record_2.clone());
        resolver.set_cache(cache);

        assert_eq!(resolver.get_cache().get_size(), 2);

        // Send cache to delete
        let _result = tx_delete_cache_udp.send((domain_name_1, resource_record_1));
        resolver.delete_from_cache(&rx_delete_cache_udp);
        assert_eq!(resolver.get_cache().get_size(), 1);

        let _result = tx_delete_cache_udp.send((domain_name_2, resource_record_2));
        resolver.delete_from_cache(&rx_delete_cache_udp);
        assert_eq!(resolver.get_cache().get_size(), 0);
    }

    #[test]
    fn update_cache_response_time_udp() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_time_udp, 
            rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_time_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp.clone(),
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_time_udp.clone(),
            tx_update_cache_time_tcp,
        );

        // Set the Resource Records to the Resolver's cache
        let domain_name = "dcc.uchile.cl.".to_string();
        let mut a_rdata = ARdata::new();
        let ip_address: [u8; 4] = [127, 0, 0, 1];
        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rtype::A);

        let mut cache = DnsCache::new();
        cache.set_max_size(5);
        cache.add(domain_name.clone(), resource_record.clone());
        resolver.set_cache(cache);
        assert_eq!(resolver.get_cache().get_size(), 1);

        let old_response_time = resolver.get_cache().get_response_time(
            domain_name.clone(),
            String::from("A"),
            String::from("127.0.0.1"),
        );
        assert_eq!(old_response_time, 5000 as u32);

        let response_time_to_update = 4000 as u32;

        // Send response time to update
        let _result = tx_update_cache_time_udp.send(
            (domain_name.clone(), 
            String::from("127.0.0.1"), 
            response_time_to_update)
        );

        resolver.update_cache_response_time_udp(&rx_update_cache_time_udp);

        let new_response_time = resolver.get_cache().get_response_time(
            domain_name.clone(),
            String::from("A"),
            String::from("127.0.0.1"),
        );
        assert_eq!(new_response_time, (old_response_time+response_time_to_update)/2);
    }

    #[test]
    fn add_to_cache_upd() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_time_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_time_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp.clone(),
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_time_udp,
            tx_update_cache_time_tcp,
        );

        // Set the Resource Records to the Resolver's cache
        let domain_name_1 = "dcc.uchile.cl.".to_string();
        let mut a_rdata_1 = ARdata::new();
        let ip_address_1: [u8; 4] = [127, 0, 0, 0];
        a_rdata_1.set_address(ip_address_1);
        let rdata_1 = Rdata::SomeARdata(a_rdata_1);
        let mut resource_record_1 = ResourceRecord::new(rdata_1);
        resource_record_1.set_type_code(Rtype::A);

        let mut cache = DnsCache::new();
        cache.set_max_size(5);
        cache.add(domain_name_1.clone(), resource_record_1.clone());
        resolver.set_cache(cache);
        assert_eq!(resolver.get_cache().get_size(), 1);

        // Send a new Resource Record to add to the cache
        let domain_name_to_add = "example.com.".to_string();
        let domain_name_ref = &domain_name_to_add;
        let mut a_rdata_to_add = ARdata::new();
        let ip_address_to_add: [u8; 4] = [127, 0, 0, 0];
        a_rdata_to_add.set_address(ip_address_to_add);
        let rdata_to_add = Rdata::SomeARdata(a_rdata_to_add);
        let mut resource_record_to_add = ResourceRecord::new(rdata_to_add);
        resource_record_to_add.set_type_code(Rtype::A);

        let _result = tx_add_cache_udp.send(
            (domain_name_to_add.clone(), 
            resource_record_to_add)
        );
        resolver.add_to_cache_upd(&rx_add_cache_udp);

        assert_eq!(resolver.get_cache().get_size(), 2);

        for (_type, cache) in resolver.get_cache().get_cache() {
            assert!(cache.contains_key(domain_name_ref));
        }
    }

    #[test]
    #[ignore = "Investigate how timeout should be handled."]
    fn check_queries_timeout() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_time_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_time_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_time_udp,
            tx_update_cache_time_tcp,
        );

        resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);

        // Channel to update resolver queries
        let (tx_update_query, 
            rx_update_query) = mpsc::channel();
        let (tx_delete_query, 
            _rx_delete_query) = mpsc::channel();
        let (tx_update_slist_tcp, 
            _rx_update_slist_tcp) = mpsc::channel();
        let (tx_update_self_slist, 
            _rx_update_self_slist) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
            resolver.get_tx_add_cache_udp(),
            resolver.get_tx_delete_cache_udp(),
            resolver.get_tx_add_cache_tcp(),
            resolver.get_tx_delete_cache_tcp(),
            tx_update_query.clone(),
            tx_delete_query,
            DnsMessage::new(),
            resolver.get_tx_update_cache_time_tcp(),
            resolver.get_tx_update_cache_time_tcp(),
            tx_update_slist_tcp,
            tx_update_self_slist,
        );

        // Send query to be saved in the Hashmap
        let host_name = String::from("test.com");
        resolver_query.set_last_query_hostname(host_name.clone());
        let _result = tx_update_query.send(resolver_query);

        // Hashmap to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();
        resolver.update_queries(&rx_update_query, &mut queries_hash_by_id);

        // Create socket to received messages
        let origin_port_address = "127.0.0.1:34254";
        let socket_origin = UdpSocket::bind(origin_port_address).expect("Failed to bind host socket");

        resolver.check_queries_timeout(queries_hash_by_id, socket_origin);
    }

    #[test]
    fn new_query_from_msg() {
        // Create resolver channels
        let (tx_add_cache_udp, 
            _rx_add_cache_udp) = mpsc::channel();
        let (tx_delete_cache_udp, 
            _rx_delete_cache_udp) = mpsc::channel();
        let (tx_add_cache_tcp, 
            _rx_add_cache_tcp) = mpsc::channel();
        let (tx_delete_cache_tcp, 
            _rx_delete_cache_tcp) = mpsc::channel();
        let (tx_update_cache_time_udp, 
            _rx_update_cache_time_udp) = mpsc::channel();
        let (tx_update_cache_time_tcp, 
            _rx_update_cache_time_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            tx_add_cache_udp,
            tx_delete_cache_udp,
            tx_add_cache_tcp,
            tx_delete_cache_tcp,
            tx_update_cache_time_udp,
            tx_update_cache_time_tcp,
        );

        resolver.set_initial_configuration(RESOLVER_IP_PORT, SBELT_ROOT_IPS);
        assert_eq!(resolver.get_sbelt().len(), 3);
        
        // Set the Resource Records to the Resolver's cache
        let domain_name = "dcc.uchile.cl.".to_string();
        let mut a_rdata = ARdata::new();
        let ip_address: [u8; 4] = [127, 0, 0, 1];
        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rtype::A);

        let mut cache = DnsCache::new();
        cache.set_max_size(5);
        cache.add(domain_name.clone(), resource_record.clone());
        resolver.set_cache(cache);
        assert_eq!(resolver.get_cache().get_size(), 1);

        // rd must be false to be considered a query
        let dns_query_message = DnsMessage::new_query_message(
                String::from("test.com"), 
                "A", 
                "IN", 
                0, 
                false, 
                7
            );
        let src_address = "127.0.0.1:34254";

        let (tx_update_query, 
            _rx_update_query) = mpsc::channel();
        let (tx_delete_query, 
            _rx_delete_query) = mpsc::channel();
        
        let (
            resolver_query, 
            _rx_update_slist_tcp, 
            _rx_update_self_slist
        ) = resolver.new_query_from_msg(
            dns_query_message, 
            src_address.to_string(), 
            tx_update_query, 
            tx_delete_query);

        assert_eq!(resolver_query.get_sname(), String::from("test.com"));
        assert_eq!(resolver_query.get_stype(), Qtype::A);
        assert_eq!(resolver_query.get_sclass(), Qclass::IN);
        assert_eq!(resolver_query.get_op_code(), 0);
        assert!(!resolver_query.get_rd());
        assert_eq!(resolver_query.get_cache().get_size(), 1);
        assert_eq!(resolver_query.get_old_id(), 7);
        assert_eq!(resolver_query.get_sbelt().len(), 3);
    }
 }
