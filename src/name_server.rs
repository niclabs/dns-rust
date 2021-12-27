use crate::dns_cache::DnsCache;
use crate::message::rdata::cname_rdata::CnameRdata;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::name_server::zone::NSZone;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub mod master_file;
pub mod zone;

#[derive(Clone)]
/// Structs that represents a name server
pub struct NameServer {
    zones: HashMap<String, NSZone>,
    cache: DnsCache,
    queries_id: HashMap<u16, u16>,
}

impl NameServer {
    /// Creates a new name server
    pub fn new() -> Self {
        let name_server = NameServer {
            zones: HashMap::<String, NSZone>::new(),
            cache: DnsCache::new(),
            queries_id: HashMap::<u16, u16>::new(),
        };

        name_server
    }

    pub fn add_zone_from_master_file(&mut self, file_name: String) {
        let new_zone = NSZone::from_file(file_name);
        let mut zones = self.get_zones();

        zones.insert(new_zone.get_name(), new_zone);

        self.set_zones(zones);
    }

    pub fn run_name_server_udp(
        &mut self,
        mut name_server_ip_address: String,
        local_resolver_ip_and_port: String,
    ) {
        name_server_ip_address.push_str(":53");

        let (tx, rx) = mpsc::channel();

        // Creates an UDP socket
        let socket = UdpSocket::bind(&name_server_ip_address).expect("Failed to bind host socket");
        println!("{}", "Socket Created");

        loop {
            println!("{}", "Waiting msg");

            // We receive the msg
            let mut received_msg = [0; 512];
            let (_number_of_bytes, src_address) = socket
                .recv_from(&mut received_msg)
                .expect("No data received");

            // Update queries ids

            let mut received = rx.try_iter();

            let mut next_value = received.next();

            let mut queries_id = self.get_queries_id();

            while next_value.is_none() == false {
                let (old, new) = next_value.unwrap();
                queries_id.insert(new, old);
                next_value = received.next();
            }

            self.set_queries_id(queries_id);

            //
            println!("{}", "Message recv");

            // Msg parsed
            let mut dns_message = DnsMessage::from_bytes(&received_msg);

            println!("{}", "Paso parseo");

            let socket_copy = socket.try_clone().unwrap();

            if dns_message.get_header().get_qr() == false {
                let zones = self.get_zones();

                let cache = self.get_cache();

                let tx_clone = tx.clone();

                let resolver_ip_clone = local_resolver_ip_and_port.clone();

                thread::spawn(move || {
                    // Set RA bit to 1
                    let new_msg = NameServer::set_ra(dns_message, true);

                    let rd = new_msg.get_header().get_rd();

                    if rd == true {
                        NameServer::step_5(resolver_ip_clone, new_msg, socket_copy, tx_clone);
                    } else {
                        let response_dns_msg = NameServer::step_2(new_msg, zones, cache);
                        NameServer::send_response_udp(
                            socket_copy,
                            response_dns_msg,
                            src_address.to_string(),
                        );
                    }
                });
            } else {
                let mut queries_id = self.get_queries_id();
                let new_id = dns_message.get_query_id();
                match queries_id.get(&new_id) {
                    Some(&val) => {
                        let mut header = dns_message.get_header();
                        header.set_id(val);
                        dns_message.set_header(header);
                        queries_id.remove(&new_id);

                        NameServer::send_response_udp(
                            socket_copy,
                            dns_message,
                            src_address.to_string(),
                        );
                    }
                    None => {}
                }
            }
        }
    }

    fn set_ra(mut msg: DnsMessage, ra: bool) -> DnsMessage {
        let mut header = msg.get_header();
        header.set_ra(ra);

        msg.set_header(header);

        msg
    }
}

// utils functions
impl NameServer {
    // Step 2 from RFC 1034
    fn search_nearest_ancestor_zone(
        zones: HashMap<String, NSZone>,
        mut qname: String,
    ) -> (NSZone, bool) {
        let (mut zone, mut available) = match zones.get(&qname) {
            Some(val) => (val.clone(), true),
            None => (NSZone::new(), false),
        };

        let dot_position = qname.find(".").unwrap_or(0);
        if dot_position > 0 {
            qname.replace_range(..dot_position + 1, "");
            return NameServer::search_nearest_ancestor_zone(zones, qname);
        } else {
            return (zone, available);
        }
    }

    //Step 3 from RFC 1034
    fn search_in_zone(
        zone: NSZone,
        qname: String,
        msg: DnsMessage,
        zones: HashMap<String, NSZone>,
        cache: DnsCache,
    ) -> DnsMessage {
        let mut qname_without_zone_label = qname.replace(&zone.get_name(), "");

        // We were looking for the first node
        if qname_without_zone_label == "".to_string() {
            return NameServer::step_3a(zone, msg, zones, cache);
        }

        // Delete last dot
        qname_without_zone_label.pop().unwrap();

        let mut labels: Vec<&str> = qname_without_zone_label.split(".").collect();

        labels.reverse();

        for label in labels {
            let exist_child = zone.exist_child(label.to_string());

            if exist_child == true {
                let (zone, _available) = zone.get_child(label.to_string());

                if zone.get_subzone() == true {
                    return NameServer::step_3b(zone, msg, cache, zones);
                } else {
                    continue;
                }
            } else {
                return NameServer::step_3c(zone, msg, cache, zones);
            }
        }

        return NameServer::step_3a(zone, msg, zones, cache);
    }

    pub fn step_2(msg: DnsMessage, zones: HashMap<String, NSZone>, cache: DnsCache) -> DnsMessage {
        let qname = msg.get_question().get_qname().get_name();
        let (zone, available) =
            NameServer::search_nearest_ancestor_zone(zones.clone(), qname.clone());

        if available == true {
            // Step 3 RFC 1034
            return NameServer::search_in_zone(zone, qname.clone(), msg.clone(), zones, cache);
        } else {
            // Step 4 RFC 1034
            return NameServer::step_4(msg, cache, zones);
        }
    }

    pub fn step_3a(
        zone: NSZone,
        mut msg: DnsMessage,
        zones: HashMap<String, NSZone>,
        cache: DnsCache,
    ) -> DnsMessage {
        // Step 3.a
        let qtype = msg.get_question().get_qtype();
        let mut rrs_by_type = zone.get_rrs_by_type(qtype);

        if rrs_by_type.len() > 0 {
            msg.set_answer(rrs_by_type);

            let mut header = msg.get_header();

            header.set_aa(true);
            msg.set_header(header);

            return NameServer::step_6(msg, cache, zones);
        } else {
            let rr = zone.get_value()[0].clone();
            if rr.get_type_code() == 5 && qtype != 5 {
                rrs_by_type.push(rr.clone());
                msg.set_answer(rrs_by_type);

                let canonical_name = match rr.get_rdata() {
                    Rdata::SomeCnameRdata(val) => val.get_cname(),
                    _ => unreachable!(),
                };

                let mut question = msg.get_question();

                question.set_qname(canonical_name);
                msg.set_question(question);

                return NameServer::step_2(msg, zones, cache);
            } else {
                return NameServer::step_6(msg, cache, zones);
            }
        }
        //
    }

    pub fn step_3b(
        zone: NSZone,
        mut msg: DnsMessage,
        mut cache: DnsCache,
        zones: HashMap<String, NSZone>,
    ) -> DnsMessage {
        let ns_rrs = zone.get_value();
        msg.set_authority(ns_rrs.clone());
        let mut additional = Vec::<ResourceRecord>::new();

        for ns_rr in ns_rrs {
            let mut name_ns = match ns_rr.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => unreachable!(),
            };

            let rrs = cache.get(name_ns.clone(), "A".to_string());

            if rrs.len() > 0 {
                for rr in rrs {
                    additional.push(rr.get_resource_record());
                }
            } else {
                match name_ns.find(&zone.get_name()) {
                    Some(val) => {
                        let glue_rrs = zone.get_glue_rrs();

                        let mut a_glue_rrs =
                            NameServer::look_for_type_records(name_ns, glue_rrs, 1);

                        additional.append(&mut a_glue_rrs);
                    }
                    None => {}
                }
            }
        }

        msg.set_additional(additional);

        return NameServer::step_4(msg, cache, zones);
    }

    pub fn step_3c(
        zone: NSZone,
        mut msg: DnsMessage,
        cache: DnsCache,
        zones: HashMap<String, NSZone>,
    ) -> DnsMessage {
        let exist = zone.exist_child("*".to_string());

        if exist == true {
            let (new_zone, _available) = zone.get_child("*".to_string());
            let rrs = new_zone.get_value();
            let qtype = msg.get_question().get_qtype();
            let mut answer = Vec::<ResourceRecord>::new();

            for mut rr in rrs {
                if rr.get_type_code() == qtype {
                    rr.set_name(msg.get_question().get_qname());
                    answer.push(rr);
                }
            }

            msg.set_answer(answer);

            let mut header = msg.get_header();
            header.set_aa(true);

            msg.set_header(header);

            return NameServer::step_6(msg, cache, zones);
        } else {
            if msg.get_answer().len() > 0 {
                if msg.get_answer()[0].get_type_code() == 5 {
                    let mut header = msg.get_header();
                    header.set_rcode(3);
                    header.set_aa(true);

                    msg.set_header(header);
                }
            }

            return msg;
        }
    }

    pub fn step_4(
        mut msg: DnsMessage,
        mut cache: DnsCache,
        zones: HashMap<String, NSZone>,
    ) -> DnsMessage {
        let mut domain_name = msg.get_question().get_qname().get_name();
        let qtype = msg.get_question_qtype();
        let rrs = cache.get(domain_name.clone(), qtype);
        let mut answer = Vec::<ResourceRecord>::new();

        for rr in rrs {
            answer.push(rr.get_resource_record());
        }

        msg.set_answer(answer);

        if msg.get_authority().len() > 0 {
            return NameServer::step_6(msg, cache, zones);
        } else {
            let mut authority = Vec::<ResourceRecord>::new();

            while domain_name != "".to_string() {
                let mut rrs = cache.get(domain_name.clone(), "NS".to_string());

                if rrs.len() > 0 {
                    for rr in rrs {
                        authority.push(rr.get_resource_record());
                    }

                    msg.set_authority(authority);

                    break;
                } else {
                    let dot_index = domain_name.find(".").unwrap_or(domain_name.len());

                    if dot_index == domain_name.len() {
                        break;
                    } else {
                        domain_name.replace_range(..dot_index + 1, "");
                    }
                }
            }
        }

        return NameServer::step_6(msg, cache, zones);
    }

    fn step_5(
        resolver_ip_and_port: String,
        mut msg: DnsMessage,
        socket: UdpSocket,
        tx: Sender<(u16, u16)>,
    ) {
        let old_id = msg.get_query_id();
        let mut rng = thread_rng();
        let new_id: u16 = rng.gen();

        let mut header = msg.get_header();
        header.set_id(new_id);

        msg.set_header(header);

        tx.send((old_id, new_id));

        // Send request to resolver
        socket.send_to(&msg.to_bytes(), resolver_ip_and_port);
    }

    /// Adds addittional information to response
    fn step_6(
        mut msg: DnsMessage,
        mut cache: DnsCache,
        zones: HashMap<String, NSZone>,
    ) -> DnsMessage {
        let answers = msg.get_answer();
        let mut additional = Vec::<ResourceRecord>::new();
        let aa = msg.get_header().get_aa();

        for answer in answers {
            let answer_type = answer.get_type_code();

            match answer_type {
                15 => {
                    let exchange = match answer.get_rdata() {
                        Rdata::SomeMxRdata(val) => val.get_exchange().get_name(),
                        _ => unreachable!(),
                    };

                    if aa == true {
                        let (zone, _available) =
                            NameServer::search_nearest_ancestor_zone(zones.clone(), exchange);

                        let mut rrs = zone.get_rrs_by_type(1);

                        additional.append(&mut rrs);
                    } else {
                        let rrs = cache.get(exchange, "A".to_string());

                        for rr in rrs {
                            additional.push(rr.get_resource_record());
                        }
                    }
                }
                2 => {
                    let mut name_ns = match answer.get_rdata() {
                        Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                        _ => unreachable!(),
                    };

                    let (zone, _available) =
                        NameServer::search_nearest_ancestor_zone(zones.clone(), name_ns.clone());

                    if zone.get_subzone() == true {
                        let glue_rrs = zone.get_glue_rrs();

                        let mut a_glue_rrs =
                            NameServer::look_for_type_records(name_ns, glue_rrs, 1);

                        additional.append(&mut a_glue_rrs);
                    } else {
                        let rrs = cache.get(name_ns, "A".to_string());

                        for rr in rrs {
                            additional.push(rr.get_resource_record());
                        }
                    }
                }
                _ => {}
            }
        }

        msg.set_additional(additional);

        return msg;
    }

    fn look_for_type_records(
        name_ns: String,
        rrs: Vec<ResourceRecord>,
        rr_type: u16,
    ) -> Vec<ResourceRecord> {
        let mut a_rrs = Vec::<ResourceRecord>::new();

        for rr in rrs {
            let rr_type_glue = rr.get_type_code();
            let rr_name = rr.get_name().get_name();

            if rr_type_glue == rr_type && rr_name == name_ns {
                a_rrs.push(rr);
            }
        }

        return a_rrs;
    }

    fn send_response_udp(socket: UdpSocket, msg: DnsMessage, address: String) {
        let msg_to_bytes = msg.to_bytes();

        socket.send_to(&msg_to_bytes, address);
    }
}

// Getters
impl NameServer {
    // Gets the zones data from the name server
    pub fn get_zones(&self) -> HashMap<String, NSZone> {
        self.zones.clone()
    }

    // Gets the cache from the name server
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }

    pub fn get_queries_id(&self) -> HashMap<u16, u16> {
        self.queries_id.clone()
    }
}

// Setters
impl NameServer {
    // Sets the zones with a new value
    pub fn set_zones(&mut self, zones: HashMap<String, NSZone>) {
        self.zones = zones;
    }

    // Sets the cache with a new cache
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }

    pub fn set_queries_id(&mut self, queries_id: HashMap<u16, u16>) {
        self.queries_id = queries_id;
    }
}
