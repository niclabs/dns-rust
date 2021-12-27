use crate::dns_cache::DnsCache;
use crate::name_server::zone::NSZone;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub mod master_file;
pub mod zone;

#[derive(Clone)]
/// Structs that represents a name server
pub struct NameServer {
    zones: HashMap<String, NSZone>,
    cache: DnsCache,
}

impl NameServer {
    /// Creates a new name server
    pub fn new() -> Self {
        let name_server = NameServer {
            zones: HashMap::<String, NSZone>::new(),
            cache: DnsCache::new(),
        };

        name_server
    }

    pub fn add_zone_from_master_file(&mut self, file_name: String) {
        let new_zone = NSZone::from_file(file_name);
        let mut zones = self.get_zones();

        zones.insert(new_zone.get_name(), new_zone);

        self.set_zones(zones);
    }

    pub fn run_name_server_udp(&mut self, ip_address: String) {
        ip_address.push_str(":53");

        // Creates an UDP socket
        let socket = UdpSocket::bind(&ip_address).expect("Failed to bind host socket");
        println!("{}", "Socket Created");

        loop {
            println!("{}", "Waiting msg");

            // We receive the msg
            let mut received_msg = [0; 512];
            let (_number_of_bytes, src_address) = socket
                .recv_from(&mut received_msg)
                .expect("No data received");

            println!("{}", "Message recv");

            let zones = self.get_zones();

            thread::spawn(move || {
                // Msg parsed
                let dns_message = DnsMessage::from_bytes(&received_msg);

                println!("{}", "Paso parseo");

                // Set RA bit to 1
                let new_msg = self.set_ra(true);

                let rd = new_msg.get_header().get_rd();

                if rd == true {
                } else {
                    let qname = new_msg.get_question().get_qname().get_name();
                    let mut (zone, available) = NameServer::search_nearest_ancestor_zone(zones, qname.clone());

                    if available == true {
                        let response_msg = NameServer::search_in_zone(zone: NSZone, qname.clone(), new_msg.clone());
                    }
                    else {
                        let response_msg = NameServer::step_4();
                    }

                }
            });
        }
    }

    fn set_ra(&mut self, msg: DnsMessage, ra: bool) -> DnsMessage {
        let mut header = msg.get_header();
        header.set_ra(ra);

        msg.set_header(header);

        msg
    }
}

// utils functions
impl NameServer {
    // Step 2 from RFC 1034
    fn search_nearest_ancestor_zone(zones: HashMap<String, NSZone>, qname: String) -> (NSZone, bool) {
        let (mut zone, mut available) = match zones.get(&qname) {
            Some(val) => (val, true),
            None => (NSZone::new(), false),
        };
        
        let dot_position = qname.find(".").unwrap_or(0);
        if dot_position > 0 {
            qname.replace_range(..dot_position + 1, "");
            return search_nearest_ancestor_zone(zones, qname);
        }
        else {
            return (zone, available);
        }
    }

    //Step 3 from RFC 1034
    fn search_in_zone(zone: NSZone, qname: String, msg: DnsMessage) -> DnsMessage {
        let mut qname_without_zone_label = qname.replace(zone.get_name(), "");

        // We were looking for the first node
        if qname_without_zone_label == "".to_string() {
            return NameServer::step_3a(zone, msg);
        }

        // Delete last dot
        qname_without_zone_label.pop().unwrap_or("".to_string());

        let mut labels: Vec<&str> = qname_without_zone_label.split(".").collect();
        
        labels.reverse();
        
        for label in labels {
            let exist_child = zone.exist_child(label);

            if exist_child == true {
                (zone, _) = zone.get_child(label);

                if zone.get_subzone() == true {
                    return NameServer::step_3b();
                }
                else {
                    continue;
                }

            }
            else {
                return NameServer::step_3c();
            }
        }

        return NameServer::step_3a(zone, msg);
    }

    pub fn step_3a(zone: NSZone, msg: DnsMessage) -> DnsMessage {
        // Step 3.a
        let qtype = msg.get_question().get_qtype();
        let rrs_by_type = zone.get_rrs_by_type(qtype);

        if rrs_by_type.len() > 0 {
            msg.set_answer(rrs_by_type);
            msg.set_header(msg.get_header().set_aa(true));

            return msg;
        }
        else {
            let rr = zone.get_value()[0];
            if rr.get_type_code() == 5 && qtype != 5 {
                rrs_by_type.push(rr);
                msg.set_answer(rrs_by_type);

                let canonical_name = match rr.get_rdata() {
                    Rdata::SomeCNameRdata(val) => val.get_cname().get_name();
                    _ => unreachable!(),
                };

                msg.set_question(msg.get_question().set_qname(canonical_name));

                return msg;
            }
        }
        //
    }

    pub fn step_3b(zone: NSZone, msg: DnsMessage, cache: DnsCache) -> DnsMessage {
        let ns_rrs = zone.get_value();
        msg.set_authority(ns_rrs);
        let mut additional = Vec::<ResourceRecord>::new();

        for ns_rr in ns_rrs {
            let mut name_ns = match ns_rr.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => unreachable!(),
            };

            let rrs = cache.get(name_ns, "A");

            if rrs.len() > 0 {
                additional.append(rrs);
            }
            else {
                match name_ns.find(zone.get_name()) {
                    Some(val) => {
                        let glue_rrs = zone.get_glue_rrs();

                        let a_glue_rrs = NameServer::look_for_type_records(name_ns, glue_rrs, 1);

                        additional.append(a_glue_rrs);
                    },
                    None => {},
                }
            } 
        }

        msg.set_additional(additional);

        return msg;
    }

    pub fn step_3c(zone: NSZone, msg: DnsMessage) -> DnsMessage {
        let exist = zone.exist_child("*".to_string());

        if exist == true {
            let new_zone = zone.get_child("*".to_string());
            let rrs = new_zone.get_value();
            let qtype = msg.get_question().get_qtype();
            let answer = Vec::<ResourceRecord>::new();

            for rr in rrs{
                if rr.get_type_code() == qtype {
                    rr.set_name(msg.get_question().get_qname());
                    answer.push(rr);
                }
            }

            msg.set_answer(answer);

            return msg;
        }
        else {
            if msg.get_answer().len() > 0 {
                if msg.get_answer()[0].get_type_code() == 5 {
                    let header = msg.get_header();
                    header.set_rcode(3);
                    header.set_aa(true);

                    msg.set_header(header);
                }
            }

            return msg;
        }
    }

    pub fn step_4() -> DnsMessage {

    }

    fn look_for_type_records(name_ns: String, rrs: Vec<ResourceRecord>, rr_type: u16) -> Vec<ResourceRecord> {
        let a_rrs = Vec::<ResourceRecord>::new();

        for rr in rrs {
            let rr_type_glue = rr.get_type_code();
            let rr_name = rr.get_name().get_name();

            if rr_type_glue == rr_type && rr_name == name_ns {
                a_rrs.push(rr);
            }
        }

        return a_rrs
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
}
