use crate::dns_cache::DnsCache;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::resolver::slist::Slist;
use crate::resolver::Resolver;
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
    sname: String,
    stype: u16,
    sclass: u16,
    op_code: u8,
    rd: bool,
    slist: Slist,
    sbelt: Slist,
    cache: DnsCache,
    ns_data: HashMap<String, HashMap<String, Vec<ResourceRecord>>>,
    main_query_id: u16,
    src_address: String,
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
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let query = ResolverQuery {
            sname: "".to_string(),
            stype: 0 as u16,
            sclass: 0 as u16,
            op_code: 0 as u8,
            rd: false,
            slist: Slist::new(),
            sbelt: Slist::new(),
            cache: DnsCache::new(),
            ns_data: HashMap::<String, HashMap<String, Vec<ResourceRecord>>>::new(),
            main_query_id: rng.gen(),
            src_address: "".to_string(),
        };

        query
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

            // Gets a vector of NS RR for host_name
            let ns_parent_host_name = cache.get(parent_host_name.to_string(), ns_type.clone());

            /*
            if ns_parent_host_name.len() == 0 {
                labels.remove(0);
                continue;
            }
            */

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
                    new_slist.insert(ns_parent_host_name_string, "".to_string(), 6.0);
                    continue;
                }

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

    // Add a new element to cache
    pub fn add_to_cache(&mut self, domain_name: String, resource_record: ResourceRecord) {
        let mut cache = self.get_cache();

        cache.add(domain_name, resource_record);

        self.set_cache(cache);
    }

    pub fn look_for_local_info(&self) -> Vec<ResourceRecord> {
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

        if ns_data.len() > 0 {
            let rr_type_hash = match ns_data.get(&s_type) {
                Some(val) => val.clone(),
                None => HashMap::new(),
            };

            if rr_type_hash.len() > 0 {
                let host_names_vec = match rr_type_hash.get(&s_name) {
                    Some(val) => val.clone(),
                    None => Vec::new(),
                };

                // Por mientras
                return host_names_vec.clone();
            }
        }

        let mut cache = self.get_cache();

        let cache_answer = cache.get(s_name, s_type);

        let mut rr_vec = Vec::<ResourceRecord>::new();

        if cache_answer.len() > 0 {
            for answer in cache_answer.iter() {
                rr_vec.push(answer.get_resource_record());
            }
        }

        return rr_vec;
    }

    pub fn send_query_udp(&mut self, socket: UdpSocket) {
        let mut slist = self.get_slist();
        slist.sort();

        let best_server = slist.get_first(); //hashamp of server that responds faster
        let mut best_server_ip = best_server.get(&"ip_address".to_string()).unwrap().clone();

        /* Implementar: se deben consultar las ips de los ns que no tienen ips

        let mut ns_list_without_first = slist.get_ns_list();
        ns_list_without_first.remove(0);

        for ns in ns_list_without_first {
        }

        ///////////////////////////////////////////////
        ///
        */

        let query_msg = self.create_query_message();
        let msg_to_bytes = query_msg.to_bytes();

        best_server_ip.push_str(":53");

        println!("Server to ask {}", best_server_ip);

        self.send_udp_query(&msg_to_bytes, best_server_ip, socket);
    }

    pub fn send_query_tcp(&mut self, tx: Sender<(String, ResourceRecord)>) {
        let mut slist = self.get_slist();
        slist.sort();

        let best_server = slist.get_first(); //hashamp of server that responds faster
        let mut best_server_ip = best_server.get(&"ip_address".to_string()).unwrap().clone();

        // Implementar: se deben consultar las ips de los ns que no tienen ips

        ///////////////////////////////////////////////

        let query_msg = self.create_query_message();
        let msg_to_bytes = query_msg.to_bytes();

        best_server_ip.push_str(":53");

        println!("Server to ask {}", best_server_ip);

        self.send_tcp_query(&msg_to_bytes, best_server_ip, tx);
    }

    fn send_udp_query(&self, msg: &[u8], ip_address: String, socket: UdpSocket) {
        socket
            .send_to(msg, ip_address)
            .expect("failed to send message");
    }

    fn send_tcp_query(
        &mut self,
        msg: &[u8],
        ip_address: String,
        tx: Sender<(String, ResourceRecord)>,
    ) {
        let mut stream = TcpStream::connect(ip_address.clone()).unwrap();
        stream.write(msg);

        let mut received_msg = [0; 512];
        stream.read(&mut received_msg);

        let dns_response = DnsMessage::from_bytes(&received_msg);

        let response = match self.process_answer_tcp(dns_response, tx) {
            Some(val) => vec![val],
            None => Vec::new(),
        };

        if response.len() > 0 {
            Resolver::send_answer_by_tcp(response[0].clone(), self.get_src_address());
        }
    }

    pub fn process_answer_udp(
        &mut self,
        msg_from_response: DnsMessage,
        socket: UdpSocket,
        tx: Sender<(String, ResourceRecord)>,
    ) -> Option<DnsMessage> {
        let rcode = msg_from_response.get_header().get_rcode();
        let authority = msg_from_response.get_authority();
        let answer = msg_from_response.get_answer();
        let additional = msg_from_response.get_additional();

        if ((answer.len() > 0) && rcode == 0 && answer[0].get_type_code() == self.get_stype())
            || rcode == 3
        {
            if rcode == 0 {
                for an in answer.iter() {
                    if an.get_ttl() > 0 && an.get_type_code() == self.get_stype() {
                        tx.send((an.get_name().get_name(), an.clone())).unwrap();
                    }
                }
            }

            return Some(msg_from_response);
        }

        let mut slist = self.get_slist();
        let best_server = slist.get_first();
        let best_server_hostname = best_server.get(&"name".to_string()).unwrap();

        // If there is authority and it is NS type
        if (authority.len() > 0) && (authority[0].get_type_code() == 2) {
            let mut initialize_slist = false;

            // Adds NS and A RRs to cache if these can help
            for ns in authority.iter() {
                if self.compare_match_count(ns.get_name().get_name()) {
                    tx.send((ns.get_name().get_name(), ns.clone())).unwrap();
                    self.add_to_cache(ns.get_name().get_name(), ns.clone());

                    for ip in additional.iter() {
                        if ns.get_name().get_name() == ip.get_name().get_name() {
                            tx.send((ip.get_name().get_name(), ip.clone())).unwrap();
                            self.add_to_cache(ip.get_name().get_name(), ip.clone());
                            initialize_slist = true;
                        }
                    }
                }
            }

            // If RRs are added, reinitialize the slist
            if initialize_slist {
                self.initialize_slist(self.get_sbelt());
            } else {
                // Si no entrega una buena delegacion, se deberia eliminar el server de la slist? Para asi evitar preguntarle al mismo.
                slist.delete(best_server_hostname.clone());
                self.set_slist(slist.clone());
            }
            // cargarle los datos adecuados
            self.send_query_udp(socket.try_clone().unwrap());
        }

        // If the answer is CName and the user dont want CName
        if answer.len() > 0
            && answer[0].get_type_code() == 5
            && answer[0].get_type_code() != self.get_stype()
        {
            let resource_record = answer[0].clone();
            let rdata = resource_record.get_rdata();

            let rr_data = match rdata {
                Rdata::SomeCnameRdata(val) => val.clone(),
                _ => unreachable!(),
            };

            let cname = rr_data.get_cname();
            tx.send((cname.get_name(), resource_record.clone()))
                .unwrap();
            self.add_to_cache(cname.get_name(), resource_record);

            self.set_sname(cname.get_name());
            let rr_info = self.look_for_local_info();

            if rr_info.len() > 0 {
                let mut new_dns_msg = msg_from_response.clone();
                new_dns_msg.set_answer(rr_info.clone());
                new_dns_msg.set_authority(Vec::new());
                new_dns_msg.set_additional(Vec::new());

                let mut header = new_dns_msg.get_header();
                header.set_ancount(rr_info.len() as u16);

                return Some(new_dns_msg);
            } else {
                let sbelt = self.get_sbelt();
                self.initialize_slist(sbelt);
                self.send_query_udp(socket);
                return None;
            }
        } else {
            slist.delete(best_server_hostname.clone());
            self.set_slist(slist);
            self.send_query_udp(socket);
            return None;
        }
    }

    pub fn process_answer_tcp(
        &mut self,
        msg_from_response: DnsMessage,
        tx: Sender<(String, ResourceRecord)>,
    ) -> Option<DnsMessage> {
        let rcode = msg_from_response.get_header().get_rcode();
        let authority = msg_from_response.get_authority();
        let answer = msg_from_response.get_answer();
        let additional = msg_from_response.get_additional();

        if ((answer.len() > 0) && rcode == 0 && answer[0].get_type_code() == self.get_stype())
            || rcode == 3
        {
            if rcode == 0 {
                for an in answer.iter() {
                    if an.get_ttl() > 0 && an.get_type_code() == self.get_stype() {
                        self.add_to_cache(an.get_name().get_name(), an.clone());
                        tx.send((an.get_name().get_name(), an.clone())).unwrap();
                    }
                }
            }

            return Some(msg_from_response);
        }

        let mut slist = self.get_slist();
        let best_server = slist.get_first();
        let best_server_hostname = best_server.get(&"name".to_string()).unwrap();

        if (authority.len() > 0) && (authority[0].get_type_code() == 2) {
            let mut initialize_slist = false;

            // Adds NS and A RRs to cache if these can help
            for ns in authority.iter() {
                if self.compare_match_count(ns.get_name().get_name()) {
                    self.add_to_cache(ns.get_name().get_name(), ns.clone());
                    tx.send((ns.get_name().get_name(), ns.clone())).unwrap();

                    for ip in additional.iter() {
                        if ns.get_name().get_name() == ip.get_name().get_name() {
                            self.add_to_cache(ip.get_name().get_name(), ip.clone());
                            tx.send((ip.get_name().get_name(), ip.clone())).unwrap();
                            initialize_slist = true;
                        }
                    }
                }
            }

            // If RRs are added, reinitialize the slist
            if initialize_slist {
                self.initialize_slist(self.get_sbelt());
                let mut slist = self.get_slist();
                slist.sort();
                self.set_slist(slist.clone());
            } else {
                // Si no entrega una buena delegacion, se deberia eliminar el server de la slist? Para asi evitar preguntarle al mismo.
                slist.delete(best_server_hostname.clone());
                self.set_slist(slist.clone());
            }
            // cargarle los datos adecuados
            self.send_query_tcp(tx.clone());
        }

        if answer.len() > 0
            && answer[0].get_type_code() == 5
            && answer[0].get_type_code() != self.get_stype()
        {
            let resource_record = answer[0].clone();
            let rdata = resource_record.get_rdata();

            let rr_data = match rdata {
                Rdata::SomeCnameRdata(val) => val.clone(),
                _ => unreachable!(),
            };

            let cname = rr_data.get_cname();
            self.add_to_cache(cname.get_name(), resource_record.clone());
            tx.send((cname.get_name(), resource_record)).unwrap();
            self.set_sname(cname.get_name());
            let rr_info = self.look_for_local_info();

            if rr_info.len() > 0 {
                let mut new_dns_msg = msg_from_response.clone();
                new_dns_msg.set_answer(rr_info.clone());
                new_dns_msg.set_authority(Vec::new());
                new_dns_msg.set_additional(Vec::new());

                let mut header = new_dns_msg.get_header();
                header.set_ancount(rr_info.len() as u16);

                return Some(new_dns_msg);
            } else {
                self.send_query_tcp(tx);
                return None;
            }
        } else {
            slist.delete(best_server_hostname.clone());
            self.set_slist(slist);
            self.send_query_tcp(tx);
            return None;
        }
    }
}

// Getters
impl ResolverQuery {
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
    pub fn get_ns_data(&self) -> HashMap<String, HashMap<String, Vec<ResourceRecord>>> {
        self.ns_data.clone()
    }

    /// Gets the main_query_id
    pub fn get_main_query_id(&self) -> u16 {
        self.main_query_id
    }

    /// Get the owner's query address
    pub fn get_src_address(&self) -> String {
        self.src_address.clone()
    }

    //utility
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
    pub fn set_ns_data(&mut self, ns_data: HashMap<String, HashMap<String, Vec<ResourceRecord>>>) {
        self.ns_data = ns_data;
    }

    /// Sets the main query id attribute with a new id
    pub fn set_main_query_id(&mut self, query_id: u16) {
        self.main_query_id = query_id;
    }

    /// Sets the owner's query address
    pub fn set_src_address(&mut self, address: String) {
        self.src_address = address;
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
    use std::collections::HashMap;
    use std::vec::Vec;

    #[test]
    fn constructor_test() {
        let resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sname, "".to_string());
        assert_eq!(resolver_query.stype, 0);
        assert_eq!(resolver_query.sclass, 0);
        assert_eq!(resolver_query.slist.get_ns_list().len(), 0);
        assert_eq!(resolver_query.cache.clone().get_size(), 0);
    }

    #[test]
    fn set_and_get_sname() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sname, "".to_string());

        resolver_query.set_sname("test.com".to_string());

        assert_eq!(resolver_query.get_sname(), "test.com".to_string());
    }

    #[test]
    fn set_and_get_stype() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.stype, 0);

        resolver_query.set_stype(1);

        assert_eq!(resolver_query.get_stype(), 1);
    }

    #[test]
    fn set_and_get_sclass() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sclass, 0);

        resolver_query.set_sclass(1);

        assert_eq!(resolver_query.get_sclass(), 1);
    }

    #[test]
    fn set_and_get_op_code() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.op_code, 0);

        resolver_query.set_op_code(1);

        assert_eq!(resolver_query.get_op_code(), 1);
    }

    #[test]
    fn set_and_get_rd() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.rd, false);

        resolver_query.set_rd(true);

        assert_eq!(resolver_query.get_rd(), true);
    }

    #[test]
    fn set_and_get_slist() {
        let mut resolver_query = ResolverQuery::new();
        let mut slist = Slist::new();

        assert_eq!(resolver_query.slist.get_ns_list().len(), 0);

        slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5.0);
        resolver_query.set_slist(slist);

        assert_eq!(resolver_query.get_slist().get_ns_list().len(), 1);
    }

    #[test]
    fn set_and_get_sbelt() {
        let mut resolver_query = ResolverQuery::new();
        let mut sbelt = Slist::new();

        assert_eq!(resolver_query.sbelt.get_ns_list().len(), 0);

        sbelt.insert("test.com".to_string(), "127.0.0.1".to_string(), 5.0);
        resolver_query.set_sbelt(sbelt);

        assert_eq!(resolver_query.get_sbelt().get_ns_list().len(), 1);
    }

    #[test]
    fn set_and_get_cache() {
        let mut resolver_query = ResolverQuery::new();
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
        let mut resolver_query = ResolverQuery::new();

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
        let mut resolver_query = ResolverQuery::new();
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
        let mut resolver_query = ResolverQuery::new();
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

        let mut resolver_query_test = ResolverQuery::new();

        assert_eq!(resolver_query_test.get_ns_data().len(), 0);

        resolver_query_test.set_ns_data(rr_type_hash);

        assert_eq!(resolver_query_test.get_ns_data().len(), 1);
    }
}
