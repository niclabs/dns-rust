use crate::dns_cache::DnsCache;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::name_server::zone::NSZone;
use crate::name_server::NameServer;
use crate::resolver::slist::Slist;
use crate::resolver::Resolver;

use chrono::{DateTime, Utc};
use rand::{thread_rng, Rng};
use std::cmp;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::thread;
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
    ns_data: HashMap<String, NSZone>,
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
    ) -> Self {
        let mut rng = thread_rng();
        let now = Utc::now();
        let timestamp = now.timestamp() as u32;

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
            ns_data: HashMap::<String, NSZone>::new(),
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
        ns_data: HashMap<String, NSZone>,
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

    pub fn initialize_slist(&mut self, sbelt: Slist) {
        // Buscar NS de los ancentros del sname en el caché y agregarlos al slist
        // Agregar las ips conocidas de estos ns a la slist
        // Si no se tienen ips, se deben encontrar usando una query (mientras que con las ips disponibles voy preguntando por la respuesta para el usuario). A menos que no exista ninguna ip, en cuyo caso se debe reiniciar la slist, pero ahora con el ancestro siguiente
        // Finalmente agregar a la slist, información adicional para poder ordenar lo que esta en la slist, como por ej tiempo de respuesta, y porcentaje que ha respondido.
        // Si no hay info, entre 5 y 10 seg es un tiempo de peor caso
        let host_name = self.get_sname();
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

                /*
                if ns_ip_address.len() == 0 {
                    new_slist.insert(ns_parent_host_name_string, "".to_string(), 6.0);
                    continue;
                }
                */

                for ip in ns_ip_address.clone() {
                    let ns_ip_address_rdata = match ip.get_resource_record().get_rdata() {
                        Rdata::SomeARdata(val) => val.clone(),
                        _ => unreachable!(),
                    };

                    let int_ip_address = ns_ip_address_rdata.get_address();
                    let mut ip_address = "".to_string();

                    for num in int_ip_address.iter() {
                        ip_address.push_str(num.to_string().as_str());
                        ip_address.push_str(".");
                    }

                    ip_address.pop();

                    let response_time = cache
                        .get_response_time(ns_parent_host_name_string.clone(), "A".to_string());

                    new_slist.insert(
                        ns_parent_host_name_string.clone(),
                        ip_address.to_string(),
                        response_time as f32,
                    );
                    ip_found = ip_found + 1;
                }
            }

            if ip_found == 0 {
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
        let ns_data = self.get_ns_data();
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
            _ => unreachable!(),
        };

        let s_name = self.get_sname();

        let (mut main_zone, available) =
            NameServer::search_nearest_ancestor_zone(self.get_ns_data(), s_name.clone());

        let mut rr_vec = Vec::<ResourceRecord>::new();

        println!("Existe la zona en el resolver: {}", available);

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

        let mut cache = self.get_cache();

        let cache_answer = cache.get(s_name.clone(), s_type);

        if cache_answer.len() > 0 {
            for answer in cache_answer.iter() {
                let mut rr = answer.get_resource_record();
                let rr_ttl = rr.get_ttl();
                let relative_ttl = rr_ttl - self.get_timestamp();

                if relative_ttl > 0 {
                    rr.set_ttl(relative_ttl);
                    rr_vec.push(rr);
                }
            }

            if rr_vec.len() < cache_answer.len() {
                self.remove_from_cache(s_name, cache_answer[0].get_resource_record());
            }
        }

        return rr_vec;
    }
}

// Util for TCP and UDP
impl ResolverQuery {
    pub fn step_2(&mut self) {
        let sbelt = self.get_sbelt();
        self.initialize_slist(sbelt);

        let mut slist = self.get_slist();
        slist.sort();

        self.set_slist(slist);
    }

    pub fn step_4a(&mut self, msg: DnsMessage) -> DnsMessage {
        let mut answer = msg.get_answer();
        let rcode = msg.get_header().get_rcode();

        if rcode == 0 {
            for an in answer.iter_mut() {
                if an.get_ttl() > 0 && an.get_type_code() == self.get_stype() {
                    an.set_ttl(an.get_ttl() + self.get_timestamp());

                    // Cache
                    self.add_to_cache(an.get_name().get_name(), an.clone());
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

    pub fn step_1_udp(&mut self, socket: UdpSocket) -> Option<Vec<ResourceRecord>> {
        let local_info = self.look_for_local_info();

        if local_info.len() > 0 {
            return Some(local_info);
        } else {
            self.step_2();
            self.step_3_udp(socket);
            return None;
        }
    }

    pub fn step_3_udp(&mut self, socket: UdpSocket) {
        let best_server_to_ask = self.get_slist().get_first();
        let mut best_server_ip = best_server_to_ask
            .get(&"ip_address".to_string())
            .unwrap()
            .clone();

        best_server_ip.push_str(":53");

        // Implementar: se deben consultar las ips de los ns que no tienen ips

        ///////////////////////////////////////////////

        let query_msg = self.create_query_message();
        let msg_to_bytes = query_msg.to_bytes();

        println!("Server to ask {}", best_server_ip);

        self.send_udp_query(&msg_to_bytes, best_server_ip, socket);
    }

    pub fn step_4_udp(
        &mut self,
        msg_from_response: DnsMessage,
        socket: UdpSocket,
    ) -> Option<DnsMessage> {
        let rcode = msg_from_response.get_header().get_rcode();
        let answer = msg_from_response.get_answer();

        println!("{:?}", msg_from_response.get_answer().len());

        // Step 4a
        if (answer.len() > 0 && rcode == 0 && answer[0].get_type_code() == self.get_stype())
            || rcode == 3
        {
            return Some(self.step_4a(msg_from_response));
        }

        let authority = msg_from_response.get_authority();
        let additional = msg_from_response.get_additional();

        // Step 4b
        /// If there is authority and it is NS type
        if (authority.len() > 0) && (authority[0].get_type_code() == 2) {
            println!("Delegation response");
            self.step_4b_udp(msg_from_response, socket);
            return None;
        }

        // Step 4c
        /// If the answer is CName and the user dont want CName
        if answer.len() > 0
            && answer[0].get_type_code() == 5
            && answer[0].get_type_code() != self.get_stype()
        {
            return self.step_4c_udp(msg_from_response, socket);
        }

        let mut slist = self.get_slist();
        let best_server = slist.get_first();
        let best_server_hostname = best_server.get(&"name".to_string()).unwrap();

        // Step 4d
        return self.step_4d_udp(best_server_hostname.to_string(), socket);
    }

    pub fn step_4b_udp(&mut self, msg: DnsMessage, socket: UdpSocket) {
        let mut authority = msg.get_authority();
        let mut additional = msg.get_additional();

        // Adds NS and A RRs to cache if these can help
        for ns in authority.iter_mut() {
            if self.compare_match_count(ns.get_name().get_name()) {
                ns.set_ttl(ns.get_ttl() + self.get_timestamp());

                // Cache
                self.add_to_cache(ns.get_name().get_name(), ns.clone());
                //

                // Get the NS domain name
                let ns_domain_name = match ns.get_rdata() {
                    Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                    _ => unreachable!(),
                };
                //

                for ip in additional.iter_mut() {
                    if ns_domain_name == ip.get_name().get_name() {
                        ip.set_ttl(ip.get_ttl() + self.get_timestamp());

                        // Cache
                        self.add_to_cache(ip.get_name().get_name(), ip.clone());
                        //
                    }
                }
            }
        }

        self.step_2();
        self.step_3_udp(socket);
    }

    pub fn step_4c_udp(&mut self, mut msg: DnsMessage, socket: UdpSocket) -> Option<DnsMessage> {
        let answer = msg.get_answer();
        let mut resource_record = answer[0].clone();
        let rdata = resource_record.get_rdata();

        let rr_data = match rdata {
            Rdata::SomeCnameRdata(val) => val.clone(),
            _ => unreachable!(),
        };

        let cname = rr_data.get_cname();
        resource_record.set_ttl(resource_record.get_ttl() + self.get_timestamp());

        // Cache
        self.add_to_cache(cname.get_name(), resource_record);

        self.set_sname(cname.get_name());

        let resp = match self.step_1_udp(socket) {
            Some(val) => {
                println!("Local info!");

                msg.set_answer(val);
                msg.set_authority(Vec::new());
                msg.set_additional(Vec::new());

                let mut header = msg.get_header();
                header.set_ancount(answer.len() as u16);
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
        };
    }

    pub fn step_4d_udp(
        &mut self,
        host_name_asked: String,
        socket: UdpSocket,
    ) -> Option<DnsMessage> {
        let mut slist = self.get_slist();
        slist.delete(host_name_asked);
        self.set_slist(slist);
        self.step_3_udp(socket);
        return None;
    }
}

// Utils for tcp
impl ResolverQuery {
    fn send_tcp_query(&mut self, msg: &[u8], ip_address: String) -> DnsMessage {
        // Adds the two bytes needs for tcp
        let msg_length: u16 = msg.len() as u16;
        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg = [&tcp_bytes_length, msg].concat();

        let mut stream = TcpStream::connect(ip_address.clone()).unwrap();
        stream.write(&full_msg);

        let mut received_msg = Resolver::receive_tcp_msg(stream);

        let dns_response = DnsMessage::from_bytes(&received_msg);

        return self.step_4_tcp(dns_response);
    }

    pub fn step_1_tcp(&mut self, mut query_msg: DnsMessage) -> DnsMessage {
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
            self.step_2();
            return self.step_3_tcp();
        }
    }

    pub fn step_3_tcp(&mut self) -> DnsMessage {
        let best_server_to_ask = self.get_slist().get_first();
        let mut best_server_ip = best_server_to_ask
            .get(&"ip_address".to_string())
            .unwrap()
            .clone();

        best_server_ip.push_str(":53");

        // Implementar: se deben consultar las ips de los ns que no tienen ips

        ///////////////////////////////////////////////

        let query_msg = self.create_query_message();
        let msg_to_bytes = query_msg.to_bytes();

        println!("Server to ask {}", best_server_ip);

        return self.send_tcp_query(&msg_to_bytes, best_server_ip);
    }

    pub fn step_4_tcp(&mut self, msg_from_response: DnsMessage) -> DnsMessage {
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
        /// If there is authority and it is NS type
        if (authority.len() > 0) && (authority[0].get_type_code() == 2) {
            return self.step_4b_tcp(msg_from_response);
        }

        // Step 4c
        /// If the answer is CName and the user dont want CName
        if answer.len() > 0
            && answer[0].get_type_code() == 5
            && answer[0].get_type_code() != self.get_stype()
        {
            return self.step_4c_tcp(msg_from_response);
        }

        let mut slist = self.get_slist();
        let best_server = slist.get_first();
        let best_server_hostname = best_server.get(&"name".to_string()).unwrap();

        // Step 4d
        return self.step_4d_tcp(best_server_hostname.to_string());
    }

    pub fn step_4b_tcp(&mut self, msg: DnsMessage) -> DnsMessage {
        let mut authority = msg.get_authority();
        let mut additional = msg.get_additional();

        // Adds NS and A RRs to cache if these can help
        for ns in authority.iter_mut() {
            if self.compare_match_count(ns.get_name().get_name()) {
                ns.set_ttl(ns.get_ttl() + self.get_timestamp());

                // Cache
                self.add_to_cache(ns.get_name().get_name(), ns.clone());

                // Get the NS domain name
                let ns_domain_name = match ns.get_rdata() {
                    Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                    _ => unreachable!(),
                };
                //

                for ip in additional.iter_mut() {
                    if ns_domain_name == ip.get_name().get_name() {
                        ip.set_ttl(ip.get_ttl() + self.get_timestamp());

                        // Cache
                        self.add_to_cache(ip.get_name().get_name(), ip.clone());
                    }
                }
            }
        }

        self.step_2();
        return self.step_3_tcp();
    }

    pub fn step_4c_tcp(&mut self, mut msg: DnsMessage) -> DnsMessage {
        let answer = msg.get_answer();
        let resource_record = answer[0].clone();
        let rdata = resource_record.get_rdata();

        let rr_data = match rdata {
            Rdata::SomeCnameRdata(val) => val.clone(),
            _ => unreachable!(),
        };

        let cname = rr_data.get_cname();

        // Cache
        self.add_to_cache(cname.get_name(), resource_record);

        self.set_sname(cname.get_name());

        return self.step_1_tcp(msg);
    }

    pub fn step_4d_tcp(&mut self, host_name_asked: String) -> DnsMessage {
        let mut slist = self.get_slist();
        slist.delete(host_name_asked);
        self.set_slist(slist);

        return self.step_3_tcp();
    }
}

// Others utils
impl ResolverQuery {
    // Add a new element to cache
    pub fn add_to_cache(&mut self, domain_name: String, resource_record: ResourceRecord) {
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
    pub fn get_ns_data(&self) -> HashMap<String, NSZone> {
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

    // utility
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
    pub fn set_ns_data(&mut self, ns_data: HashMap<String, NSZone>) {
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
}

mod resolver_query_tests {
    use crate::dns_cache::DnsCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::resolver::resolver_query::ResolverQuery;
    use crate::resolver::slist::Slist;
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::vec::Vec;
    use chrono::Utc;

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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
        );
        let mut slist = Slist::new();

        assert_eq!(resolver_query.slist.get_ns_list().len(), 0);

        slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5.0);
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
        );
        let mut sbelt = Slist::new();

        assert_eq!(resolver_query.sbelt.get_ns_list().len(), 0);

        sbelt.insert("test.com".to_string(), "127.0.0.1".to_string(), 5.0);
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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
        sbelt.insert("test4.com".to_string(), "190.0.0.1".to_string(), 5.0);

        resolver_query.initialize_slist(sbelt);

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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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
        sbelt.insert("test4.com".to_string(), "190.0.0.1".to_string(), 5.0);

        resolver_query.initialize_slist(sbelt);

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

        let mut resolver_query_test = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
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

        let mut resolver_query = ResolverQuery::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
        );

        assert_eq!(resolver_query.get_src_address(), "".to_string());

        resolver_query.set_src_address(String::from("test.com"));

        assert_eq!(resolver_query.get_src_address(), "test.com".to_string());
    }
}
