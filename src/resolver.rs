use crate::dns_cache::DnsCache;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::resolver::resolver_query::ResolverQuery;
use crate::resolver::slist::Slist;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

pub mod resolver_query;
pub mod slist;

#[derive(Clone)]
/// Struct that represents a dns resolver
pub struct Resolver {
    /// Ip address where the resolver will send the messages
    ip_address: String,
    // Port where the resolver will be connected
    port: String,
    // Struct that contains a default server list to ask
    sbelt: Slist,
    // Cache for the resolver
    cache: DnsCache,
    // Name server data
    ns_data: HashMap<String, HashMap<String, Vec<ResourceRecord>>>,
}

impl Resolver {
    /// Creates a new Resolver with default values
    pub fn new() -> Self {
        let mut cache = DnsCache::new();
        cache.set_max_size(100);

        let resolver = Resolver {
            ip_address: String::from(""),
            port: String::from(""),
            sbelt: Slist::new(),
            cache: cache,
            ns_data: HashMap::<String, HashMap<String, Vec<ResourceRecord>>>::new(),
        };
        resolver
    }

    ////////////////////////////////////////
    // Al crear una nueva query, dejar sbelt como default de slist
    ////////////////////////////////////////

    pub fn run_resolver_udp(&mut self) {
        // Vector to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();
        let (tx, rx) = mpsc::channel();

        // Create ip and port str
        let mut host_address_and_port = self.get_ip_address();
        let port = &self.get_port();
        host_address_and_port.push_str(":");
        host_address_and_port.push_str(&port.clone());

        // Creates an UDP socket
        let socket = UdpSocket::bind(&host_address_and_port).expect("Failed to bind host socket");
        println!("{}", "Socket Created");

        // Receives messages
        loop {
            let mut resolver = self.clone();

            println!("{}", "Waiting msg");

            // We receive the msg
            let mut received_msg = [0; 512];
            let (_number_of_bytes, src_address) = socket
                .recv_from(&mut received_msg)
                .expect("No data received");

            println!("{}", "Message recv");

            // Updating Cache

            let mut received = rx.try_iter();

            let mut next_value = received.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (name, rr) = next_value.unwrap();
                cache.add(name, rr);
                next_value = received.next();
            }

            self.set_cache(cache);

            //

            let dns_message = DnsMessage::from_bytes(&received_msg);

            println!("{}", "Paso parseo");

            // We get the msg type, it can be query or answer
            let msg_type = dns_message.get_header().get_qr();

            println!("Msg type: {}", msg_type.clone());

            let mut new_port: u16 = port.parse().unwrap();
            new_port = new_port + 1;

            let mut ip_local = socket.local_addr().unwrap().ip().to_string();
            ip_local.push_str(":");
            ip_local.push_str(&new_port.to_string());

            if msg_type == false {
                let sname = dns_message.get_question().get_qname().get_name();
                let stype = dns_message.get_question().get_qtype();
                let sclass = dns_message.get_question().get_qclass();
                let op_code = dns_message.get_header().get_op_code();
                let rd = dns_message.get_header().get_rd();

                let mut resolver_query = ResolverQuery::new();

                resolver_query.set_sname(sname);
                resolver_query.set_stype(stype);
                resolver_query.set_sclass(sclass);
                resolver_query.set_op_code(op_code);
                resolver_query.set_rd(rd);
                resolver_query.set_sbelt(resolver.get_sbelt());
                resolver_query.set_cache(resolver.get_cache());
                resolver_query.set_ns_data(resolver.get_ns_data());
                resolver_query.set_src_address(src_address.clone().to_string());

                let sbelt = resolver_query.get_sbelt();
                resolver_query.initialize_slist(sbelt);

                queries_hash_by_id
                    .insert(resolver_query.get_main_query_id(), resolver_query.clone());

                let socket_copy = socket.try_clone().unwrap();

                let resolver_copy = resolver.clone();

                let dns_msg_copy = dns_message.clone();

                let ip_local_copy = ip_local.clone();

                thread::spawn(move || {
                    let answer = resolver_query.look_for_local_info();

                    if answer.len() > 0 {
                        println!("Cacheeeeeeee!");

                        let mut new_dns_msg = dns_msg_copy.clone();
                        new_dns_msg.set_answer(answer.clone());
                        new_dns_msg.set_authority(Vec::new());
                        new_dns_msg.set_additional(Vec::new());

                        let mut header = new_dns_msg.get_header();
                        header.set_ancount(answer.len() as u16);

                        Resolver::send_answer_by_udp(
                            new_dns_msg,
                            src_address.to_string(),
                            &socket_copy,
                        );
                    } else {
                        resolver_query.send_query_udp(socket_copy);
                    }
                    println!("{}", "Thread Finished")
                });
            }

            if msg_type == true {
                let tx_clone = tx.clone();
                let socket_copy = socket.try_clone().unwrap();
                let answer_id = dns_message.get_query_id();
                let queries_hash_by_id_copy = queries_hash_by_id.clone();

                let resolver_copy = resolver.clone();

                if queries_hash_by_id_copy.contains_key(&answer_id) {
                    println!("Entro por la ID");
                    thread::spawn(move || {
                        let resolver_query = queries_hash_by_id_copy.get(&answer_id).unwrap();
                        let response = match resolver_query.clone().process_answer_udp(
                            dns_message,
                            socket_copy.try_clone().unwrap(),
                            tx_clone,
                        ) {
                            Some(val) => vec![val],
                            None => Vec::new(),
                        };

                        if response.len() > 0 {
                            let answers = response[0].clone().get_answer();

                            Resolver::send_answer_by_udp(
                                response[0].clone(),
                                resolver_query.get_src_address(),
                                &socket_copy,
                            );
                        }
                    });
                }
            }
        }
    }

    pub fn run_resolver_tcp(&mut self) {
        // Vector to save the queries in process
        let mut queries_hash_by_id = HashMap::<u16, ResolverQuery>::new();

        // Create sender
        let (tx, rx) = mpsc::channel();

        // Create ip and port str
        let mut host_address_and_port = self.get_ip_address();
        host_address_and_port.push_str(":");
        host_address_and_port.push_str(&self.get_port());

        // Creates an TCP Listener
        let listener = TcpListener::bind(&host_address_and_port).expect("Could not bind");
        println!("{}", "TcpListener Created");

        // Receives messages
        loop {
            let resolver = self.clone();

            println!("{}", "Waiting msg");

            match listener.accept() {
                Ok((mut stream, src_address)) => {
                    // Updating Cache

                    let mut received = rx.try_iter();

                    let mut next_value = received.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (name, rr) = next_value.unwrap();
                        cache.add(name, rr);
                        next_value = received.next();
                    }

                    self.set_cache(cache);

                    //

                    println!("New connection: {}", stream.peer_addr().unwrap());

                    // We receive the msg
                    let mut received_msg = [0; 512];
                    let _number_of_bytes =
                        stream.read(&mut received_msg).expect("No data received");

                    println!("{}", "Message recv");

                    //let socket_copy = socket.try_clone().unwrap();

                    let resolver_copy = resolver.clone();

                    let tx_clone = tx.clone();

                    thread::spawn(move || {
                        let dns_message = DnsMessage::from_bytes(&received_msg);

                        println!("{}", "Paso parseo");

                        // We get the msg type, it can be query or answer
                        let msg_type = dns_message.get_header().get_qr();

                        if msg_type == false {
                            let sname = dns_message.get_question().get_qname().get_name();
                            let stype = dns_message.get_question().get_qtype();
                            let sclass = dns_message.get_question().get_qclass();
                            let op_code = dns_message.get_header().get_op_code();
                            let rd = dns_message.get_header().get_rd();

                            let mut resolver_query = ResolverQuery::new();

                            resolver_query.set_sname(sname);
                            resolver_query.set_stype(stype);
                            resolver_query.set_sclass(sclass);
                            resolver_query.set_op_code(op_code);
                            resolver_query.set_rd(rd);
                            resolver_query.set_sbelt(resolver.get_sbelt());
                            resolver_query.set_cache(resolver.get_cache());
                            resolver_query.set_ns_data(resolver.get_ns_data());
                            resolver_query.set_src_address(src_address.clone().to_string());

                            let answer = resolver_query.look_for_local_info();

                            if answer.len() > 0 {
                                let mut new_dns_msg = dns_message.clone();
                                new_dns_msg.set_answer(answer.clone());
                                new_dns_msg.set_authority(Vec::new());
                                new_dns_msg.set_additional(Vec::new());

                                let mut header = new_dns_msg.get_header();
                                header.set_ancount(answer.len() as u16);

                                Resolver::send_answer_by_tcp(
                                    new_dns_msg,
                                    stream.peer_addr().unwrap().to_string(),
                                );
                            } else {
                                let sbelt = resolver_query.get_sbelt();
                                resolver_query.initialize_slist(sbelt);
                                resolver_query.send_query_tcp(tx_clone);
                            }
                            println!("{}", "Thread Finished")
                        }

                        /*
                        if msg_type == true {
                            //let socket_copy = socket.try_clone().unwrap();
                            let answer_id = dns_message.get_query_id();
                            let queries_hash_by_id_copy = queries_hash_by_id.clone();

                            let resolver_copy = resolver.clone();

                            if queries_hash_by_id_copy.contains_key(&answer_id) {
                                    let resolver_query = queries_hash_by_id_copy.get(&answer_id).unwrap();
                                    let response = match resolver_query
                                        .clone()
                                        .process_answer(dns_message, "udp".to_string())
                                    {
                                        Some(val) => vec![val],
                                        None => Vec::new(),
                                    };

                                    if response.len() > 0 {
                                        resolver_copy.send_answer_by_udp(
                                            response[0].clone(),
                                            src_address.to_string(),
                                            &socket_copy,
                                        );
                                    }
                            }
                        }
                        */
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    // Sends the response to the address by udp
    fn send_answer_by_udp(response: DnsMessage, src_address: String, socket: &UdpSocket) {
        let bytes = response.to_bytes();

        socket
            .send_to(&bytes, src_address)
            .expect("failed to send message");
    }

    // Sends the response to the address by tcp
    fn send_answer_by_tcp(response: DnsMessage, src_address: String) {
        let bytes = response.to_bytes();

        let mut stream = TcpStream::connect(src_address).unwrap();
        stream.write(&bytes);
    }
}

// Getters
impl Resolver {
    // Gets the ip address
    pub fn get_ip_address(&self) -> String {
        self.ip_address.clone()
    }

    // Gets the port of the resolver
    pub fn get_port(&self) -> String {
        self.port.clone()
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
    pub fn get_ns_data(&self) -> HashMap<String, HashMap<String, Vec<ResourceRecord>>> {
        self.ns_data.clone()
    }
}

//Setters
impl Resolver {
    // Sets the ip address attribute with a value
    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = ip_address;
    }

    // Sets the port attribute with a value
    pub fn set_port(&mut self, port: String) {
        self.port = port;
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
    pub fn set_ns_data(&mut self, ns_data: HashMap<String, HashMap<String, Vec<ResourceRecord>>>) {
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
    use crate::resolver::resolver_query::ResolverQuery;
    use crate::resolver::slist::Slist;
    use crate::resolver::Resolver;
    use std::collections::HashMap;
    use std::vec::Vec;

    #[test]
    fn constructor_test() {
        let resolver = Resolver::new();

        assert_eq!(resolver.ip_address, "".to_string());
        assert_eq!(resolver.port, "".to_string());
        assert_eq!(resolver.sbelt.get_ns_list().len(), 0);
        assert_eq!(resolver.cache.get_size(), 0);
    }

    #[test]
    fn set_and_get_ip_address() {
        let mut resolver = Resolver::new();

        assert_eq!(resolver.get_ip_address(), "".to_string());

        resolver.set_ip_address("127.0.0.1".to_string());

        assert_eq!(resolver.get_ip_address(), "127.0.0.1".to_string());
    }

    #[test]
    fn set_and_get_port() {
        let mut resolver = Resolver::new();

        assert_eq!(resolver.get_port(), "".to_string());

        resolver.set_port("25".to_string());

        assert_eq!(resolver.get_port(), "25".to_string());
    }

    #[test]
    fn set_and_get_sbelt() {
        let mut resolver = Resolver::new();
        let mut sbelt_test = Slist::new();

        sbelt_test.insert("test.com".to_string(), "127.0.0.1".to_string(), 5.0);

        resolver.set_sbelt(sbelt_test);

        assert_eq!(resolver.get_sbelt().get_ns_list().len(), 1);
    }

    #[test]
    fn set_and_get_cache() {
        let mut resolver = Resolver::new();
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

        let mut resolver_query_test = ResolverQuery::new();

        assert_eq!(resolver_query_test.get_ns_data().len(), 0);

        resolver_query_test.set_ns_data(rr_type_hash);

        assert_eq!(resolver_query_test.get_ns_data().len(), 1);
    }
}
