use crate::dns_cache::DnsCache;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::name_server::zone::NSZone;
use crate::name_server::NameServer;
use crate::resolver::slist::Slist;
use crate::resolver::Resolver;

use crate::config::QUERIES_FOR_CLIENT_REQUEST;
use crate::config::USE_CACHE;

use chrono::Utc;
use rand::seq::index;
use rand::{thread_rng, Rng};
use std::cmp;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

#[derive(Clone)]
/// This struct represents a resolver query
pub struct ResolverQuery {
    timestamp: u32,
    sname: String,
    stype: u16,
    sclass: u16,
    op_code: u8,
    rd: bool,
    slist: Slist,
    sbelt: Slist,
    cache: DnsCache,
    ns_data: HashMap<u16, HashMap<String, NSZone>>,
    main_query_id: u16,
    old_id: u16,
    src_address: String,
    // Channel to share cache data between threads
    add_channel_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    delete_channel_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    add_channel_tcp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    delete_channel_tcp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between name server and resolver
    add_channel_ns_udp: Sender<(String, ResourceRecord)>,
    // Channel to delete cache data in name server and resolver
    delete_channel_ns_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between name server and resolver
    add_channel_ns_tcp: Sender<(String, ResourceRecord)>,
    // Channel to delete cache data in name server and resolver
    delete_channel_ns_tcp: Sender<(String, ResourceRecord)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_udp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_tcp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_ns_udp: Sender<(String, String, u32)>,
    // Channel to update response time in cache data in name server and resolver
    update_cache_sender_ns_tcp: Sender<(String, String, u32)>,
    // Number of queries that the resolver do before send temporary error
    queries_before_temporary_error: u16,
    // Sender to update ResolverQuery struct in the resolver
    tx_update_query: Sender<ResolverQuery>,
    // Sender to delete ResolverQuery struct in the resolver
    tx_delete_query: Sender<ResolverQuery>,
    // Client msg
    client_msg: DnsMessage,
    // Index to choose from Slist
    index_to_choose: u16,
    // Timeout
    timeout: u32,
    // Last query timestamp
    last_query_timestamp: u64,
    // Last query host name
    last_query_hostname: String,
    // Internal query
    internal_query: bool,
    // Slist query id
    query_id_update_slist: u16,
    // Channel to update slist in tcp resolver
    update_slist_tcp_sender: Sender<(String, Vec<ResourceRecord>)>,
    // Channel to update slist inside a resolver query
    tx_update_self_slist: Sender<Slist>,
}

impl ResolverQuery {
    /// Creates a new ResolverQuery struct with default values
    ///
    /// # Examples
    /// '''
    /// let resolver_query = ResolverQuery::new();
    ///
    /// assert_eq!(resolver_query.sname, "".to_string());
    /// assert_eq!(resolver_query.stype, 0);
    /// assert_eq!(resolver_query.sclass, 0);
    /// assert_eq!(resolver_query.slist.len(), 0);
    /// assert_eq!(resolver_query.cache.clone().len(), 0);
    /// '''
    ///
    pub fn new(
        add_channel_udp: Sender<(String, ResourceRecord)>,
        delete_channel_udp: Sender<(String, ResourceRecord)>,
        add_channel_tcp: Sender<(String, ResourceRecord)>,
        delete_channel_tcp: Sender<(String, ResourceRecord)>,
        add_channel_ns_udp: Sender<(String, ResourceRecord)>,
        delete_channel_ns_udp: Sender<(String, ResourceRecord)>,
        add_channel_ns_tcp: Sender<(String, ResourceRecord)>,
        delete_channel_ns_tcp: Sender<(String, ResourceRecord)>,
        tx_update_query: Sender<ResolverQuery>,
        tx_delete_query: Sender<ResolverQuery>,
        client_msg: DnsMessage,
        update_cache_sender_udp: Sender<(String, String, u32)>,
        update_cache_sender_tcp: Sender<(String, String, u32)>,
        update_cache_sender_ns_udp: Sender<(String, String, u32)>,
        update_cache_sender_ns_tcp: Sender<(String, String, u32)>,
        update_slist_tcp_sender: Sender<(String, Vec<ResourceRecord>)>,
        tx_update_self_slist: Sender<Slist>,
    ) -> Self {
        let mut rng = thread_rng();
        let now = Utc::now();
        let timestamp = now.timestamp() as u32;
        let queries_before_temporary_error = QUERIES_FOR_CLIENT_REQUEST;

        let query = ResolverQuery {
            timestamp: timestamp,
            sname: "".to_string(),
            stype: 0 as u16,
            sclass: 0 as u16,
            op_code: 0 as u8,
            rd: false,
            slist: Slist::new(),
            sbelt: Slist::new(),
            cache: DnsCache::new(),
            ns_data: HashMap::<u16, HashMap<String, NSZone>>::new(),
            main_query_id: rng.gen(),
            old_id: 0,
            src_address: "".to_string(),
            add_channel_udp: add_channel_udp,
            delete_channel_udp: delete_channel_udp,
            add_channel_tcp: add_channel_tcp,
            delete_channel_tcp: delete_channel_tcp,
            add_channel_ns_udp: add_channel_ns_udp,
            delete_channel_ns_udp: delete_channel_ns_udp,
            add_channel_ns_tcp: add_channel_ns_tcp,
            delete_channel_ns_tcp: delete_channel_ns_tcp,
            queries_before_temporary_error: queries_before_temporary_error,
            tx_update_query: tx_update_query,
            tx_delete_query: tx_delete_query,
            client_msg: client_msg,
            index_to_choose: 0,
            last_query_timestamp: 0,
            timeout: 0,
            last_query_hostname: "".to_string(),
            update_cache_sender_udp: update_cache_sender_udp,
            update_cache_sender_tcp: update_cache_sender_tcp,
            update_cache_sender_ns_udp: update_cache_sender_ns_udp,
            update_cache_sender_ns_tcp: update_cache_sender_ns_tcp,
            internal_query: false,
            query_id_update_slist: 0,
            update_slist_tcp_sender: update_slist_tcp_sender,
            tx_update_self_slist: tx_update_self_slist,
        };

        query
    }

    // Initializes the resolver query
    pub fn initialize(
        &mut self,
        sname: String,
        stype: u16,
        sclass: u16,
        op_code: u8,
        rd: bool,
        sbelt: Slist,
        cache: DnsCache,
        ns_data: HashMap<u16, HashMap<String, NSZone>>,
        src_address: String,
        old_id: u16,
    ) {
        self.set_sname(sname);
        self.set_stype(stype);
        self.set_sclass(sclass);
        self.set_op_code(op_code);
        self.set_rd(rd);
        self.set_sbelt(sbelt);
        self.set_cache(cache);
        self.set_ns_data(ns_data);
        self.set_src_address(src_address);
        self.set_old_id(old_id);
    }

    pub fn initialize_slist_udp(
        &mut self,
        sbelt: Slist,
        start_look_up_host_name: String,
        socket: UdpSocket,
    ) {
        let host_name = start_look_up_host_name;
        let mut cache = self.get_cache();
        let ns_type = "NS".to_string();
        let host_name_copy = host_name.clone();
        let mut labels: Vec<&str> = host_name_copy.split('.').collect();
        let mut new_slist = Slist::new();

        while labels.len() > 0 {
            let mut parent_host_name = "".to_string();

            for label in labels.iter() {
                parent_host_name.push_str(label);
                parent_host_name.push_str(".");
            }

            parent_host_name.pop();

            println!("Hostname in slist: {}", parent_host_name.clone());

            // Gets a vector of NS RR for host_name
            let ns_parent_host_name = cache.get(parent_host_name.to_string(), ns_type.clone());

            println!("Found {} NS records", ns_parent_host_name.len());

            let mut ip_found = 0;

            for ns in ns_parent_host_name.clone() {
                let rr_rdata = match ns.get_resource_record().get_rdata() {
                    Rdata::SomeNsRdata(val) => val.clone(),
                    _ => unreachable!(),
                };

                let ns_parent_host_name_string = rr_rdata.get_nsdname().get_name();

                new_slist.set_zone_name_equivalent(labels.len() as i32);

                // Gets list of ip addresses
                let ns_ip_address = cache.get(ns_parent_host_name_string.clone(), "A".to_string());

                if ns_ip_address.len() == 0 {
                    new_slist.insert(ns_parent_host_name_string, "".to_string(), 6000);
                    continue;
                }

                for ip in ns_ip_address.clone() {
                    let ns_ip_address_rdata = match ip.get_resource_record().get_rdata() {
                        Rdata::SomeARdata(val) => val.clone(),
                        _ => unreachable!(),
                    };

                    let ip_address = ns_ip_address_rdata.get_string_address();

                    let response_time = cache.get_response_time(
                        ns_parent_host_name_string.clone(),
                        "A".to_string(),
                        ip_address.clone(),
                    );

                    new_slist.insert(
                        ns_parent_host_name_string.clone(),
                        ip_address.to_string(),
                        response_time as u32,
                    );
                    ip_found = ip_found + 1;
                }
            }

            println!("Ip found: {}", ip_found);

            if ip_found == 0 {
                if new_slist.len() > 0
                    && new_slist
                        .get_first()
                        .get(&"name".to_string())
                        .unwrap()
                        .contains(&parent_host_name)
                        == false
                {
                    self.send_internal_queries_for_slist_udp(
                        new_slist.clone(),
                        socket.try_clone().unwrap(),
                    );

                    break;
                }

                new_slist = Slist::new();
                labels.remove(0);
                continue;
            }

            break;
        }

        if new_slist.get_zone_name_equivalent() == -1 {
            self.set_slist(sbelt.clone());
        } else {
            self.set_slist(new_slist.clone());
        }
    }

    pub fn initialize_slist_tcp(&mut self, sbelt: Slist, start_look_up_host_name: String) {
        let host_name = start_look_up_host_name;
        let mut cache = self.get_cache();
        let ns_type = "NS".to_string();
        let host_name_copy = host_name.clone();
        let mut labels: Vec<&str> = host_name_copy.split('.').collect();
        let mut new_slist = Slist::new();

        while labels.len() > 0 {
            let mut parent_host_name = "".to_string();

            for label in labels.iter() {
                parent_host_name.push_str(label);
                parent_host_name.push_str(".");
            }

            parent_host_name.pop();

            println!("Hostname in slist: {}", parent_host_name.clone());

            // Gets a vector of NS RR for host_name
            let ns_parent_host_name = cache.get(parent_host_name.to_string(), ns_type.clone());

            println!("Found {} NS records", ns_parent_host_name.len());

            let mut ip_found = 0;

            for ns in ns_parent_host_name.clone() {
                let rr_rdata = match ns.get_resource_record().get_rdata() {
                    Rdata::SomeNsRdata(val) => val.clone(),
                    _ => unreachable!(),
                };

                let ns_parent_host_name_string = rr_rdata.get_nsdname().get_name();

                new_slist.set_zone_name_equivalent(labels.len() as i32);

                // Gets list of ip addresses
                let ns_ip_address = cache.get(ns_parent_host_name_string.clone(), "A".to_string());

                if ns_ip_address.len() == 0 {
                    new_slist.insert(ns_parent_host_name_string, "".to_string(), 6000);
                    continue;
                }

                for ip in ns_ip_address.clone() {
                    let ns_ip_address_rdata = match ip.get_resource_record().get_rdata() {
                        Rdata::SomeARdata(val) => val.clone(),
                        _ => unreachable!(),
                    };

                    let ip_address = ns_ip_address_rdata.get_string_address();

                    let response_time = cache.get_response_time(
                        ns_parent_host_name_string.clone(),
                        "A".to_string(),
                        ip_address.clone(),
                    );

                    new_slist.insert(
                        ns_parent_host_name_string.clone(),
                        ip_address.to_string(),
                        response_time as u32,
                    );
                    ip_found = ip_found + 1;
                }
            }

            println!("Ip found: {}", ip_found);

            if ip_found == 0 {
                if new_slist.len() > 0
                    && new_slist
                        .get_first()
                        .get(&"name".to_string())
                        .unwrap()
                        .contains(&parent_host_name)
                        == false
                {
                    self.send_internal_queries_for_slist_tcp(new_slist.clone());

                    break;
                }

                new_slist = Slist::new();
                labels.remove(0);
                continue;
            }

            break;
        }

        if new_slist.get_zone_name_equivalent() == -1 {
            self.set_slist(sbelt);
        } else {
            self.set_slist(new_slist);
        }
    }

    // Looks for local info in name server zone and cache
    pub fn look_for_local_info(&mut self) -> Vec<ResourceRecord> {
        let s_type = match self.get_stype() {
            1 => "A".to_string(),
            2 => "NS".to_string(),
            5 => "CNAME".to_string(),
            6 => "SOA".to_string(),
            11 => "WKS".to_string(),
            12 => "PTR".to_string(),
            13 => "HINFO".to_string(),
            14 => "MINFO".to_string(),
            15 => "MX".to_string(),
            16 => "TXT".to_string(),
            255 => "*".to_string(),
            _ => unreachable!(),
        };

        let s_name = self.get_sname();
        let s_class = self.get_sclass();

        // Class is *
        if s_class == 255 {
            let mut all_answers = Vec::new();

            for (class, hashzone) in self.get_ns_data().iter() {
                let (main_zone, available) = NameServer::search_nearest_ancestor_zone(
                    self.get_ns_data(),
                    s_name.clone(),
                    *class,
                );

                if available == true {
                    let mut sname_without_zone_label = s_name.replace(&main_zone.get_name(), "");

                    // We were looking for the first node
                    if sname_without_zone_label == "".to_string() {
                        let mut rrs_by_type = main_zone.get_rrs_by_type(self.get_stype());
                        let soa_rr = main_zone.get_rrs_by_type(6)[0].clone();
                        let soa_minimun_ttl = match soa_rr.get_rdata() {
                            Rdata::SomeSoaRdata(val) => val.get_minimum(),
                            _ => unreachable!(),
                        };

                        // Sets TTL to max between RR ttl and SOA min.
                        for rr in rrs_by_type.iter_mut() {
                            let rr_ttl = rr.get_ttl();

                            rr.set_ttl(cmp::max(rr_ttl, soa_minimun_ttl));
                        }

                        return rrs_by_type;
                    }

                    // Delete last dot
                    sname_without_zone_label.pop().unwrap();

                    let mut labels: Vec<&str> = sname_without_zone_label.split(".").collect();

                    labels.reverse();

                    let mut last_label = "";

                    let mut zone = main_zone.clone();

                    for label in labels {
                        let exist_child = zone.exist_child(label.to_string());

                        if exist_child == true {
                            zone = zone.get_child(label.to_string()).0;
                            last_label = label.clone();
                            continue;
                        }
                    }

                    if last_label == zone.get_name() {
                        let mut rrs_by_type = zone.get_rrs_by_type(self.get_stype());

                        let soa_rr = main_zone.get_rrs_by_type(6)[0].clone();
                        let soa_minimun_ttl = match soa_rr.get_rdata() {
                            Rdata::SomeSoaRdata(val) => val.get_minimum(),
                            _ => unreachable!(),
                        };

                        // Sets TTL to max between RR ttl and SOA min.
                        for rr in rrs_by_type.iter_mut() {
                            let rr_ttl = rr.get_ttl();

                            rr.set_ttl(cmp::max(rr_ttl, soa_minimun_ttl));
                        }

                        all_answers.append(&mut rrs_by_type);
                    }
                }
            }

            if all_answers.len() > 0 {
                return all_answers;
            }
        } else {
            let (main_zone, available) = NameServer::search_nearest_ancestor_zone(
                self.get_ns_data(),
                s_name.clone(),
                s_class,
            );

            if available == true {
                let mut sname_without_zone_label = s_name.replace(&main_zone.get_name(), "");

                // We were looking for the first node
                if sname_without_zone_label == "".to_string() {
                    let mut rrs_by_type = main_zone.get_rrs_by_type(self.get_stype());
                    let soa_rr = main_zone.get_rrs_by_type(6)[0].clone();
                    let soa_minimun_ttl = match soa_rr.get_rdata() {
                        Rdata::SomeSoaRdata(val) => val.get_minimum(),
                        _ => unreachable!(),
                    };

                    // Sets TTL to max between RR ttl and SOA min.
                    for rr in rrs_by_type.iter_mut() {
                        let rr_ttl = rr.get_ttl();

                        rr.set_ttl(cmp::max(rr_ttl, soa_minimun_ttl));
                    }

                    return rrs_by_type;
                }

                // Delete last dot
                sname_without_zone_label.pop().unwrap();

                let mut labels: Vec<&str> = sname_without_zone_label.split(".").collect();

                labels.reverse();

                let mut last_label = "";

                let mut zone = main_zone.clone();

                for label in labels {
                    let exist_child = zone.exist_child(label.to_string());

                    if exist_child == true {
                        zone = zone.get_child(label.to_string()).0;
                        last_label = label.clone();
                        continue;
                    }
                }

                if last_label == zone.get_name() {
                    let mut rrs_by_type = zone.get_rrs_by_type(self.get_stype());

                    let soa_rr = main_zone.get_rrs_by_type(6)[0].clone();
                    let soa_minimun_ttl = match soa_rr.get_rdata() {
                        Rdata::SomeSoaRdata(val) => val.get_minimum(),
                        _ => unreachable!(),
                    };

                    // Sets TTL to max between RR ttl and SOA min.
                    for rr in rrs_by_type.iter_mut() {
                        let rr_ttl = rr.get_ttl();

                        rr.set_ttl(cmp::max(rr_ttl, soa_minimun_ttl));
                    }

                    return rrs_by_type;
                }
            }
        }

        let mut rr_vec = Vec::<ResourceRecord>::new();

        if USE_CACHE == true {
            let mut cache = self.get_cache();

            let cache_answer = cache.get(s_name.clone(), s_type);
            let mut rrs_cache_answer = Vec::new();

            if s_class != 255 {
                for rr in cache_answer {
                    let rr_class = rr.get_resource_record().get_class();

                    if rr_class == s_class {
                        rrs_cache_answer.push(rr);
                    }
                }
            }

            if rrs_cache_answer.len() > 0 {
                for answer in rrs_cache_answer.iter() {
                    let mut rr = answer.get_resource_record();
                    let rr_ttl = rr.get_ttl();
                    let relative_ttl = rr_ttl - self.get_timestamp();

                    if relative_ttl > 0 {
                        rr.set_ttl(relative_ttl);
                        rr_vec.push(rr);
                    }
                }

                if rr_vec.len() < rrs_cache_answer.len() {
                    self.remove_from_cache(s_name, rrs_cache_answer[0].get_resource_record());
                }
            }
        }

        return rr_vec;
    }
}

// Util for TCP and UDP
impl ResolverQuery {
    pub fn step_2_tcp(&mut self) {
        let sbelt = self.get_sbelt();
        let sname = self.get_sname();
        self.initialize_slist_tcp(sbelt, sname);

        let mut slist = self.get_slist();
        slist.sort();

        println!("Slist intia len: {}", slist.len());

        self.set_slist(slist);
    }

    pub fn step_2_udp(&mut self, socket: UdpSocket) {
        let sbelt = self.get_sbelt();
        let sname = self.get_sname();
        self.initialize_slist_udp(sbelt, sname, socket);

        let mut slist = self.get_slist();
        slist.sort();

        self.set_slist(slist);

        self.get_tx_update_query().send(self.clone());
    }

    pub fn step_4a(&mut self, msg: DnsMessage) -> DnsMessage {
        let mut answer = msg.get_answer();
        let rcode = msg.get_header().get_rcode();

        let aa = msg.get_header().get_aa();

        if rcode == 0 {
            // Get qname
            let qname = msg.get_question().get_qname().get_name();

            // Check if qnanem contains *, if its true dont cache the data
            if qname.contains("*") == false {
                if aa == true {
                    let mut remove_exist_cache = true;
                    for an in answer.iter_mut() {
                        if an.get_ttl() > 0 && an.get_type_code() == self.get_stype() {
                            an.set_ttl(an.get_ttl() + self.get_timestamp());

                            // Remove old cache
                            if remove_exist_cache == true {
                                self.remove_from_cache(an.get_name().get_name(), an.clone());
                                remove_exist_cache = false;
                            }

                            // Add new Cache
                            self.add_to_cache(an.get_name().get_name(), an.clone());
                        }
                    }
                } else {
                    let exist_in_cache = self.exist_cache_data(
                        msg.get_question().get_qname().get_name(),
                        answer[0].clone(),
                    );

                    if exist_in_cache == false {
                        for an in answer.iter_mut() {
                            if an.get_ttl() > 0 && an.get_type_code() == self.get_stype() {
                                an.set_ttl(an.get_ttl() + self.get_timestamp());

                                // Cache
                                self.add_to_cache(an.get_name().get_name(), an.clone());
                            }
                        }
                    }
                }
            }
        }

        return msg;
    }
}

// Utils for Udp
impl ResolverQuery {
    fn send_udp_query(&self, msg: &[u8], ip_address: String, socket: UdpSocket) {
        socket
            .send_to(msg, ip_address)
            .expect("failed to send message");
    }

    pub fn step_1_udp(
        &mut self,
        socket: UdpSocket,
        rx_update_self_slist: Receiver<Slist>,
    ) -> Option<Vec<ResourceRecord>> {
        let local_info = self.look_for_local_info();

        if local_info.len() > 0 {
            return Some(local_info);
        } else {
            self.step_2_udp(socket.try_clone().unwrap());
            self.step_3_udp(socket, rx_update_self_slist);
            return None;
        }
    }

    pub fn step_3_udp(&mut self, socket: UdpSocket, rx_update_self_slist: Receiver<Slist>) {
        let queries_left = self.get_queries_before_temporary_error();

        // Temporary Error
        if queries_left <= 0 {
            let tx_delete_query = self.get_tx_delete_query();
            tx_delete_query.send(self.clone());
            panic!("Temporary Error");
        }

        let mut slist = self.get_slist();
        let mut slist_len = slist.len();

        let mut index_to_choose = self.get_index_to_choose() % slist_len as u16;

        let mut best_server_to_ask = slist.get(index_to_choose);
        let mut best_server_ip = best_server_to_ask
            .get(&"ip_address".to_string())
            .unwrap()
            .clone();

        let mut counter = 0;

        while &best_server_ip == "" {
            if counter > slist.len() {
                let mut received_update_slist = rx_update_self_slist.try_iter();

                let mut next_slist_value = received_update_slist.next();

                while next_slist_value.is_none() == false {
                    println!("Habia un valor kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
                    let new_slist = next_slist_value.unwrap();

                    self.set_slist(new_slist);

                    next_slist_value = received_update_slist.next();
                }
            }

            slist = self.get_slist();
            self.set_index_to_choose((index_to_choose + 1) % slist.len() as u16);
            index_to_choose = self.get_index_to_choose();

            best_server_to_ask = slist.get(index_to_choose);
            best_server_ip = best_server_to_ask
                .get(&"ip_address".to_string())
                .unwrap()
                .clone();
            counter = counter + 1;
        }

        // Set query timeout
        let response_time = best_server_to_ask
            .get(&"response_time".to_string())
            .unwrap();

        self.set_timeout(response_time.parse::<u32>().unwrap() * 1.5 as u32);

        //

        best_server_ip.push_str(":53");

        // Update the index to choose
        self.set_index_to_choose((index_to_choose + 1) % slist.len() as u16);
        //

        // Implementar: se deben consultar las ips de los ns que no tienen ips

        self.send_internal_queries_for_slist_udp(self.get_slist(), socket.try_clone().unwrap());

        ///////////////////////////////////////////////

        let query_msg = self.create_query_message();
        let msg_to_bytes = query_msg.to_bytes();

        println!("Server to ask {}", best_server_ip);
        println!(
            "Asking for: {}",
            query_msg.get_question().get_qname().get_name()
        );

        // Update the queries count before temporary error
        self.set_queries_before_temporary_error(queries_left - 1);

        //

        // Set query timestamp
        let now = Utc::now();
        let timestamp_query = now.timestamp_millis();

        self.set_last_query_timestamp(timestamp_query as u64);

        //

        // Set last host name asked
        let host_name = best_server_to_ask.get(&"name".to_string()).unwrap().clone();
        self.set_last_query_hostname(host_name);
        //

        // Send the resolver query to the resolver for update
        self.get_tx_update_query().send(self.clone());
        //

        self.send_udp_query(&msg_to_bytes, best_server_ip, socket);
    }

    pub fn step_4_udp(
        &mut self,
        msg_from_response: DnsMessage,
        socket: UdpSocket,
        rx_update_self_slist: Receiver<Slist>,
    ) -> Option<DnsMessage> {
        let rcode = msg_from_response.get_header().get_rcode();
        let answer = msg_from_response.get_answer();

        // Step 4a
        if (answer.len() > 0 && rcode == 0 && answer[0].get_type_code() == self.get_stype())
            || rcode == 3
        {
            return Some(self.step_4a(msg_from_response));
        }

        let authority = msg_from_response.get_authority();

        // Step 4b
        // If there is authority and it is NS type
        if (authority.len() > 0) && (authority[0].get_type_code() == 2) {
            println!("Delegation response");
            self.step_4b_udp(msg_from_response, socket, rx_update_self_slist);
            return None;
        }

        // Step 4c
        // If the answer is CName and the user dont want CName
        if answer.len() > 0
            && answer[0].get_type_code() == 5
            && answer[0].get_type_code() != self.get_stype()
        {
            println!("Step 4C");
            return self.step_4c_udp(msg_from_response, socket, rx_update_self_slist);
        }

        let slist = self.get_slist();
        let index_to_choose = (self.get_index_to_choose() - 1) % slist.len() as u16;
        let best_server = slist.get(index_to_choose);
        let best_server_hostname = best_server.get(&"name".to_string()).unwrap();

        // Step 4d
        return self.step_4d_udp(
            best_server_hostname.to_string(),
            socket,
            rx_update_self_slist,
        );
    }

    pub fn step_4b_udp(
        &mut self,
        msg: DnsMessage,
        socket: UdpSocket,
        rx_update_self_slist: Receiver<Slist>,
    ) {
        let mut authority = msg.get_authority();
        let mut additional = msg.get_additional();

        // Adds NS and A RRs to cache if these can help
        let mut remove_exist_cache = true;
        for ns in authority.iter_mut() {
            if self.compare_match_count(ns.get_name().get_name()) {
                ns.set_ttl(ns.get_ttl() + self.get_timestamp());

                // Cache
                // Remove old cache
                if remove_exist_cache == true {
                    self.remove_from_cache(ns.get_name().get_name(), ns.clone());
                    remove_exist_cache = false;
                }

                // Add new cache
                self.add_to_cache(ns.get_name().get_name(), ns.clone());

                //

                // Get the NS domain name
                let ns_domain_name = match ns.get_rdata() {
                    Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                    _ => unreachable!(),
                };
                //

                let mut remove_exist_cache_ip = true;

                for ip in additional.iter_mut() {
                    if ns_domain_name == ip.get_name().get_name() {
                        ip.set_ttl(ip.get_ttl() + self.get_timestamp());

                        // Remove old cache
                        if remove_exist_cache_ip == true {
                            self.remove_from_cache(ip.get_name().get_name(), ip.clone());
                            remove_exist_cache_ip = false;
                        }

                        // Cache
                        self.add_to_cache(ip.get_name().get_name(), ip.clone());
                        //
                    }
                }
            }
        }

        self.step_2_udp(socket.try_clone().unwrap());
        self.step_3_udp(socket, rx_update_self_slist);
    }

    pub fn step_4c_udp(
        &mut self,
        mut msg: DnsMessage,
        socket: UdpSocket,
        rx_update_self_slist: Receiver<Slist>,
    ) -> Option<DnsMessage> {
        let answers = msg.get_answer();
        let mut resource_record = answers[0].clone();
        let rdata = resource_record.get_rdata();

        let rr_data = match rdata {
            Rdata::SomeCnameRdata(val) => val.clone(),
            _ => unreachable!(),
        };

        let cname = rr_data.get_cname();
        resource_record.set_ttl(resource_record.get_ttl() + self.get_timestamp());

        // Cache

        self.remove_from_cache(cname.get_name(), resource_record.clone());
        self.add_to_cache(cname.get_name(), resource_record);

        //

        // Check if contains the answer for cname

        if answers.len() > 1 {
            let cname_name = cname.get_name();
            let mut answers_found = 0;
            let qtype = self.get_stype();

            let mut answers_for_cname = Vec::<ResourceRecord>::new();

            for answer in answers[1..].into_iter() {
                let answer_name = answer.get_name().get_name();
                let answer_type = answer.get_type_code();

                if answer_name == cname_name && answer_type == qtype {
                    answers_found = answers_found + 1;
                    answers_for_cname.push(answer.clone());
                }
            }

            // Add to cache and return msg
            if answers_found > 0 {
                let mut msg_without_answer_cname = msg.clone();
                msg_without_answer_cname.set_answer(answers_for_cname);
                msg_without_answer_cname.update_header_counters();

                self.step_4a(msg_without_answer_cname);

                return Some(msg);
            }
        }

        //

        self.set_sname(cname.get_name());

        match self.step_1_udp(socket, rx_update_self_slist) {
            Some(val) => {
                println!("Local info!");

                msg.set_answer(val);
                msg.set_authority(Vec::new());
                msg.set_additional(Vec::new());

                let mut header = msg.get_header();
                header.set_ancount(answers.len() as u16);
                header.set_nscount(0);
                header.set_arcount(0);
                header.set_id(self.get_old_id());
                header.set_qr(true);

                msg.set_header(header);

                return Some(msg);
            }
            None => {
                return None;
            }
        }
    }

    pub fn step_4d_udp(
        &mut self,
        host_name_asked: String,
        socket: UdpSocket,
        rx_update_self_slist: Receiver<Slist>,
    ) -> Option<DnsMessage> {
        let mut slist = self.get_slist();
        slist.delete(host_name_asked.clone());

        if slist.len() == 0 {
            match host_name_asked.find(".") {
                Some(index) => {
                    let parent_host_name = &host_name_asked[index + 1..];
                    self.initialize_slist_udp(
                        self.get_sbelt(),
                        parent_host_name.to_string(),
                        socket.try_clone().unwrap(),
                    );
                    self.set_index_to_choose(0);
                }
                None => {
                    self.initialize_slist_udp(
                        self.get_sbelt(),
                        ".".to_string(),
                        socket.try_clone().unwrap(),
                    );
                    self.set_index_to_choose(0);
                }
            }
        } else {
            self.set_index_to_choose(self.get_index_to_choose() % slist.len() as u16);
            self.set_slist(slist);
        }

        // Update the query data in resolver
        self.get_tx_update_query().send(self.clone());
        //

        self.step_3_udp(socket, rx_update_self_slist);
        return None;
    }

    fn send_internal_queries_for_slist_udp(&self, slist: Slist, socket: UdpSocket) {
        println!("Entro a send_internal_queries");
        let ns_list = slist.get_ns_list();

        for ns in ns_list {
            let resolver_query_to_update = self.clone();
            let socket_copy = socket.try_clone().unwrap();
            let queries_left = self.get_queries_before_temporary_error();

            thread::spawn(move || {
                let ip_addr = ns.get(&"ip_address".to_string()).unwrap().to_string();
                let qname = ns.get(&"name".to_string()).unwrap().to_string();

                if ip_addr == "".to_string() {
                    println!("Internal Query para {}", qname.clone());
                    let mut rng = thread_rng();
                    let id: u16 = rng.gen();
                    let dns_msg = DnsMessage::new_query_message(qname.clone(), 1, 1, 0, false, id);

                    let tx_add_udp_copy = resolver_query_to_update.get_add_channel_udp();
                    let tx_delete_udp_copy = resolver_query_to_update.get_delete_channel_udp();
                    let tx_add_tcp_copy = resolver_query_to_update.get_add_channel_tcp();
                    let tx_delete_tcp_copy = resolver_query_to_update.get_delete_channel_tcp();
                    let tx_add_ns_udp_copy = resolver_query_to_update.get_add_channel_ns_udp();
                    let tx_delete_ns_udp_copy =
                        resolver_query_to_update.get_delete_channel_ns_udp();
                    let tx_add_ns_tcp_copy = resolver_query_to_update.get_add_channel_ns_tcp();
                    let tx_delete_ns_tcp_copy =
                        resolver_query_to_update.get_delete_channel_ns_tcp();
                    let tx_update_query_copy = resolver_query_to_update.get_tx_update_query();
                    let tx_delete_query_copy = resolver_query_to_update.get_tx_delete_query();
                    let tx_update_cache_udp_copy = resolver_query_to_update.get_update_cache_udp();
                    let tx_update_cache_tcp_copy = resolver_query_to_update.get_update_cache_tcp();
                    let tx_update_cache_ns_udp_copy =
                        resolver_query_to_update.get_update_cache_ns_udp();
                    let tx_update_cache_ns_tcp_copy =
                        resolver_query_to_update.get_update_cache_ns_tcp();

                    let (tx_update_slist_tcp, _rx_update_slist_tcp) = mpsc::channel();

                    let (tx_update_self_slist, rx_update_self_slist) = mpsc::channel();

                    let mut internal_query = ResolverQuery::new(
                        tx_add_udp_copy,
                        tx_delete_udp_copy,
                        tx_add_tcp_copy,
                        tx_delete_tcp_copy,
                        tx_add_ns_udp_copy,
                        tx_delete_ns_udp_copy,
                        tx_add_ns_tcp_copy,
                        tx_delete_ns_tcp_copy,
                        tx_update_query_copy.clone(),
                        tx_delete_query_copy,
                        dns_msg,
                        tx_update_cache_udp_copy,
                        tx_update_cache_tcp_copy,
                        tx_update_cache_ns_udp_copy,
                        tx_update_cache_ns_tcp_copy,
                        tx_update_slist_tcp,
                        tx_update_self_slist,
                    );

                    // Initializes the query data struct
                    internal_query.initialize(
                        qname,
                        1,
                        1,
                        0,
                        false,
                        resolver_query_to_update.get_sbelt(),
                        resolver_query_to_update.get_cache(),
                        resolver_query_to_update.get_ns_data(),
                        socket_copy.local_addr().unwrap().to_string(),
                        id,
                    );

                    internal_query.set_internal_query(true, queries_left - 1);
                    internal_query
                        .set_query_id_update_slist(resolver_query_to_update.get_main_query_id());

                    tx_update_query_copy.send(internal_query.clone());

                    internal_query.step_2_udp(socket_copy.try_clone().unwrap());
                    internal_query.step_3_udp(socket_copy, rx_update_self_slist);
                }
            });
        }
    }
}

// Utils for tcp
impl ResolverQuery {
    fn send_tcp_query(
        &mut self,
        msg: &[u8],
        ip_address: String,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        // Adds the two bytes needs for tcp
        let msg_length: u16 = msg.len() as u16;
        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg = [&tcp_bytes_length, msg].concat();

        // Timeout config
        let timeout = self.get_timeout();
        //

        let mut stream = TcpStream::connect(ip_address.clone()).unwrap();

        // Set timeout for read
        stream.set_read_timeout(Some(Duration::from_millis(timeout as u64)));

        stream.write(&full_msg);

        match Resolver::receive_tcp_msg(stream) {
            Some(val) => {
                let dns_response_result = DnsMessage::from_bytes(&val);

                match dns_response_result {
                    Ok(_) => {}
                    Err(e) => {
                        return DnsMessage::format_error_msg();
                    }
                }

                let dns_response = dns_response_result.unwrap();

                // Update response time in cache
                let last_query_timestamp = self.get_last_query_timestamp();
                let now = Utc::now();
                let timestamp_ms = now.timestamp_millis() as u64;

                let response_time = (timestamp_ms - last_query_timestamp) as u32;

                // Send request to update cache to resolver and name server
                self.get_update_cache_udp().send((
                    self.get_last_query_hostname(),
                    ip_address.clone(),
                    response_time,
                ));

                self.get_update_cache_tcp().send((
                    self.get_last_query_hostname(),
                    ip_address.clone(),
                    response_time,
                ));

                self.get_update_cache_ns_udp().send((
                    self.get_last_query_hostname(),
                    ip_address.clone(),
                    response_time,
                ));

                self.get_update_cache_ns_tcp().send((
                    self.get_last_query_hostname(),
                    ip_address.clone(),
                    response_time,
                ));
                //

                return self.step_4_tcp(dns_response, update_slist_tcp_recv);
            }
            None => return self.step_3_tcp(update_slist_tcp_recv),
        };
    }

    pub fn step_1_tcp(
        &mut self,
        mut query_msg: DnsMessage,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        let local_info = self.look_for_local_info();

        if local_info.len() > 0 {
            println!("Local info!");

            query_msg.set_answer(local_info.clone());
            query_msg.set_authority(Vec::new());
            query_msg.set_additional(Vec::new());

            let mut header = query_msg.get_header();
            header.set_ancount(local_info.len() as u16);
            header.set_nscount(0);
            header.set_arcount(0);
            header.set_id(self.get_old_id());
            header.set_qr(true);

            query_msg.set_header(header);

            return query_msg;
        } else {
            self.step_2_tcp();
            return self.step_3_tcp(update_slist_tcp_recv);
        }
    }

    pub fn step_3_tcp(
        &mut self,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        let queries_left = self.get_queries_before_temporary_error();

        // Temporary Error
        if queries_left <= 0 {
            panic!("Temporary Error");
        }

        let mut slist = self.get_slist();
        let mut index_to_choose = self.get_index_to_choose() % slist.len() as u16;
        let mut best_server_to_ask = slist.get(index_to_choose);
        let mut best_server_ip = best_server_to_ask
            .get(&"ip_address".to_string())
            .unwrap()
            .clone();

        let mut counter = 0;

        while &best_server_ip == "" {
            if counter > slist.len() {
                // Update slist
                let mut slists_to_update = update_slist_tcp_recv.try_iter();
                let mut next_slist_to_update = slists_to_update.next();

                while next_slist_to_update.is_none() == false {
                    let (host_name, answers) = next_slist_to_update.unwrap();

                    let mut slist_to_update = self.get_slist();
                    let mut ns_list_to_update = slist_to_update.get_ns_list();
                    let mut ns_index = 0;

                    for ns in ns_list_to_update.clone() {
                        let answers_copy = answers.clone();
                        let ns_name = ns.get(&"name".to_string()).unwrap().to_string();

                        if ns_name == host_name {
                            ns_list_to_update.remove(ns_index);

                            for answer in answers_copy {
                                let ip = match answer.get_rdata() {
                                    Rdata::SomeARdata(val) => val.get_string_address(),
                                    _ => unreachable!(),
                                };

                                let mut new_ns_to_ask = HashMap::new();

                                new_ns_to_ask.insert("name".to_string(), host_name.clone());
                                new_ns_to_ask.insert("ip_address".to_string(), ip);
                                new_ns_to_ask
                                    .insert("response_time".to_string(), "5000".to_string());

                                ns_list_to_update.push(new_ns_to_ask);
                            }
                        }
                        ns_index = ns_index + 1;
                    }

                    slist_to_update.set_ns_list(ns_list_to_update);

                    self.set_slist(slist_to_update);

                    next_slist_to_update = slists_to_update.next();
                }
                //
            }

            slist = self.get_slist();
            self.set_index_to_choose((index_to_choose + 1) % slist.len() as u16);
            index_to_choose = self.get_index_to_choose();

            best_server_to_ask = slist.get(index_to_choose);
            best_server_ip = best_server_to_ask
                .get(&"ip_address".to_string())
                .unwrap()
                .clone();
            counter = counter + 1;
        }

        // Set query timeout
        let response_time = best_server_to_ask
            .get(&"response_time".to_string())
            .unwrap();

        self.set_timeout(response_time.parse::<u32>().unwrap() * 1.5 as u32);

        //

        best_server_ip.push_str(":53");

        // Update the index to choose
        self.set_index_to_choose((index_to_choose + 1) % slist.len() as u16);
        //

        // Get address for empty ns in slist
        self.send_internal_queries_for_slist_tcp(self.get_slist());
        ///////////////////////////////////////////////

        let query_msg = self.create_query_message();
        let msg_to_bytes = query_msg.to_bytes();

        println!("Server to ask {}", best_server_ip);

        // Update the queries count before temporary error
        self.set_queries_before_temporary_error(queries_left - 1);

        //

        // Set query timestamp
        let now = Utc::now();
        let timestamp_query = now.timestamp_millis();
        self.set_last_query_timestamp(timestamp_query as u64);
        //

        // Set last host name asked
        let host_name = best_server_to_ask.get(&"name".to_string()).unwrap().clone();
        self.set_last_query_hostname(host_name);
        //

        return self.send_tcp_query(&msg_to_bytes, best_server_ip, update_slist_tcp_recv);
    }

    pub fn step_4_tcp(
        &mut self,
        msg_from_response: DnsMessage,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        let rcode = msg_from_response.get_header().get_rcode();
        let answer = msg_from_response.get_answer();

        // Step 4a
        if (answer.len() > 0 && rcode == 0 && answer[0].get_type_code() == self.get_stype())
            || rcode == 3
        {
            return self.step_4a(msg_from_response);
        }

        let authority = msg_from_response.get_authority();
        let additional = msg_from_response.get_additional();

        // Step 4b
        // If there is authority and it is NS type
        if (authority.len() > 0) && (authority[0].get_type_code() == 2) {
            return self.step_4b_tcp(msg_from_response, update_slist_tcp_recv);
        }

        // Step 4c
        // If the answer is CName and the user dont want CName
        if answer.len() > 0
            && answer[0].get_type_code() == 5
            && answer[0].get_type_code() != self.get_stype()
        {
            return self.step_4c_tcp(msg_from_response, update_slist_tcp_recv);
        }

        let slist = self.get_slist();
        let last_index_to_choose = (self.get_index_to_choose() - 1) % slist.len() as u16;
        let best_server = slist.get(last_index_to_choose);
        let best_server_hostname = best_server.get(&"name".to_string()).unwrap();

        // Step 4d
        return self.step_4d_tcp(best_server_hostname.to_string(), update_slist_tcp_recv);
    }

    pub fn step_4b_tcp(
        &mut self,
        msg: DnsMessage,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        let mut authority = msg.get_authority();
        let mut additional = msg.get_additional();

        let mut remove_exist_cache = true;
        // Adds NS and A RRs to cache if these can help
        for ns in authority.iter_mut() {
            if self.compare_match_count(ns.get_name().get_name()) {
                ns.set_ttl(ns.get_ttl() + self.get_timestamp());

                // Cache
                // Remove old cache
                if remove_exist_cache == true {
                    self.remove_from_cache(ns.get_name().get_name(), ns.clone());
                    remove_exist_cache = false;
                }

                // Add new cache
                self.add_to_cache(ns.get_name().get_name(), ns.clone());

                //

                // Get the NS domain name
                let ns_domain_name = match ns.get_rdata() {
                    Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                    _ => unreachable!(),
                };
                //

                let mut remove_exist_cache_ip = true;

                for ip in additional.iter_mut() {
                    if ns_domain_name == ip.get_name().get_name() {
                        ip.set_ttl(ip.get_ttl() + self.get_timestamp());

                        // Remove old cache
                        if remove_exist_cache_ip == true {
                            self.remove_from_cache(ip.get_name().get_name(), ip.clone());
                            remove_exist_cache_ip = false;
                        }

                        // Cache
                        self.add_to_cache(ip.get_name().get_name(), ip.clone());
                    }
                }
            }
        }

        self.step_2_tcp();
        return self.step_3_tcp(update_slist_tcp_recv);
    }

    pub fn step_4c_tcp(
        &mut self,
        mut msg: DnsMessage,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        let answer = msg.get_answer();
        let resource_record = answer[0].clone();
        let rdata = resource_record.get_rdata();

        let rr_data = match rdata {
            Rdata::SomeCnameRdata(val) => val.clone(),
            _ => unreachable!(),
        };

        let cname = rr_data.get_cname();

        // Cache
        self.remove_from_cache(cname.get_name(), resource_record.clone());
        self.add_to_cache(cname.get_name(), resource_record);

        self.set_sname(cname.get_name());

        return self.step_1_tcp(msg, update_slist_tcp_recv);
    }

    pub fn step_4d_tcp(
        &mut self,
        host_name_asked: String,
        update_slist_tcp_recv: Receiver<(String, Vec<ResourceRecord>)>,
    ) -> DnsMessage {
        let mut slist = self.get_slist();
        slist.delete(host_name_asked.clone());

        if slist.len() == 0 {
            match host_name_asked.find(".") {
                Some(index) => {
                    let parent_host_name = &host_name_asked[index + 1..];
                    self.initialize_slist_tcp(self.get_sbelt(), parent_host_name.to_string());
                    self.set_index_to_choose(0);
                }
                None => {
                    self.initialize_slist_tcp(self.get_sbelt(), ".".to_string());
                    self.set_index_to_choose(0);
                }
            }
        } else {
            self.set_index_to_choose(self.get_index_to_choose() % slist.len() as u16);
            self.set_slist(slist);
        }

        return self.step_3_tcp(update_slist_tcp_recv);
    }

    fn send_internal_queries_for_slist_tcp(&self, slist: Slist) {
        let ns_list = slist.get_ns_list();

        for ns in ns_list {
            let resolver_query_to_update = self.clone();
            let queries_left = self.get_queries_before_temporary_error();

            thread::spawn(move || {
                let ip_addr = ns.get(&"ip_address".to_string()).unwrap().to_string();
                let qname = ns.get(&"name".to_string()).unwrap().to_string();

                if ip_addr == "".to_string() {
                    let mut rng = thread_rng();
                    let id: u16 = rng.gen();
                    let dns_msg = DnsMessage::new_query_message(qname.clone(), 1, 1, 0, false, id);

                    let tx_add_udp_copy = resolver_query_to_update.get_add_channel_udp();
                    let tx_delete_udp_copy = resolver_query_to_update.get_delete_channel_udp();
                    let tx_add_tcp_copy = resolver_query_to_update.get_add_channel_tcp();
                    let tx_delete_tcp_copy = resolver_query_to_update.get_delete_channel_tcp();
                    let tx_add_ns_udp_copy = resolver_query_to_update.get_add_channel_ns_udp();
                    let tx_delete_ns_udp_copy =
                        resolver_query_to_update.get_delete_channel_ns_udp();
                    let tx_add_ns_tcp_copy = resolver_query_to_update.get_add_channel_ns_tcp();
                    let tx_delete_ns_tcp_copy =
                        resolver_query_to_update.get_delete_channel_ns_tcp();
                    let tx_update_query_copy = resolver_query_to_update.get_tx_update_query();
                    let tx_delete_query_copy = resolver_query_to_update.get_tx_delete_query();
                    let tx_update_cache_udp_copy = resolver_query_to_update.get_update_cache_udp();
                    let tx_update_cache_tcp_copy = resolver_query_to_update.get_update_cache_tcp();
                    let tx_update_cache_ns_udp_copy =
                        resolver_query_to_update.get_update_cache_ns_udp();
                    let tx_update_cache_ns_tcp_copy =
                        resolver_query_to_update.get_update_cache_ns_tcp();

                    let (update_slist_tcp_sender, update_slist_tcp_recv) = mpsc::channel();
                    let (tx_update_self_slist, rx_update_self_slist) = mpsc::channel();

                    let mut internal_query = ResolverQuery::new(
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
                        dns_msg,
                        tx_update_cache_udp_copy,
                        tx_update_cache_tcp_copy,
                        tx_update_cache_ns_udp_copy,
                        tx_update_cache_ns_tcp_copy,
                        update_slist_tcp_sender,
                        tx_update_self_slist,
                    );

                    // Initializes the query data struct
                    internal_query.initialize(
                        qname,
                        1,
                        1,
                        0,
                        false,
                        resolver_query_to_update.get_sbelt(),
                        resolver_query_to_update.get_cache(),
                        resolver_query_to_update.get_ns_data(),
                        "".to_string(),
                        id,
                    );

                    internal_query.set_internal_query(true, queries_left - 1);
                    internal_query
                        .set_query_id_update_slist(resolver_query_to_update.get_main_query_id());

                    internal_query.step_2_tcp();
                    let response_msg = internal_query.step_3_tcp(update_slist_tcp_recv);

                    // Update resolver query

                    let answers = response_msg.get_answer();
                    let host_name = answers[0].clone().get_name().get_name();

                    resolver_query_to_update
                        .get_update_slist_tcp_sender()
                        .send((host_name, answers));
                }
            });
        }
    }
}

// Others utils
impl ResolverQuery {
    // Add a new element to cache
    pub fn add_to_cache(&mut self, domain_name: String, resource_record: ResourceRecord) {
        println!(
            "-------------- Adding to cache: {} ------------------------",
            domain_name.clone()
        );
        let mut cache = self.get_cache();

        self.get_add_channel_udp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap_or(());
        self.get_add_channel_tcp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap_or(());
        self.get_add_channel_ns_udp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap_or(());
        self.get_add_channel_ns_tcp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap_or(());

        cache.add(domain_name, resource_record);

        self.set_cache(cache);
    }

    // Add a new element to cache
    pub fn remove_from_cache(&mut self, domain_name: String, resource_record: ResourceRecord) {
        let mut cache = self.get_cache();
        let rr_type = resource_record.get_string_type();

        self.get_delete_channel_udp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap();
        self.get_delete_channel_tcp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap();
        self.get_delete_channel_ns_udp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap();
        self.get_delete_channel_ns_tcp()
            .send((domain_name.clone(), resource_record.clone()))
            .unwrap();

        cache.remove(domain_name, rr_type);

        self.set_cache(cache);
    }

    pub fn exist_cache_data(
        &mut self,
        domain_name: String,
        resource_record: ResourceRecord,
    ) -> bool {
        let mut cache = self.get_cache();
        let rr_type = resource_record.get_string_type();

        let data_in_cache = cache.get(domain_name, rr_type);

        if data_in_cache.len() > 0 {
            return true;
        } else {
            return false;
        }
    }

    // Creates a new query dns message
    pub fn create_query_message(&mut self) -> DnsMessage {
        let sname = self.get_sname();
        let stype = self.get_stype();
        let sclass = self.get_sclass();
        let op_code = self.get_op_code();
        let rd = self.get_rd();
        let id = self.get_main_query_id();

        let query_message = DnsMessage::new_query_message(sname, stype, sclass, op_code, rd, id);

        query_message
    }

    // Compares the match count from slist with the given hostname
    pub fn compare_match_count(&self, name: String) -> bool {
        let slist_match_count = self.get_slist().get_zone_name_equivalent();
        let s_name_labels: String = self.get_sname();
        let mut s_name_labels_vec: Vec<&str> = s_name_labels.split('.').collect();
        let mut name_labels: Vec<&str> = name.split('.').collect();
        let min_len = cmp::min(s_name_labels.len(), name_labels.len());

        let mut name_match_count = 0;

        for _i in 0..min_len {
            let s_name_last_element = s_name_labels_vec[s_name_labels_vec.len() - 1];
            let name_last_element = name_labels[name_labels.len() - 1];
            if s_name_last_element == name_last_element {
                name_match_count = name_match_count + 1;
                s_name_labels_vec.pop();
                name_labels.pop();
            } else {
                break;
            }
        }

        if name_match_count > slist_match_count {
            return true;
        }

        return false;
    }
}

// Getters
impl ResolverQuery {
    /// Gets the timestamp
    pub fn get_timestamp(&self) -> u32 {
        self.timestamp.clone()
    }

    /// Gets the sname
    pub fn get_sname(&self) -> String {
        self.sname.clone()
    }

    /// Gets the stype
    pub fn get_stype(&self) -> u16 {
        self.stype
    }

    /// Gets the sclass
    pub fn get_sclass(&self) -> u16 {
        self.sclass
    }

    /// Gets the op_code
    pub fn get_op_code(&self) -> u8 {
        self.op_code
    }

    /// Gets the recursion desired bit
    pub fn get_rd(&self) -> bool {
        self.rd
    }

    /// Gets the slist
    pub fn get_slist(&self) -> Slist {
        self.slist.clone()
    }

    /// Gets the sbelt
    pub fn get_sbelt(&self) -> Slist {
        self.sbelt.clone()
    }

    /// Gets the cache
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }

    /// Gets the ns_data
    pub fn get_ns_data(&self) -> HashMap<u16, HashMap<String, NSZone>> {
        self.ns_data.clone()
    }

    /// Gets the main_query_id
    pub fn get_main_query_id(&self) -> u16 {
        self.main_query_id
    }

    /// Gets the old id
    pub fn get_old_id(&self) -> u16 {
        self.old_id
    }

    /// Get the owner's query address
    pub fn get_src_address(&self) -> String {
        self.src_address.clone()
    }

    /// Get the owner's query address
    pub fn get_add_channel_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_channel_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_channel_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_channel_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_channel_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_channel_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_channel_ns_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_channel_ns_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_channel_ns_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_channel_ns_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_ns_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_channel_ns_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_ns_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_channel_ns_tcp.clone()
    }

    /// Gets the queries before temporary error field
    pub fn get_queries_before_temporary_error(&self) -> u16 {
        self.queries_before_temporary_error
    }

    /// Gets the sender to update the resolver query in the resolver
    pub fn get_tx_update_query(&self) -> Sender<ResolverQuery> {
        self.tx_update_query.clone()
    }

    /// Gets the sender to delete the resolver query in the resolver
    pub fn get_tx_delete_query(&self) -> Sender<ResolverQuery> {
        self.tx_delete_query.clone()
    }

    /// Gets the index to choose from slist
    pub fn get_index_to_choose(&self) -> u16 {
        self.index_to_choose
    }

    /// Gets the last query timestamp
    pub fn get_last_query_timestamp(&self) -> u64 {
        self.last_query_timestamp
    }

    /// Gets the timeout for the actual query to name server
    pub fn get_timeout(&self) -> u32 {
        self.timeout
    }

    ///Gets the last query hostname
    pub fn get_last_query_hostname(&self) -> String {
        self.last_query_hostname.clone()
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

    /// Gets true if the query is an internal query
    pub fn get_internal_query(&self) -> bool {
        self.internal_query
    }

    /// Gets the query id from the slist to update
    pub fn get_query_id_update_slist(&self) -> u16 {
        self.query_id_update_slist
    }

    pub fn get_update_slist_tcp_sender(&self) -> Sender<(String, Vec<ResourceRecord>)> {
        self.update_slist_tcp_sender.clone()
    }

    pub fn get_tx_update_self_slist(&self) -> Sender<Slist> {
        self.tx_update_self_slist.clone()
    }
}

// Setters
impl ResolverQuery {
    /// Sets the timestamp attribute with a new value
    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    /// Sets the sname attribute with a new value
    pub fn set_sname(&mut self, sname: String) {
        self.sname = sname;
    }

    /// Sets the stype attribute with a new value
    pub fn set_stype(&mut self, stype: u16) {
        self.stype = stype;
    }

    /// Sets the sclass attribute with a new value
    pub fn set_sclass(&mut self, sclass: u16) {
        self.sclass = sclass;
    }

    /// Sets the op_code attribute with a new value
    pub fn set_op_code(&mut self, op_code: u8) {
        self.op_code = op_code;
    }

    /// Sets the rd attribute with a new value
    pub fn set_rd(&mut self, rd: bool) {
        self.rd = rd;
    }

    /// Sets the slist attribute with a new value
    pub fn set_slist(&mut self, slist: Slist) {
        self.slist = slist;
    }

    /// Sets the sbelt attribute with a new value
    pub fn set_sbelt(&mut self, sbelt: Slist) {
        self.sbelt = sbelt;
    }

    /// Sets the cache attribute with a new value
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }

    /// Sets the ns_data attribute with a new value
    pub fn set_ns_data(&mut self, ns_data: HashMap<u16, HashMap<String, NSZone>>) {
        self.ns_data = ns_data;
    }

    /// Sets the old id attribute with a new id
    pub fn set_main_query_id(&mut self, query_id: u16) {
        self.main_query_id = query_id;
    }

    /// Sets the old id attribute with a new id
    pub fn set_old_id(&mut self, query_id: u16) {
        self.old_id = query_id;
    }

    /// Sets the owner's query address
    pub fn set_src_address(&mut self, address: String) {
        self.src_address = address;
    }

    /// Sets the queries before temporary error field with a new value
    pub fn set_queries_before_temporary_error(&mut self, queries_before_temporary_error: u16) {
        self.queries_before_temporary_error = queries_before_temporary_error;
    }

    /// Sets the index to choose from slist with a new value
    pub fn set_index_to_choose(&mut self, index_to_choose: u16) {
        self.index_to_choose = index_to_choose;
    }

    /// Sets the timestamp for the last query for the request
    pub fn set_last_query_timestamp(&mut self, last_query_timestamp: u64) {
        self.last_query_timestamp = last_query_timestamp;
    }

    /// Sets the timeout for a query to name server
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    /// Sets the host name for the last query
    pub fn set_last_query_hostname(&mut self, last_query_hostname: String) {
        self.last_query_hostname = last_query_hostname;
    }

    /// Sets the query id to update the slist
    pub fn set_query_id_update_slist(&mut self, query_id_update_slist: u16) {
        self.query_id_update_slist = query_id_update_slist;
    }

    /// Sets the value for the internal query
    pub fn set_internal_query(&mut self, internal_query: bool, queries_left: u16) {
        self.internal_query = internal_query;
        self.queries_before_temporary_error = queries_left;
    }

    pub fn set_tx_update_self_slist(&mut self, tx_update_self_slist: Sender<Slist>) {
        self.tx_update_self_slist = tx_update_self_slist;
    }
}

mod resolver_query_tests {
    use crate::dns_cache::DnsCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;
    use crate::resolver::resolver_query::ResolverQuery;
    use crate::resolver::slist::Slist;

    use chrono::Utc;
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

        let (tx_update_query, rx_update_query) = mpsc::channel();
        let (tx_delete_query, rx_delete_query) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.sname, "".to_string());
        assert_eq!(resolver_query.stype, 0);
        assert_eq!(resolver_query.sclass, 0);
        assert_eq!(resolver_query.slist.get_ns_list().len(), 0);
        assert_eq!(resolver_query.cache.clone().get_size(), 0);
    }

    #[test]
    fn set_and_get_timestamp() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        let now = Utc::now();
        let now_timestamp = now.timestamp() as u32;

        resolver_query.set_timestamp(now_timestamp);

        assert_eq!(resolver_query.get_timestamp(), now_timestamp);
    }

    #[test]
    fn set_and_get_sname() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.sname, "".to_string());

        resolver_query.set_sname("test.com".to_string());

        assert_eq!(resolver_query.get_sname(), "test.com".to_string());
    }

    #[test]
    fn set_and_get_stype() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.stype, 0);

        resolver_query.set_stype(1);

        assert_eq!(resolver_query.get_stype(), 1);
    }

    #[test]
    fn set_and_get_sclass() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.sclass, 0);

        resolver_query.set_sclass(1);

        assert_eq!(resolver_query.get_sclass(), 1);
    }

    #[test]
    fn set_and_get_op_code() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.op_code, 0);

        resolver_query.set_op_code(1);

        assert_eq!(resolver_query.get_op_code(), 1);
    }

    #[test]
    fn set_and_get_rd() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.rd, false);

        resolver_query.set_rd(true);

        assert_eq!(resolver_query.get_rd(), true);
    }

    #[test]
    fn set_and_get_slist() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        let mut slist = Slist::new();

        assert_eq!(resolver_query.slist.get_ns_list().len(), 0);

        slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5000);
        resolver_query.set_slist(slist);

        assert_eq!(resolver_query.get_slist().get_ns_list().len(), 1);
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

        let (tx_update_query, rx_update_query) = mpsc::channel();
        let (tx_delete_query, rx_delete_query) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        let mut sbelt = Slist::new();

        assert_eq!(resolver_query.sbelt.get_ns_list().len(), 0);

        sbelt.insert("test.com".to_string(), "127.0.0.1".to_string(), 5000);
        resolver_query.set_sbelt(sbelt);

        assert_eq!(resolver_query.get_sbelt().get_ns_list().len(), 1);
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

        let (tx_update_query, rx_update_query) = mpsc::channel();
        let (tx_delete_query, rx_delete_query) = mpsc::channel();

        let (tx_update_cache_udp, rx_update_cache_udp) = mpsc::channel();
        let (tx_update_cache_tcp, rx_update_cache_tcp) = mpsc::channel();
        let (tx_update_cache_ns_udp, rx_update_cache_ns_udp) = mpsc::channel();
        let (tx_update_cache_ns_tcp, rx_update_cache_ns_tcp) = mpsc::channel();

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        let mut cache = DnsCache::new();
        cache.set_max_size(1);

        assert_eq!(resolver_query.cache.get_size(), 0);

        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);

        cache.add("127.0.0.0".to_string(), resource_record);
        resolver_query.set_cache(cache);

        assert_eq!(resolver_query.get_cache().get_size(), 1);
    }

    #[test]
    fn create_query_message_test() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        resolver_query.set_sname("test.com".to_string());
        resolver_query.set_rd(true);
        resolver_query.set_stype(1);
        resolver_query.set_sclass(1);

        let dns_message = resolver_query.create_query_message();

        assert_eq!(dns_message.get_header().get_rd(), true);
        assert_eq!(dns_message.get_question().get_qtype(), 1);
        assert_eq!(dns_message.get_question().get_qclass(), 1);
        assert_eq!(
            dns_message.get_question().get_qname().get_name(),
            "test.com".to_string()
        );
    }

    #[test]
    fn initialize_slist_test() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        resolver_query.set_sname("test.test2.com".to_string());
        resolver_query.set_rd(true);
        resolver_query.set_stype(1);
        resolver_query.set_sclass(1);

        let mut cache = DnsCache::new();
        cache.set_max_size(4);

        let mut domain_name = DomainName::new();
        domain_name.set_name("test2.com".to_string());

        let mut ns_rdata = NsRdata::new();
        ns_rdata.set_nsdname(domain_name);

        let r_data = Rdata::SomeNsRdata(ns_rdata);
        let mut ns_resource_record = ResourceRecord::new(r_data);
        ns_resource_record.set_type_code(2);

        let mut a_rdata = ARdata::new();
        a_rdata.set_address([127, 0, 0, 1]);

        let r_data = Rdata::SomeARdata(a_rdata);

        let mut a_resource_record = ResourceRecord::new(r_data);
        a_resource_record.set_type_code(1);

        cache.add("test2.com".to_string(), ns_resource_record);

        cache.add("test2.com".to_string(), a_resource_record);

        resolver_query.set_cache(cache);

        assert_eq!(resolver_query.get_slist().get_ns_list().len(), 0);

        let mut sbelt = Slist::new();
        sbelt.insert("test4.com".to_string(), "190.0.0.1".to_string(), 5000);

        resolver_query.initialize_slist_tcp(sbelt, resolver_query.get_sname());

        assert_eq!(resolver_query.get_slist().get_ns_list().len(), 1);

        assert_eq!(
            resolver_query
                .get_slist()
                .get_first()
                .get(&"name".to_string())
                .unwrap(),
            &"test2.com".to_string()
        );
    }

    #[test]
    fn initialize_slist_empty_test() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        resolver_query.set_sname("test6.test4.com".to_string());
        resolver_query.set_rd(true);
        resolver_query.set_stype(1);
        resolver_query.set_sclass(1);

        let mut cache = DnsCache::new();
        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name("test2.com".to_string());

        let mut ns_rdata = NsRdata::new();
        ns_rdata.set_nsdname(domain_name);

        let r_data = Rdata::SomeNsRdata(ns_rdata);
        let mut ns_resource_record = ResourceRecord::new(r_data);
        ns_resource_record.set_type_code(2);

        let mut a_rdata = ARdata::new();
        a_rdata.set_address([127, 0, 0, 1]);

        let r_data = Rdata::SomeARdata(a_rdata);

        let mut a_resource_record = ResourceRecord::new(r_data);
        a_resource_record.set_type_code(1);

        cache.add("test2.com".to_string(), ns_resource_record);

        cache.add("test2.com".to_string(), a_resource_record);

        resolver_query.set_cache(cache);

        assert_eq!(resolver_query.get_slist().get_ns_list().len(), 0);

        let mut sbelt = Slist::new();
        sbelt.insert("test4.com".to_string(), "190.0.0.1".to_string(), 5000);

        resolver_query.initialize_slist_tcp(sbelt, resolver_query.get_sname());

        assert_eq!(resolver_query.get_slist().get_ns_list().len(), 1);
        assert_eq!(
            resolver_query
                .get_slist()
                .get_first()
                .get(&"name".to_string())
                .unwrap(),
            &"test4.com".to_string()
        );
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query_test.get_ns_data().len(), 0);

        //resolver_query_test.set_ns_data(rr_type_hash);

        //assert_eq!(resolver_query_test.get_ns_data().len(), 1);
    }

    #[test]
    fn set_and_get_main_query_id() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        resolver_query.set_main_query_id(0);

        assert_eq!(resolver_query.get_main_query_id(), 0);
    }

    #[test]
    fn set_and_get_old_id() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.get_old_id(), 0);

        resolver_query.set_old_id(5);

        assert_eq!(resolver_query.get_old_id(), 5);
    }

    #[test]
    fn set_and_get_src_address() {
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

        let (tx_update_slist_tcp, rx_update_slist_tcp) = mpsc::channel();

        let mut resolver_query = ResolverQuery::new(
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
            tx_update_slist_tcp,
        );

        assert_eq!(resolver_query.get_src_address(), "".to_string());

        resolver_query.set_src_address(String::from("test.com"));

        assert_eq!(resolver_query.get_src_address(), "test.com".to_string());
    }
}
