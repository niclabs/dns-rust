pub mod rr_stored_data;

extern crate lru;

use lru::LruCache;
use std::num::NonZeroUsize;

use crate::dns_cache::rr_stored_data::RRStoredData;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::rcode::Rcode;
use crate::message::type_qtype::Qtype;
use crate::message::class_qclass::Qclass;
use std::net::IpAddr;
use crate::domain_name::DomainName;
use chrono::Utc;

#[derive(Clone, Debug)]
/// Struct that represents a cache for dns
pub struct DnsCache {
    // Cache for the resource records, where the key is the type of the query, the class of the query and the qname of the query
    cache: LruCache<(Qtype, Qclass, DomainName), Vec<RRStoredData>>,
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
            cache: LruCache::new(max_size.unwrap_or_else(|| NonZeroUsize::new(1667).unwrap())),
            max_size: max_size.unwrap_or_else(|| NonZeroUsize::new(100).unwrap()),
        };
        cache
    }

    /// Adds an element to cache
    pub fn add(&mut self, domain_name: DomainName, resource_record: ResourceRecord, qtype: Qtype, qclass: Qclass, rcode: Option<Rcode>) {

        let mut rr_cache = RRStoredData::new(resource_record);

        let rcode = rcode.unwrap_or_else(|| Rcode::NOERROR);

        if rcode != Rcode::NOERROR {
            rr_cache.set_rcode(rcode);
        }

        let mut cache_data = self.get_cache();

        if let Some(rr_cache_vec) = cache_data.get_mut(&(qtype, qclass, domain_name.clone())) {
            let mut val_exist = false;
            for rr in rr_cache_vec.iter_mut() {
                if rr.get_resource_record().get_rdata() == rr_cache.get_resource_record().get_rdata() {
                    val_exist = true;
                    *rr = rr_cache.clone();
                    break;
                }
            }
            if !val_exist {
                rr_cache_vec.push(rr_cache);
            }
        } else {
            let mut rr_cache_vec = Vec::new();
            rr_cache_vec.push(rr_cache);
            cache_data.put((qtype, qclass, domain_name.clone()), rr_cache_vec);
        }

        self.set_cache(cache_data); 
        // see cache space
    }

    /// TODO: Crear test y mejorar función de acuerdo a RFC de Negative caching
    /// Add negative resource record type SOA to cache for negative answers
    pub fn add_negative_answer(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass, resource_record:ResourceRecord) {
        let mut cache_data = self.get_cache();
        let rr_cache = RRStoredData::new(resource_record);
        
        if let Some(rr_cache_vec) = cache_data.get_mut(&(qtype, qclass, domain_name.clone())){
            rr_cache_vec.push(rr_cache);
        } else {
            let mut rr_cache_vec = Vec::new();
            rr_cache_vec.push(rr_cache);
            cache_data.put((qtype, qclass, domain_name.clone()), rr_cache_vec);
        }

        self.set_cache(cache_data);
    }

    /// Removes an element from cache
    pub fn remove(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        let mut cache_data = self.get_cache();
        let _extracted = cache_data.pop(&(qtype, qclass, domain_name));
        self.set_cache(cache_data); 
    }

    /// Given a domain_name, gets an element from cache
    pub fn get(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) -> Option<Vec<RRStoredData>> {
        let mut cache = self.get_cache();

        let rr_cache_vec = cache.get(&(qtype, qclass, domain_name)).cloned();

        self.set_cache(cache);

        rr_cache_vec
    }

    /// Removes the resource records from a domain name and type which were the oldest used
    pub fn remove_oldest_used(&mut self) {
        let mut cache = self.get_cache();
        let _oldest = cache.peek_lru();
        let oldest_key = _oldest.unwrap().0.clone(); // Clone the key to release the immutable borrow
        let _extracted = cache.pop(&oldest_key);
        self.set_cache(cache); 
    }

    /// Gets the response time from a domain name and type resource record
    pub fn get_response_time(
        &mut self,
        domain_name: DomainName,
        qtype: Qtype,
        qclass: Qclass,
        ip_address: IpAddr,
    ) -> u32 {
        let rr_cache_vec = self.get(domain_name, qtype, qclass).unwrap();

        for rr_cache in rr_cache_vec {
            let rr_ip_address = match rr_cache.get_resource_record().get_rdata() {
                Rdata::A(val) => val.get_address(),
                _ => unreachable!(),
            };
            let boolean = ip_address == rr_ip_address;
            if boolean {
                let response_time = rr_cache.get_response_time();
                return response_time;
            }
        }
        // Default response time in RFC 1034/1035
        return 5000;
    }

    // Gets the response time from a domain name and type resource record
    pub fn update_response_time(
        &mut self,
        domain_name: DomainName,
        qtype: Qtype,
        qclass: Qclass,
        response_time: u32,
        ip_address: IpAddr,
    ) {
        let mut cache = self.get_cache();

        if let Some(rr_cache_vec) = cache.get_mut(&(qtype, qclass, domain_name)){
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
        self.set_cache(cache);
    }

    /// Checks if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Checks if a domain name is cached
    pub fn is_cached(&self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) -> bool {
        if let Some(key_data) = self.cache.peek(&(qtype, qclass, domain_name)) {
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
        
        for (key, rr_cache_vec) in cache {
            let mut rr_cache_vec_cleaned = Vec::new();

            for stored_element in rr_cache_vec.iter() {
                let ttl = stored_element.get_resource_record().get_ttl();
                let creation_time = stored_element.get_creation_time();
                let now = Utc::now();
                let duration = now.signed_duration_since(creation_time);

                if duration.num_seconds() < ttl as i64 {
                    let new_ttl = ttl - duration.num_seconds() as u32;
                    let mut resource_record = stored_element.get_resource_record();
                    resource_record.set_ttl(new_ttl);
                    let mut new_stored_element = stored_element.clone();
                    new_stored_element.set_resource_record(resource_record);
                    rr_cache_vec_cleaned.push(new_stored_element.clone());
                }
            
            }
            if rr_cache_vec_cleaned.is_empty(){
                let _removed = self.cache.pop(&key);
            }

        }
        let cloned_cache = self.get_cache().clone();
        self.set_cache(cloned_cache);
    }
}

// Getters
impl DnsCache {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> LruCache<(Qtype, Qclass, DomainName), Vec<RRStoredData>>{
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
    pub fn set_cache(&mut self, cache: LruCache<(Qtype, Qclass, DomainName), Vec<RRStoredData>>) {
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
    use crate::message::type_rtype::Rtype;
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
        cache_data.put((Qtype::A, Qclass::IN, DomainName::new_from_str("example.com")), vec![]);

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

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        let first_rr_cache = rr_cache_vec.first().unwrap();

        assert_eq!(rr_cache_vec.len(), 1);

        assert_eq!(first_rr_cache.get_resource_record().get_rtype(), Rtype::A);

        assert_eq!(first_rr_cache.get_resource_record().get_name(), domain_name.clone());
    }

    #[test]
    fn add_two_elements_same_type_class_and_domain_name(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

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

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let ip_address_v6 = IpAddr::from([0, 0, 0, 0, 0, 0, 0, 1]);
        let mut aaaa_rdata = AAAARdata::new();
        aaaa_rdata.set_address(ip_address_v6);
        let rdata_2 = Rdata::AAAA(aaaa_rdata);
        let mut resource_record_2 = ResourceRecord::new(rdata_2);
        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::AAAA);

        cache.add(domain_name.clone(), resource_record_2.clone(), Qtype::AAAA, Qclass::IN, None);

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        let rr_cache_vec_2 = cache.get(domain_name.clone(), Qtype::AAAA, Qclass::IN).unwrap();

        assert_eq!(rr_cache_vec.len(), 1);
        assert_eq!(rr_cache_vec_2.len(), 1);
    }

    #[test]
    fn add_duplicate_elements(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr_cache_vec.len(), 1);
    }

    #[test]
    fn remove() {
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        cache.remove(domain_name.clone(), Qtype::A, Qclass::IN);

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec.is_none());
    }

    #[test]
    fn get() {
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        let first_rr_cache = rr_cache_vec.first().unwrap();

        assert_eq!(rr_cache_vec.len(), 1);

        assert_eq!(first_rr_cache.get_resource_record().get_rtype(), Rtype::A);

        assert_eq!(first_rr_cache.get_resource_record().get_name(), domain_name.clone());

        let rr_rdata = first_rr_cache.get_resource_record().get_rdata();

        match rr_rdata {
            Rdata::A(val) => {
                assert_eq!(val.get_address(), ip_address);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn get_none() {
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec.is_none());
    }

    #[test]
    fn remove_oldest_used() {
        let mut cache = DnsCache::new(NonZeroUsize::new(3));
        let domain_name = DomainName::new_from_str("example.com");
        let domain_name_2 = DomainName::new_from_str("example2.com");
        let domain_name_3 = DomainName::new_from_str("example3.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        let mut a_rdata_2 = ARdata::new();
        a_rdata_2.set_address(ip_address_2);
        let rdata_2 = Rdata::A(a_rdata_2);
        let mut resource_record_2 = ResourceRecord::new(rdata_2);
        resource_record_2.set_name(domain_name_2.clone());
        resource_record_2.set_type_code(Rtype::A);

        let mut a_rdata_3 = ARdata::new();
        a_rdata_3.set_address(ip_address_3);
        let rdata_3 = Rdata::A(a_rdata_3);
        let mut resource_record_3 = ResourceRecord::new(rdata_3);
        resource_record_3.set_name(domain_name_3.clone());
        resource_record_3.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);
        cache.add(domain_name_2.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        cache.add(domain_name_3.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        let _rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN);

        let _rr_cache_vec_2 = cache.get(domain_name_2.clone(), Qtype::A, Qclass::IN);

        cache.remove_oldest_used();

        let rr_cache_vec = cache.get(domain_name_3.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec.is_none());

        let rr_cache_vec_2 = cache.get(domain_name_2.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec_2.is_some());

        let rr_cache_vec_3 = cache.get(domain_name.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec_3.is_some());
    }

    #[test]
    fn get_response_time(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let response_time = 1000;
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        let mut rr_cache = RRStoredData::new(resource_record.clone());
        rr_cache.set_response_time(response_time);

        let rr_cache_vec = vec![rr_cache];

        let mut lru_cache = cache.get_cache();

        lru_cache.put((Qtype::A, Qclass::IN, domain_name.clone()), rr_cache_vec);

        cache.set_cache(lru_cache);

        let response_time_obtained = cache.get_response_time(domain_name.clone(), Qtype::A, Qclass::IN, ip_address);

        assert_eq!(response_time_obtained, response_time);
    }

    #[test]
    fn update_response_time(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));
        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let new_response_time = 2000;

        cache.update_response_time(domain_name.clone(), Qtype::A, Qclass::IN, new_response_time, ip_address);

        let response_time_obtained = cache.get_response_time(domain_name.clone(), Qtype::A, Qclass::IN, ip_address);

        assert_eq!(response_time_obtained, new_response_time);
    }

    #[test]
    fn is_empty(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));

        assert!(cache.is_empty());

        let domain_name = DomainName::new_from_str("example.com");
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata); 
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        
        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        assert!(!cache.is_empty());
    }

    #[test]
    fn is_cached(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));

        let domain_name = DomainName::new_from_str("example.com");

        assert!(!cache.is_cached(domain_name.clone(), Qtype::A, Qclass::IN));

        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata); 
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        
        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        assert!(cache.is_cached(domain_name.clone(), Qtype::A, Qclass::IN));

        assert!(!cache.is_cached(domain_name.clone(), Qtype::AAAA, Qclass::IN));
    }

    #[test]
    fn timeout_cache(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));

        let domain_name = DomainName::new_from_str("example.com");

        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let ttl = 0;
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata); 
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        resource_record.set_ttl(ttl);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        cache.timeout_cache();

        assert!(cache.is_empty());
    }

    #[test]
    fn timeout_cache_two_elements(){
        let mut cache = DnsCache::new(NonZeroUsize::new(10));

        let domain_name = DomainName::new_from_str("example.com");
        let domain_name_2 = DomainName::new_from_str("example2.com");

        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let ttl = 0;
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata); 
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        resource_record.set_ttl(ttl);

        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ttl_2 = 100;
        let mut a_rdata_2 = ARdata::new();
        a_rdata_2.set_address(ip_address_2);
        let rdata_2 = Rdata::A(a_rdata_2); 
        let mut resource_record_2 = ResourceRecord::new(rdata_2);
        resource_record_2.set_name(domain_name_2.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(ttl_2);

        cache.add(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);
        cache.add(domain_name_2.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);

        cache.timeout_cache();

        assert!(!cache.is_empty());

        let rr_cache_vec = cache.get(domain_name.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec.is_none());

        let rr_cache_vec_2 = cache.get(domain_name_2.clone(), Qtype::A, Qclass::IN);

        assert!(rr_cache_vec_2.is_some());
    }
}