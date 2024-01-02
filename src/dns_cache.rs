pub mod cache_by_record_type;

use crate::dns_cache::cache_by_record_type::CacheByRecordType;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::rr_cache::RRStoredData;
use crate::message::type_rtype::Rtype;
use std::net::IpAddr;
use crate::domain_name::DomainName;
use std::cmp;

#[derive(Clone, Debug)]
/// Struct that represents a cache for dns
pub struct DnsCache {
    // first hash by type, then by hostname
    cache: CacheByRecordType,
    max_size: u32,
    size: u32,
}

impl DnsCache {
    /// Creates a new DnsCache with default values
    ///
    /// # Examples
    /// ```
    /// let cache = DnsCache::new();
    ///
    /// assert_eq!(cache.cache.len(), 0);
    /// ```
    ///
    pub fn new() -> Self {
        let cache = DnsCache {
            cache: CacheByRecordType::new(),
            max_size: 0,
            size: 0,
        };

        cache
    }

    /// Adds an element to cache
    pub fn add(&mut self, domain_name: DomainName, resource_record: ResourceRecord) {
        //See if max size is 0
        if self.max_size < 1 {
            return;
        }

        // see cache space
        if self.get_size() >= self.max_size {
            self.remove_oldest_used();
        }

        let rtype = resource_record.get_rtype();
        let rr_cache = RRStoredData::new(resource_record);

        let mut cache_data = self.get_cache();
        cache_data.add_to_cache_data(rtype, domain_name, rr_cache);
        self.set_cache(cache_data);
        self.set_size(self.get_size() + 1);
    }

    /// Add negative resource record type SOA to cache for negative answers
    pub fn add_negative_answer(&mut self, domain_name: DomainName, rtype: Rtype, resource_record:ResourceRecord) {
        
        // see cache space
        if self.get_size() >= self.max_size {
            self.remove_oldest_used();
        }

        let rr_cache = RRStoredData::new(resource_record);
        let mut cache_data = self.get_cache();
        cache_data.add_to_cache_data(rtype, domain_name, rr_cache);
        self.set_cache(cache_data);
        self.set_size(self.get_size() + 1);

    }

    /// Removes an element from cache
    pub fn remove(&mut self, domain_name: DomainName, rtype: Rtype) {
        let mut cache_data = self.get_cache();
        let length = cache_data.remove_from_cache_data(domain_name, rtype);
        //Size needs to be modified if something was removed
        self.set_cache(cache_data);
        self.set_size(cmp::max(self.get_size() - length as u32, 0));  
    }

    /// Given a domain_name, gets an element from cache
    pub fn get(&mut self, domain_name: DomainName, rtype: Rtype) -> Option<Vec<RRStoredData>> {
        let mut cache = self.get_cache();
        let rr_cache_vec = cache.get_from_cache_data(domain_name, rtype);
        self.set_cache(cache);
        return rr_cache_vec;
    }

    /// Removes the resource records from a domain name and type which were the oldest used
    pub fn remove_oldest_used(&mut self) {
        let mut cache = self.get_cache();

        let length = cache.remove_oldest_used();
        self.set_cache(cache);
        self.set_size(cmp::max(self.get_size() - length as u32, 0)); 
    }

    /// Gets the response time from a domain name and type resource record
    pub fn get_response_time(
        &mut self,
        domain_name: DomainName,
        rr_type: Rtype,
        ip_address: IpAddr,
    ) -> u32 {
        let rr_cache_vec = self.get(domain_name, rr_type).unwrap();

        for rr_cache in rr_cache_vec {
            let rr_ip_address = match rr_cache.get_resource_record().get_rdata() {
                Rdata::A(val) => val.get_address(),
                _ => unreachable!(),
            };
            
            if ip_address == rr_ip_address {
                return rr_cache.get_response_time();
            }
        }

        // Default response time in RFC 1034/1035
        return 5000;
    }

    // Gets the response time from a domain name and type resource record
    pub fn update_response_time(
        &mut self,
        domain_name: DomainName,
        rr_type: Rtype,
        response_time: u32,
        ip_address: IpAddr,
    ) {
        let mut cache = self.get_cache();

        cache.update_response_time(domain_name, rr_type, response_time, ip_address);

        self.set_cache(cache);
    }

    /// Checks if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.get_cache_data().is_empty()
    }

    /// Checks if a domain name is cached
    pub fn is_cached(&self, domain_name: DomainName, rtype: Rtype) -> bool {
        if let Some(mut host_data) = self.cache.get_cache_data().get(&rtype).cloned() {
            if let Some(_rrs) = host_data.get_from_host_data(domain_name) {
                return true;
            }
        }

        false
    }

    // TODO: Make print cache function
    // pub fn print(&self) {
    //     let cache = self.get_cache();

    //     for (key, val) in cache.iter() {
    //         println!("Type: {}", key);

    //         for (key2, _val2) in val.iter() {
    //             println!("Host Name: {}", key2);
    //         }
    //     }
    // }

    /// Performs the timeout of cache by removing the elements that have expired.
    /// 
    /// For each Resource Record in the cache, it checks if it has expired by its TTL.
    /// If it has expired, it removes it from the cache.
    pub fn timeout_cache(&mut self) {
        let mut cache = self.get_cache();
        cache.filter_timeout_cache_data();
        self.set_cache(cache);
    }
}

// Getters
impl DnsCache {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> CacheByRecordType{
        self.cache.clone()
    }

    //Gets the max size of the cache
    pub fn get_max_size(&self) -> u32 {
        self.max_size.clone()
    }

    // Gets the size of the cache
    pub fn get_size(&self) -> u32 {
        self.size.clone()
    }
}

// Setters
impl DnsCache {
    // Sets the cache
    pub fn set_cache(&mut self, cache: CacheByRecordType) {
        self.cache = cache
    }

    // Sets the max size of the cache
    pub fn set_max_size(&mut self, max_size: u32) {
        self.max_size = max_size
    }

    // Sets the size of the cache
    pub fn set_size(&mut self, size: u32) {
        self.size = size
    }
}

#[cfg(test)] 
mod dns_cache_test {
    use crate::dns_cache::DnsCache;
    use crate::dns_cache::cache_by_record_type::CacheByRecordType;
    use crate::dns_cache::cache_by_record_type::cache_by_domain_name::CacheByDomainName;
    use crate::rr_cache::RRStoredData;
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::type_rtype::Rtype;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use std::{collections::HashMap, net::IpAddr, str::FromStr};

    //Constructor test 
    #[test]
    fn constructor_test(){
        let cache = DnsCache::new();
        assert!(cache
            .get_cache()
            .get_cache_data()
            .is_empty());
    }

    //Setters and getters test
    #[test]
    fn get_and_set_max_size(){
        let mut cache = DnsCache::new();
        assert_eq!(cache.get_max_size(), 0);
        cache.set_max_size(5);
        assert_eq!(cache.get_max_size(), 5);
    }

    #[test]
    fn set_and_get_size(){
        let mut cache = DnsCache::new();
        assert_eq!(cache.get_size(), 0);
        cache.set_size(5);
        assert_eq!(cache.get_size(), 5);
    }

    #[test]
    fn set_and_get_cache(){
        let mut cache = DnsCache::new();
        assert!(cache.get_cache().get_cache_data().is_empty());
        let mut cache_data = CacheByRecordType::new();
        let mut cache_data_hash = HashMap::new();
        let mut host_data = CacheByDomainName::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);
        host_data.add_to_host_data(domain_name, rr_cache);
        cache_data_hash.insert(Rtype::A, host_data);

        cache_data.set_cache_data(cache_data_hash);

        cache.set_cache(cache_data);

        assert!(!cache.get_cache().get_cache_data().is_empty());
    }

    //Add test
    #[test]
    fn add_to_cache_data(){
        let mut cache = DnsCache::new();

        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        assert!(cache.get_cache().get_cache_data().is_empty());

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);

        cache.add(domain_name.clone(), resource_record_2);

        assert_eq!(cache.get_cache().get_cache_data().len(), 2);
        assert_eq!(cache.get_size(), 2)
    }

    //Add domain with full cache test
    #[test]
    fn add_domain_with_full_cache(){
        let mut cache = DnsCache::new();

        cache.set_max_size(1);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_size(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);

        cache.add(domain_name.clone(), resource_record_2);

        assert_eq!(cache.get_size(), 1);

        let cache_element = cache.get(domain_name.clone(), Rtype::TXT).unwrap();

        assert_eq!(cache_element.len(), 1);
    }

    //Remove test
    #[test]
    fn remove_from_dns_cache(){
        let mut cache = DnsCache::new();

        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);

        cache.add(domain_name.clone(), resource_record_2);

        assert_eq!(cache.get_cache().get_cache_data().len(), 2);
        assert_eq!(cache.get_size(), 2);

        cache.remove(domain_name.clone(), Rtype::TXT);

        assert_eq!(cache.get_cache().get_cache_data().len(), 2);
        assert_eq!(cache.get_size(), 1);
    }

    //Get test
    #[test]
    fn get_from_dns_cache(){
        let mut cache = DnsCache::new();

        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);

        cache.add(domain_name.clone(), resource_record_2);

        assert_eq!(cache.get_cache().get_cache_data().len(), 2);
        assert_eq!(cache.get_size(), 2);

        let cache_element = cache.get(domain_name.clone(), Rtype::TXT).unwrap();

        assert_eq!(cache_element.len(), 1);
    }

    //Get and update response time
    #[test]
    fn get_and_update_response_time(){
        let mut cache = DnsCache::new();

        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        assert!(cache.get_cache().get_cache_data().is_empty());

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);
    }

    //Update response time test
    #[test]
    fn update_response_time(){
        let mut cache = DnsCache::new();

        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);

        assert!(cache.get_cache().get_cache_data().is_empty());

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);

        cache.update_response_time(domain_name.clone(), Rtype::A, 2000, ip_address.clone());

        let rr_cache_vec = cache.get(domain_name.clone(), Rtype::A).unwrap();

        //Default response time in RFC 1034/1035 is 5000 so new response time should be 4500
        for rr_cache in rr_cache_vec {
            assert_eq!(rr_cache.get_response_time(), 4500);
        }
    }
    //Remaining test: get_response_time
    #[test]
    fn get_response_time(){
        let mut cache = DnsCache::new();

        cache.set_max_size(2);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);

        assert!(cache.get_cache().get_cache_data().is_empty());

        cache.add(domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);

        let response_time = cache.get_response_time(domain_name.clone(), Rtype::A, ip_address.clone());

        //Default response time in RFC 1034/1035 is 5000
        assert_eq!(response_time, 5000);

        cache.update_response_time(domain_name.clone(), Rtype::A, 2000, ip_address.clone());

        let response_time_2 = cache.get_response_time(domain_name.clone(), Rtype::A, ip_address.clone());

        //Default response time in RFC 1034/1035 is 5000 so new response time should be 4500 because 5000/2 + 2000/2 = 4500
        assert_eq!(response_time_2, 4500);
    }

    #[test]
    fn is_cached() {
        let mut cache = DnsCache::new();
        cache.set_max_size(1);

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let a_rdata = ARdata::new_from_addr(IpAddr::from_str("93.184.216.34").unwrap());
        let a_rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(a_rdata);
        cache.add(domain_name.clone(), resource_record);
        assert_eq!(cache.get_size(), 1);

        assert_eq!(cache.is_cached(domain_name, Rtype::A), true);
    }
}
