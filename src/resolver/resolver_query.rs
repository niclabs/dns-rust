use crate::dns_cache::DnsCache;
use crate::message::rdata::Rdata;
use crate::message::DnsMessage;
use crate::resolver::slist::Slist;

#[derive(Clone)]
/// This struct represents a resolver query
pub struct ResolverQuery {
    sname: String,
    stype: u16,
    sclass: u16,
    op_code: u8,
    rd: bool,
    slist: Slist,
    cache: DnsCache,
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
        let query = ResolverQuery {
            sname: "".to_string(),
            stype: 0 as u16,
            sclass: 0 as u16,
            op_code: 0 as u8,
            rd: false,
            slist: Slist::new(),
            cache: DnsCache::new(),
        };

        query
    }

    // Creates a new query dns message
    pub fn create_query_message(&self) -> DnsMessage {
        let sname = self.get_sname();
        let stype = self.get_stype();
        let sclass = self.get_sclass();
        let op_code = self.get_op_code();
        let rd = self.get_rd();

        let query_message = DnsMessage::new_query_message(sname, stype, sclass, op_code, rd);

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

        while labels.len() > 1 {
            let mut parent_host_name = "".to_string();

            labels.remove(0);

            for label in labels.iter() {
                parent_host_name.push_str(label);
                parent_host_name.push_str(".");
            }

            parent_host_name.pop();

            println!("Parent Host Name: {}", parent_host_name);

            // Gets a vector of NS RR for host_name
            let ns_parent_host_name = cache.get(parent_host_name.to_string(), ns_type.clone());

            println!("Ns Len: {}", ns_parent_host_name.clone().len());

            if ns_parent_host_name.len() == 0 {
                continue;
            }

            let mut ip_found = 0;

            for ns in ns_parent_host_name.clone() {
                let rr_rdata = match ns.get_rdata() {
                    Rdata::SomeNsRdata(val) => val.clone(),
                    _ => unreachable!(),
                };

                let ns_parent_host_name_string = rr_rdata.get_nsdname().get_name();

                // Gets list of ip addresses
                let ns_ip_address = cache.get(ns_parent_host_name_string.clone(), "A".to_string());

                println!("Ip Len: {}", ns_ip_address.clone().len());

                if ns_ip_address.len() == 0 {
                    continue;
                }

                let ns_ip_address_rdata = match ns_ip_address[0].get_rdata() {
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

                new_slist.set_zone_name_equivalent(labels.len() as i32 - 1);
                new_slist.insert(ns_parent_host_name_string, ip_address.to_string(), 5.0);
                ip_found = ip_found + 1;
            }

            if ip_found == 0 {
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

    /// Gets the cache
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
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

    /// Sets the cache attribute with a new value
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
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

    #[test]
    fn constructor_test() {
        let resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sname, "".to_string());
        assert_eq!(resolver_query.stype, 0);
        assert_eq!(resolver_query.sclass, 0);
        assert_eq!(resolver_query.slist.get_ns_list().len(), 0);
        assert_eq!(resolver_query.cache.clone().len(), 0);
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
    fn set_and_get_cache() {
        let mut resolver_query = ResolverQuery::new();
        let mut cache = DnsCache::new();

        assert_eq!(resolver_query.cache.len(), 0);

        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);

        cache.add("127.0.0.0".to_string(), resource_record);
        resolver_query.set_cache(cache);

        assert_eq!(resolver_query.get_cache().len(), 1);
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
}
