use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::rr_cache::RRCache;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
// Struct that represents a cache for dns
pub struct DnsCache {
    // first hash by type, then by hostname
    cache: HashMap<String, HashMap<String, Vec<RRCache>>>,
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
            cache: HashMap::<String, HashMap<String, Vec<RRCache>>>::new(),
            max_size: 0,
            size: 0,
        };

        cache
    }

    // Adds an element to cache
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
            //  Replace the next line when AAAA is implemented 
            28 => "TXT".to_string(),
            //
            _ => unreachable!(),
        };

        // see cache size
        if self.max_size < 1 {
            return;
        }

        // see cache space
        if self.get_size() >= self.max_size {
            self.remove_oldest_used();
        }

        let rr_cache = RRCache::new(resource_record);

        if let Some(x) = cache.get_mut(&rr_type) {
            let mut type_hash = x.clone();

            if let Some(y) = type_hash.get(&domain_name) {
                let mut host_rrs_vec = y.clone();

                host_rrs_vec.push(rr_cache);
                type_hash.insert(domain_name, host_rrs_vec);
            } else {
                let mut rr_vec = Vec::<RRCache>::new();
                rr_vec.push(rr_cache);

                type_hash.insert(domain_name, rr_vec);
            }

            cache.insert(rr_type, type_hash);
        } else {
            let mut new_hosts_hash = HashMap::<String, Vec<RRCache>>::new();
            let mut rr_vec = Vec::<RRCache>::new();
            rr_vec.push(rr_cache);

            new_hosts_hash.insert(domain_name, rr_vec);

            cache.insert(rr_type, new_hosts_hash);
        }

        self.set_cache(cache);
        self.set_size(self.get_size() + 1);
    }

    // Removes an element from cache
    pub fn remove(&mut self, domain_name: String, rr_type: String) {
        let mut cache = self.get_cache();

        if let Some(x) = cache.get(&rr_type) {
            let mut x_clone = x.clone();
            if let Some(y) = x_clone.remove(&domain_name) {
                cache.insert(rr_type, x_clone.clone());
                self.set_cache(cache);
                self.set_size(self.get_size() - y.len() as u32);
            }
        }
    }

    // Given a domain_name, gets an element from cache
    pub fn get(&mut self, domain_name: String, rr_type: String) -> Vec<RRCache> {
        let mut cache = self.get_cache();

        if let Some(x) = cache.get(&rr_type) {
            let mut new_x = x.clone();
            if let Some(y) = new_x.get(&domain_name) {
                let new_y = y.clone();
                let mut rr_cache_vec = Vec::<RRCache>::new();

                for mut rr_cache in new_y {
                    rr_cache.set_last_use(Utc::now());
                    rr_cache_vec.push(rr_cache.clone());
                }

                new_x.insert(domain_name, rr_cache_vec.clone());

                cache.insert(rr_type, new_x);

                self.set_cache(cache);

                return rr_cache_vec;
            }
        }

        return Vec::<RRCache>::new();
    }

    // Removes the resource records from a domain name and type which were the oldest used
    pub fn remove_oldest_used(&mut self) {
        let cache = self.get_cache();
        let mut used_in = Utc::now();

        let mut oldest_used_domain_name = "".to_string();
        let mut oldest_used_type = "".to_string();

        for (key, value) in cache {
            for (host_key, host_value) in value {
                let rr_last_use = host_value[0].get_last_use();

                if rr_last_use <= used_in {
                    used_in = rr_last_use;
                    oldest_used_domain_name = host_key.clone();
                    oldest_used_type = key.clone();
                }
            }
        }

        self.remove(oldest_used_domain_name, oldest_used_type);
    }

    // Gets the response time from a domain name and type resource record
    pub fn get_response_time(
        &mut self,
        domain_name: String,
        rr_type: String,
        ip_address: String,
    ) -> u32 {
        let rr_cache_vec = self.get(domain_name, rr_type);

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
        domain_name: String,
        rr_type: String,
        response_time: u32,
        ip_address: String,
    ) {
        let mut cache = self.get_cache();

        if let Some(x) = cache.get(&rr_type) {
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
                        rr_cache.set_response_time((response_time + rr_cache.get_response_time())/2);
                    }

                    rr_cache_vec.push(rr_cache.clone());
                }

                new_x.insert(domain_name, rr_cache_vec.clone());

                cache.insert(rr_type, new_x);

                self.set_cache(cache);
            }
        }
    }

    pub fn print(&self) {
        let cache = self.get_cache();

        for (key, val) in cache.iter() {
            println!("Type: {}", key);

            for (key2, _val2) in val.iter() {
                println!("Host Name: {}", key2);
            }
        }
    }
}

// Getters
impl DnsCache {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> HashMap<String, HashMap<String, Vec<RRCache>>> {
        self.cache.clone()
    }

    // Gets the size of the cache
    pub fn get_size(&self) -> u32 {
        self.size
    }
}

// Setters
impl DnsCache {
    // Sets the cache
    pub fn set_cache(&mut self, cache: HashMap<String, HashMap<String, Vec<RRCache>>>) {
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
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn constructor_test() {
        let cache = DnsCache::new();

        assert_eq!(cache.cache.len(), 0);
    }

    #[test]
    fn add_get_and_remove_test() {
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

        cache.add("test.com".to_string(), ns_resource_record);

        assert_eq!(cache.get_size(), 1);

        cache.add("test.com".to_string(), a_resource_record);

        assert_eq!(
            cache.get("test.com".to_string(), "A".to_string())[0]
                .get_resource_record()
                .get_type_code(),
            1
        );

        assert_eq!(
            cache.get("test.com".to_string(), "NS".to_string())[0]
                .get_resource_record()
                .get_type_code(),
            2
        );

        cache.remove("test.com".to_string(), "NS".to_string());

        assert_eq!(cache.get_size(), 1);

        cache.remove("test.com".to_string(), "A".to_string());

        assert_eq!(cache.get_size(), 0);
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

        cache.add("test.com".to_string(), resource_record);

        assert_eq!(cache.get_size(), 1);

        cache.add("test.com".to_string(), second_resource_record);

        assert_eq!(cache.get_size(), 1);

        assert_eq!(
            cache.get("test.com".to_string(), "A".to_string())[0]
                .get_resource_record()
                .get_type_code(),
            1
        )
    }
}