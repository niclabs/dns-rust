use crate::message::resource_record::ResourceRecord;
use std::collections::HashMap;

#[derive(Clone)]
/// Struct that represents a cache for dns
pub struct DnsCache {
    cache: HashMap<String, ResourceRecord>,
}

impl DnsCache {
    /// Creates a new DnsCache with default values
    ///
    /// # Examples
    /// '''
    /// let cache = DnsCache::new();
    ///
    /// assert_eq!(cache.cache.len(), 0);
    /// '''
    ///
    pub fn new() -> Self {
        let cache = DnsCache {
            cache: HashMap::new(),
        };

        cache
    }

    /// Adds an element to cache
    pub fn add(&mut self, domain_name: String, resource_record: ResourceRecord) {
        let mut cache = self.get_cache();
        cache.insert(domain_name, resource_record);
        self.set_cache(cache);
    }

    /// Removes an element from cache
    pub fn remove(&mut self, domain_name: String) {
        let mut cache = self.get_cache();
        cache.remove(&domain_name);
        self.set_cache(cache);
    }

    /// Given a domain_name, gets an element from cache
    pub fn get(&mut self, domain_name: String) -> ResourceRecord {
        let cache = self.get_cache();
        match cache.get(&domain_name) {
            Some(val) => val.clone(),
            None => panic!("Can't get from cache"),
        }
    }

    /// Gets the cache length
    pub fn len(&mut self) -> usize {
        self.cache.len()
    }
}

// Getters
impl DnsCache {
    /// Gets the cache from the struct
    pub fn get_cache(&self) -> HashMap<String, ResourceRecord> {
        self.cache.clone()
    }
}

// Setters
impl DnsCache {
    /// Sets the cache
    pub fn set_cache(&mut self, cache: HashMap<String, ResourceRecord>) {
        self.cache = cache
    }
}

mod test {
    use crate::dns_cache::DnsCache;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use std::collections::HashMap;

    #[test]
    fn constructor_test() {
        let cache = DnsCache::new();

        assert_eq!(cache.cache.len(), 0);
    }

    #[test]
    fn set_and_get_cache_test() {
        let mut cache = DnsCache::new();

        assert_eq!(cache.get_cache().len(), 0);

        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);

        let resource_record = ResourceRecord::new(rdata);

        let mut test_cache = HashMap::new();
        test_cache.insert("test".to_string(), resource_record);

        cache.set_cache(test_cache);

        assert_eq!(cache.get_cache().len(), 1);
    }

    #[test]
    fn add_get_and_remove_test() {
        let mut cache = DnsCache::new();
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let resource_record = ResourceRecord::new(rdata);

        cache.add("127.0.0.0".to_string(), resource_record);

        assert_eq!(cache.len(), 1);

        cache.remove("127.0.0.0".to_string());

        assert_eq!(cache.len(), 0);
    }
}
