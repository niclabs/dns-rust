use crate::dns_cache::DnsCache;
use crate::message::resource_record::ResourceRecord;
use crate::resolver::slist::Slist;
use std::collections::HashMap;
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
        let resolver = Resolver {
            ip_address: String::from(""),
            port: String::from(""),
            sbelt: Slist::new(),
            cache: DnsCache::new(),
            ns_data: HashMap::<String, HashMap<String, Vec<ResourceRecord>>>::new(),
        };
        resolver
    }

    ////////////////////////////////////////
    // Al crear una nueva query, dejar sbelt como default de slist
    ////////////////////////////////////////
}

// Algorithm

// init answer
// if (IPv4/name is in authoritative form):
//      answer = search IPv4/name in slist in authoritative form
// if (config.check_cache):
//      answer = search IPv4/name in cache
// if (answer): return to client
// else:
//  init empty response
//  while not response
//      find [best] server in slist
//      send query of IPv4/name to server
//      if (response contains a name error) or (response ok):
//          return send response to client
//      if (better delegation to other servers):
//          cache delegation info.
//          continue
//      if (CNAME in response and CNAME is not answer):
//          add CNAME to cache
//          update SNAME to CNAME RR
//          call Algorithm
//      else:
//          delete server from slist
//          continue

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
