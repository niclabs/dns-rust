use crate::domain_cache::DomainCache;
use crate::message::resource_record::ResourceRecord;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
/// Struct that represents a cache for dns
pub struct DnsCache {
    cache: HashMap<String, DomainCache>,
    max_size: u32,
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
            cache: HashMap::<String, DomainCache>::new(),
            max_size: 0,
        };

        cache
    }

    /// Adds an element to cache
    pub fn add(&mut self, domain_name: String, resource_record: ResourceRecord) {
        let mut cache = self.get_cache();
        let rr_type = match resource_record.get_type_code() {
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

        // Obtengo el DomainCache
        if let Some(x) = cache.get_mut(&domain_name) {
            let mut new_type_in_cache = x.clone();

            let mut resource_records = new_type_in_cache.get_resource_records();

            resource_records.insert(rr_type, resource_record);
            new_type_in_cache.set_last_use(Utc::now());
            new_type_in_cache.set_resource_records(resource_records);

            cache.insert(domain_name, new_type_in_cache);
        } else {
            if cache.len() >= self.max_size as usize {
                let oldest_cache_used = self.get_oldest_domain_name_used();
                cache.remove(&oldest_cache_used);

                self.set_cache(cache);
            }

            let mut new_domain_cache = DomainCache::new();
            let mut resource_records = new_domain_cache.get_resource_records();

            resource_records.insert(rr_type, resource_record);
            new_domain_cache.set_resource_records(resource_records);
            new_domain_cache.set_last_use(Utc::now());

            cache = self.get_cache();
            cache.insert(domain_name, new_domain_cache);
        }

        self.set_cache(cache);
    }

    /// Removes an element from cache
    pub fn remove(&mut self, domain_name: String) {
        let mut cache = self.get_cache();
        cache.remove(&domain_name);
        self.set_cache(cache);
    }

    /// Given a domain_name, gets an element from cache
    pub fn get(&mut self, domain_name: String, rr_type: String) -> Vec<ResourceRecord> {
        let mut cache = self.get_cache();

        let domain_name_in_cache_empty = &mut DomainCache::new();

        let domain_name_in_cache = match cache.get_mut(&domain_name) {
            Some(val) => val,
            None => domain_name_in_cache_empty,
        };

        if domain_name_in_cache.get_resource_records().len() == 0 {
            return vec![];
        }

        let mut domain_cache = domain_name_in_cache.clone();

        domain_cache.set_last_use(Utc::now());

        *domain_name_in_cache = domain_cache;
        let resource_records = domain_name_in_cache.get_resource_records();

        match resource_records.get(&rr_type) {
            Some(val) => vec![val.clone()],
            None => vec![],
        }
    }

    /// Gets the cache length
    pub fn len(&mut self) -> usize {
        self.cache.len()
    }

    /// Gets the name of the domain that was the oldest used
    pub fn get_oldest_domain_name_used(&self) -> String {
        let cache = self.get_cache();

        let mut oldest_used = "".to_string();
        let mut used_in = Utc::now();

        for (key, value) in cache {
            if value.get_last_use() < used_in {
                oldest_used = key;
                used_in = value.get_last_use();
            }
        }

        oldest_used
    }
}

// Getters
impl DnsCache {
    /// Gets the cache from the struct
    pub fn get_cache(&self) -> HashMap<String, DomainCache> {
        self.cache.clone()
    }
}

// Setters
impl DnsCache {
    /// Sets the cache
    pub fn set_cache(&mut self, cache: HashMap<String, DomainCache>) {
        self.cache = cache
    }

    /// Sets the max size of the cache
    pub fn set_max_size(&mut self, max_size: u32) {
        self.max_size = max_size
    }
}

mod test {
    use crate::dns_cache::DnsCache;
    use crate::domain_cache::DomainCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
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
        let mut domain_name_in_cache = DomainCache::new();

        let mut resource_records = domain_name_in_cache.get_resource_records();
        resource_records.insert("A".to_string(), resource_record);
        domain_name_in_cache.set_resource_records(resource_records);

        test_cache.insert("test".to_string(), domain_name_in_cache);

        cache.set_cache(test_cache);

        assert_eq!(cache.get_cache().len(), 1);
    }

    #[test]
    fn add_get_and_remove_test() {
        let mut cache = DnsCache::new();

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

        /*
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);
        */

        cache.add("127.0.0.0".to_string(), ns_resource_record);

        assert_eq!(cache.len(), 1);

        cache.add("127.0.0.0".to_string(), a_resource_record);

        assert_eq!(
            cache.get("127.0.0.0".to_string(), "A".to_string())[0].get_type_code(),
            1
        );

        assert_eq!(
            cache.get("127.0.0.0".to_string(), "NS".to_string())[0].get_type_code(),
            2
        );

        cache.remove("127.0.0.0".to_string());

        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn add_domain_with_full_cache() {
        let mut cache = DnsCache::new();
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        cache.set_max_size(1);
        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);

        let second_resource_record = resource_record.clone();

        cache.add("127.0.0.0".to_string(), resource_record);

        assert_eq!(cache.len(), 1);

        cache.add("127.0.0.1".to_string(), second_resource_record);

        assert_eq!(cache.len(), 1);

        assert_eq!(
            cache.get("127.0.0.1".to_string(), "A".to_string())[0].get_type_code(),
            1
        )
    }
}
