use crate::cache_data::CacheData;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{ResourceRecord, self};
use crate::rr_cache::RRCache;
use crate::message::type_rtype::Rtype;
use chrono::prelude::*;
use std::collections::HashMap;
use crate::domain_name::DomainName;
use crate::cache_data::host_data::HostData;

#[derive(Clone)]
// Struct that represents a cache for dns
pub struct DnsCache {
    // first hash by type, then by hostname
    cache: CacheData,
    max_size: u32,
    size: u32,
}

impl DnsCache {
    // Creates a new DnsCache with default values
    //
    // # Examples
    // '''
    // let cache = DnsCache::new();
    //
    // assert_eq!(cache.cache.len(), 0);
    // '''
    //
    pub fn new() -> Self {
        let cache = DnsCache {
            cache: CacheData::new(),
            max_size: 0,
            size: 0,
        };

        cache
    }

    // Adds an element to cache
    pub fn add(&mut self,rtype: Rtype, domain_name: DomainName, resource_record: ResourceRecord) {
        //See if max size is 0
        if self.max_size < 1 {
            return;
        }

        // see cache space
        if self.get_size() >= self.max_size {
            self.remove_oldest_used();
        }

        let rr_cache = RRCache::new(resource_record);

        let mut cache_data = self.get_cache();
        cache_data.add_to_cache_data(rtype, domain_name, rr_cache);
        self.set_cache(cache_data);
        self.set_size(self.get_size() + 1);
    }

    // Removes an element from cache
    pub fn remove(&mut self, domain_name: DomainName, rtype: Rtype) {
        let mut cache_data = self.get_cache();
        let length = cache_data.remove_from_cache_data(domain_name, rtype);
        //Size needs to be modified if something was removed
        self.set_cache(cache_data);
        self.set_size(self.get_size() - length as u32);  
    }

    // Given a domain_name, gets an element from cache
    pub fn get(&self, domain_name: DomainName, rtype: Rtype) -> Option<Vec<RRCache>> {
        let mut cache = self.get_cache();
        return cache.get_from_cache_data(domain_name, rtype)
    }

    // Removes the resource records from a domain name and type which were the oldest used
    pub fn remove_oldest_used(&mut self) {
        let mut cache = self.get_cache();
        let mut used_in = Utc::now();

        let length = cache.remove_oldest_used();
        println!("Cache size: {}", self.get_size());
        self.set_cache(cache);
        self.set_size(self.get_size() - length as u32); 
    }

    // Gets the response time from a domain name and type resource record
    pub fn get_response_time(
        &mut self,
        domain_name: DomainName,
        rr_type: Rtype,
        ip_address: String,
    ) -> u32 {
        let rr_cache_vec = self.get(domain_name, rr_type).unwrap();

        for rr_cache in rr_cache_vec {
            let rr_ip_address = match rr_cache.get_resource_record().get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            };

            let vec_ip_str_from_string_with_port =
                ip_address.split(":").collect::<Vec<&str>>()[0].clone();

            let vec_ip_str_from_string: Vec<&str> =
                vec_ip_str_from_string_with_port.split(".").collect();

            let mut ip_address_bytes: [u8; 4] = [0; 4];

            let mut index = 0;

            for byte in vec_ip_str_from_string {
                let byte = byte.parse::<u8>().unwrap();
                ip_address_bytes[index] = byte;
                index = index + 1;
            }

            if ip_address_bytes == rr_ip_address {
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
        ip_address: String,
    ) {
        let mut cache = self.get_cache();

        if let Some(x) = cache.get(rr_type) {
            let mut new_x = x.clone();
            if let Some(y) = new_x.get(&domain_name) {
                let new_y = y.clone();
                let mut rr_cache_vec = Vec::<RRCache>::new();

                for mut rr_cache in new_y {
                    let rr_ip_address = match rr_cache.get_resource_record().get_rdata() {
                        Rdata::SomeARdata(val) => val.get_address(),
                        _ => unreachable!(),
                    };

                    let vec_ip_str_from_string_with_port =
                        ip_address.split(":").collect::<Vec<&str>>()[0].clone();

                    let vec_ip_str_from_string: Vec<&str> =
                        vec_ip_str_from_string_with_port.split(".").collect();

                    let mut ip_address_bytes: [u8; 4] = [0; 4];
                    let mut index = 0;

                    for byte in vec_ip_str_from_string {
                        let byte = byte.parse::<u8>().unwrap();
                        ip_address_bytes[index] = byte;
                        index = index + 1;
                    }

                    if ip_address_bytes == rr_ip_address {
                        rr_cache
                            .set_response_time((response_time + rr_cache.get_response_time()) / 2);
                    }

                    rr_cache_vec.push(rr_cache.clone());
                }

                new_x.insert(domain_name, rr_cache_vec.clone());

                cache.insert(rr_type, new_x);

                self.set_cache(cache);
            }
        }
    }

    // pub fn print(&self) {
    //     let cache = self.get_cache();

    //     for (key, val) in cache.iter() {
    //         println!("Type: {}", key);

    //         for (key2, _val2) in val.iter() {
    //             println!("Host Name: {}", key2);
    //         }
    //     }
    // }
}

// Getters
impl DnsCache {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> CacheData{
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
    pub fn set_cache(&mut self, cache: CacheData) {
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
    use crate::cache_data::CacheData;
    use crate::cache_data::host_data::HostData;
    use crate::rr_cache::RRCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::type_rtype::Rtype;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use std::collections::HashMap;

    //Constructor test 
    #[test]
    fn constructor_test(){
        let cache = DnsCache::new();
        assert!(cache.cache.cache_data.is_empty());
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
        let mut cache_data = CacheData::new();
        let mut cache_data_hash = HashMap::new();
        let mut host_data = HostData::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRCache::new(resource_record);
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

        cache.set_max_size(1);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        assert!(cache.get_cache().get_cache_data().is_empty());

        cache.add(Rtype::A, domain_name.clone(), resource_record);

        assert_eq!(cache.get_cache().get_cache_data().len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::SomeTxtRdata(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);

        cache.add(Rtype::TXT, domain_name.clone(), resource_record_2);

        assert_eq!(cache.get_cache().get_cache_data().len(), 2);
        assert_eq!(cache.get_size(), 2)
    }
//     #[test]
//     fn add_domain_with_full_cache() {
//         let mut cache = DnsCache::new();
//         let ip_address: [u8; 4] = [127, 0, 0, 0];
//         let mut a_rdata = ARdata::new();

//         cache.set_max_size(1);
//         a_rdata.set_address(ip_address);
//         let rdata = Rdata::SomeARdata(a_rdata);

//         let mut resource_record = ResourceRecord::new(rdata);
//         resource_record.set_type_code(Rtype::A);

//         let second_resource_record = resource_record.clone();

//         cache.add("test.com".to_string(), resource_record);

//         assert_eq!(cache.get_size(), 1);

//         cache.add("test.com".to_string(), second_resource_record);

//         assert_eq!(cache.get_size(), 1);

//         assert_eq!(
//             cache.get("test.com".to_string(), "A".to_string())[0]
//                 .get_resource_record()
//                 .get_rtype(),
//             Rtype::A
//         )
//     }
//     #[test]
//     fn update_and_get_response_time() {
//         let mut dns_cache = DnsCache::new();

//         dns_cache.set_max_size(1);

//         let mut a_rdata = ARdata::new();
//         a_rdata.set_address([127, 0, 0, 1]);

//         let r_data = Rdata::SomeARdata(a_rdata);

//         let mut a_resource_record = ResourceRecord::new(r_data);
//         a_resource_record.set_type_code(Rtype::A);

//         dns_cache.add(String::from("test.com"), a_resource_record);
//         let response_time = dns_cache.get_response_time(
//             String::from("test.com"),
//             String::from("A"),
//             String::from("127.0.0.1"),
//         );
//         assert_eq!(response_time, 5000 as u32);

//         dns_cache.update_response_time(
//             String::from("test.com"),
//             String::from("A"),
//             3000,
//             String::from("127.0.0.1"),
//         );
//         let new_response_time = dns_cache.get_response_time(
//             String::from("test.com"),
//             String::from("A"),
//             String::from("127.0.0.1"),
//         );
//         assert_eq!(new_response_time, 4000 as u32);
//     }
}
