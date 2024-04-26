pub mod rr_stored_data;

extern crate lru;

use lru::LruCache;
use std::num::NonZeroUsize;

use crate::dns_cache::rr_stored_data::RRStoredData;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::type_rtype::Rtype;
use std::net::IpAddr;
use crate::domain_name::DomainName;
use chrono::Utc;

#[derive(Clone, Debug)]
/// Struct that represents a cache for dns
pub struct DnsCache {
    // first hash by type, then by hostname
    cache: LruCache<(Rtype, DomainName), Vec<RRStoredData>>,
    max_size: NonZeroUsize,
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
    pub fn new(max_size: Option<NonZeroUsize>) -> Self {
        let cache = DnsCache {
            cache: LruCache::new(max_size.unwrap_or_else(|| NonZeroUsize::new(5000).unwrap())),
            max_size: max_size.unwrap_or_else(|| NonZeroUsize::new(100).unwrap()),
        };
        cache
    }

    /// Adds an element to cache
    pub fn add(&mut self, domain_name: DomainName, resource_record: ResourceRecord) {
        let rtype = resource_record.get_rtype();

        let rr_cache = RRStoredData::new(resource_record);

        let mut cache_data = self.get_cache();

        if let Some(rr_cache_vec) = cache_data.get_mut(&(rtype, domain_name.clone())) {
            rr_cache_vec.push(rr_cache);
        } else {
            let mut rr_cache_vec = Vec::new();
            rr_cache_vec.push(rr_cache);
            cache_data.put((rtype, domain_name.clone()), rr_cache_vec);
        }

        self.set_cache(cache_data); 
        // see cache space
    }

    /// TODO: Crear test y mejorar función de acuerdo a RFC de Negative caching
    /// Add negative resource record type SOA to cache for negative answers
    pub fn add_negative_answer(&mut self, domain_name: DomainName, rtype: Rtype, resource_record:ResourceRecord) {
        let mut cache_data = self.get_cache();
        let rr_cache = RRStoredData::new(resource_record);
        
        if let Some(rr_cache_vec) = cache_data.get_mut(&(rtype, domain_name.clone())){
            rr_cache_vec.push(rr_cache);
        } else {
            let mut rr_cache_vec = Vec::new();
            rr_cache_vec.push(rr_cache);
            cache_data.put((rtype, domain_name.clone()), rr_cache_vec);
        }

        self.set_cache(cache_data);
    }

    /// Removes an element from cache
    pub fn remove(&mut self, domain_name: DomainName, rtype: Rtype) {
        let mut cache_data = self.get_cache();
        let _extracted = cache_data.pop(&(rtype, domain_name));
        self.set_cache(cache_data); 
    }

    /// Given a domain_name, gets an element from cache
    pub fn get(&mut self, domain_name: DomainName, rtype: Rtype) -> Option<Vec<RRStoredData>> {
        let mut cache = self.get_cache();

        let rr_cache_vec = cache.get(&(rtype, domain_name)).cloned();

        self.set_cache(cache);

        rr_cache_vec
    }

    /// Removes the resource records from a domain name and type which were the oldest used
    pub fn remove_oldest_used(&mut self) {
        let mut cache = self.get_cache();
        let _oldest = cache.pop_lru();
        self.set_cache(cache); 
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

        if let Some(rr_cache_vec) = cache.get_mut(&(rr_type, domain_name)){
            for rr in rr_cache_vec {
                let rr_ip_address = match rr.get_resource_record().get_rdata() {
                    Rdata::A(val) => val.get_address(),
                    _ => unreachable!(),
                };
                if ip_address == rr_ip_address {
                    rr.set_response_time(response_time);
                }
            }
        }
    }

    /// Checks if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Checks if a domain name is cached
    pub fn is_cached(&self, domain_name: DomainName, rtype: Rtype) -> bool {
        if let Some(key_data) = self.cache.peek(&(rtype, domain_name)) {
            if key_data.len() > 0 {
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
        let cache = self.get_cache();
        
        for (_key, rr_cache_vec) in cache.iter() {
            let mut rr_cache_vec_cleaned = Vec::new();

            for stored_element in rr_cache_vec {
                let ttl = stored_element.get_resource_record().get_ttl();
                let creation_time = stored_element.get_creation_time();
                let now = Utc::now();
                let duration = now.signed_duration_since(creation_time);

                if duration.num_seconds() < ttl as i64 {
                    rr_cache_vec_cleaned.push(stored_element.clone());
                }
            
            }
        }
    }
}

// Getters
impl DnsCache {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> LruCache<(Rtype, DomainName), Vec<RRStoredData>>{
        self.cache.clone()
    }

    //Gets the max size of the cache
    pub fn get_max_size(&self) -> NonZeroUsize {
        self.max_size.clone()
    }
}

// Setters
impl DnsCache {
    // Sets the cache
    pub fn set_cache(&mut self, cache: LruCache<(Rtype, DomainName), Vec<RRStoredData>>) {
        self.cache = cache
    }

    // Sets the max size of the cache
    pub fn set_max_size(&mut self, max_size: NonZeroUsize) {
        self.max_size = max_size
    }
}

#[cfg(test)] 
mod dns_cache_test {
    use super::*;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::aaaa_rdata::AAAARdata;

    #[test]
    fn test_new() {
        let cache = DnsCache::new(NonZeroUsize::new(10));

        assert_eq!(cache.cache.len(), 0);
        assert_eq!(cache.max_size, NonZeroUsize::new(10).unwrap());
    }

    #[test]
    fn get_cache() {
        let cache = DnsCache::new(NonZeroUsize::new(10));
        let cache_data = cache.get_cache();

        assert_eq!(cache_data.len(), 0);
        assert!(cache_data.is_empty());
    }

    #[test]
    fn get_max_size() {
        let cache = DnsCache::new(NonZeroUsize::new(10));
        let max_size = cache.get_max_size();

        assert_eq!(max_size, NonZeroUsize::new(10).unwrap());
    }

    #[test]
    fn set_cache() {
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let mut cache_data = LruCache::new(NonZeroUsize::new(10).unwrap());
        cache_data.put((Rtype::A, DomainName::new_from_str("example.com")), vec![]);

        cache.set_cache(cache_data.clone());

        assert!(!cache.get_cache().is_empty());
    }

    #[test]
    fn set_max_size() {
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let max_size = NonZeroUsize::new(20).unwrap();

        cache.set_max_size(max_size.clone());

        assert_eq!(cache.get_max_size(), max_size);
    }

    #[test]
    fn add() {
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone());

        let rr_cache_vec = cache.get(domain_name.clone(), Rtype::A).unwrap();

        let first_rr_cache = rr_cache_vec.first().unwrap();

        assert_eq!(rr_cache_vec.len(), 1);

        assert_eq!(first_rr_cache.get_resource_record().get_rtype(), Rtype::A);

        assert_eq!(first_rr_cache.get_resource_record().get_name(), domain_name.clone());
    }

    #[test]
    fn add_two_elements_same_type_and_domain_name(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone());

        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone());

        let rr_cache_vec = cache.get(domain_name.clone(), Rtype::A).unwrap();

        assert_eq!(rr_cache_vec.len(), 2);
    }

    #[test]
    fn add_two_elements_different_type_and_same_domain_name(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone());

        let ip_address_v6 = IpAddr::from([0, 0, 0, 0, 0, 0, 0, 1]);
        let mut aaaa_rdata = AAAARdata::new();
        aaaa_rdata.set_address(ip_address_v6);
        let rdata_2 = Rdata::AAAA(aaaa_rdata);
        let mut resource_record_2 = ResourceRecord::new(rdata_2);
        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::AAAA);

        cache.add(domain_name.clone(), resource_record_2.clone());

        let rr_cache_vec = cache.get(domain_name.clone(), Rtype::A).unwrap();

        let rr_cache_vec_2 = cache.get(domain_name.clone(), Rtype::AAAA).unwrap();

        assert_eq!(rr_cache_vec.len(), 1);
        assert_eq!(rr_cache_vec_2.len(), 1);
    }
}