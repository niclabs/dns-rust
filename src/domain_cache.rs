use crate::message::resource_record::ResourceRecord;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
/// An structs that represents one element in the dns cache.
pub struct DomainCache {
    // Resource Records of the domain name
    resource_records: HashMap<String, ResourceRecord>,
    // Last time that the cache was used
    last_use: DateTime<Utc>,
    // Mean of response time of the ip address
    response_time: u32,
}

impl DomainCache {
    /// Creates a new DomainCache struct
    ///
    /// # Examples
    /// '''
    /// let domain_cache = DomainCache::new();
    ///
    /// assert_eq!(domain_cache.resource_records.len(), 0);
    /// assert_eq!(domain_cache.response_time, 5);
    /// '''
    ///
    pub fn new() -> Self {
        let domain_cache = DomainCache {
            resource_records: HashMap::<String, ResourceRecord>::new(),
            last_use: Utc::now(),
            response_time: 5,
        };

        domain_cache
    }
}

// Getters
impl DomainCache {
    // Gets the resource records from the domain cache
    pub fn get_resource_records(&self) -> HashMap<String, ResourceRecord> {
        self.resource_records.clone()
    }

    // Gets the last use of the domain in cache
    pub fn get_last_use(&self) -> DateTime<Utc> {
        self.last_use
    }

    // Gets the mean response time of the ip address of the domain name
    pub fn get_response_time(&self) -> u32 {
        self.response_time
    }
}

// Setters
impl DomainCache {
    // Sets the resource records attribute with new value
    pub fn set_resource_records(&mut self, resource_records: HashMap<String, ResourceRecord>) {
        self.resource_records = resource_records;
    }

    // Sets the last use attribute with new value
    pub fn set_last_use(&mut self, last_use: DateTime<Utc>) {
        self.last_use = last_use;
    }

    // Sets the response time attribute with new value
    pub fn set_response_time(&mut self, response_time: u32) {
        self.response_time = response_time;
    }
}

mod test {
    use crate::domain_cache::DomainCache;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use chrono::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn constructor_test() {
        let domain_cache = DomainCache::new();

        assert_eq!(domain_cache.resource_records.len(), 0);
        assert_eq!(domain_cache.response_time, 5);
    }

    #[test]
    fn set_and_get_resource_records() {
        let mut domain_cache = DomainCache::new();

        assert_eq!(domain_cache.get_resource_records().len(), 0);

        let mut resource_records = HashMap::<String, ResourceRecord>::new();

        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);

        resource_records.insert("A".to_string(), resource_record);

        domain_cache.set_resource_records(resource_records);

        assert_eq!(domain_cache.get_resource_records().len(), 1);
    }

    #[test]
    fn set_and_get_last_use() {
        let mut domain_cache = DomainCache::new();

        let now = Utc::now();

        assert_ne!(domain_cache.get_last_use(), now);

        domain_cache.set_last_use(now);

        assert_eq!(domain_cache.get_last_use(), now);
    }

    #[test]
    fn set_and_get_response_time() {
        let mut domain_cache = DomainCache::new();

        assert_eq!(domain_cache.get_response_time(), 5);

        domain_cache.set_response_time(2);

        assert_eq!(domain_cache.get_response_time(), 2);
    }
}
