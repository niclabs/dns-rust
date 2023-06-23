use crate::message::resource_record::ResourceRecord;
use chrono::prelude::*;

#[derive(Clone)]
// An structs that represents one element in the dns cache.
pub struct RRCache {
    // Resource Records of the domain name
    resource_record: ResourceRecord,
    // Mean of response time of the ip address
    response_time: u32,
    // Last use of the rr
    last_use: DateTime<Utc>,
}

impl RRCache {
    // Creates a new RRCache struct
    //
    // # Examples
    // '''
    // let rr_cache = RRCache::new();
    //
    // assert_eq!(rr_cache.resource_records.len(), 0);
    // assert_eq!(rr_cache.response_time, 5);
    // '''
    //
    pub fn new(resource_record: ResourceRecord) -> Self {
        let rr_cache = RRCache {
            resource_record: resource_record,
            response_time: 5000,
            last_use: Utc::now(),
        };

        rr_cache
    }
}

// Getters
impl RRCache {
    // Gets the resource record from the domain cache
    pub fn get_resource_record(&self) -> ResourceRecord {
        self.resource_record.clone()
    }

    // Gets the mean response time of the ip address of the domain name
    pub fn get_response_time(&self) -> u32 {
        self.response_time
    }

    // Gets the last use of the domain in cache
    pub fn get_last_use(&self) -> DateTime<Utc> {
        self.last_use
    }
}

// Setters
impl RRCache {
    // Sets the resource record attribute with new value
    pub fn set_resource_record(&mut self, resource_record: ResourceRecord) {
        self.resource_record = resource_record;
    }

    // Sets the response time attribute with new value
    pub fn set_response_time(&mut self, response_time: u32) {
        self.response_time = response_time;
    }

    // Sets the last use attribute with new value
    pub fn set_last_use(&mut self, last_use: DateTime<Utc>) {
        self.last_use = last_use;
    }
}

#[cfg(test)]
mod rr_cache_test {
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::Rtype;
    use crate::message::Rclass;
    use crate::message::resource_record::ResourceRecord;
    use crate::rr_cache::RRCache;
    use chrono::prelude::*;

    #[test]
    fn constructor_test() {
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rtype::A);

        let rr_cache = RRCache::new(resource_record);

        assert_eq!(Rtype::from_rtype_to_int(rr_cache.resource_record.get_rtype()), 1);
        assert_eq!(rr_cache.response_time, 5000);
    }

    #[test]
    fn set_and_get_resource_record() {
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata.clone());
        resource_record.set_type_code(Rtype::A);

        let mut rr_cache = RRCache::new(resource_record);

        assert_eq!(Rtype::from_rtype_to_int(rr_cache.resource_record.get_rtype()), 1);

        let second_ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut second_a_rdata = ARdata::new();

        second_a_rdata.set_address(second_ip_address);
        let second_rdata = Rdata::SomeARdata(second_a_rdata);

        let mut second_resource_record = ResourceRecord::new(second_rdata);
        second_resource_record.set_type_code(Rtype::NS);

        rr_cache.set_resource_record(second_resource_record);

        assert_eq!(Rtype::from_rtype_to_int(rr_cache.get_resource_record().get_rtype()), 2);
    }

    #[test]
    fn set_and_get_response_time() {
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rtype::A);

        let mut rr_cache = RRCache::new(resource_record);

        assert_eq!(rr_cache.get_response_time(), 5000);

        rr_cache.set_response_time(2000);

        assert_eq!(rr_cache.get_response_time(), 2000);
    }

    #[test]
    fn set_and_get_last_use() {
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rtype::A);

        let mut rr_cache = RRCache::new(resource_record);

        let now = Utc::now();

        assert_ne!(rr_cache.get_last_use(), now);

        rr_cache.set_last_use(now);

        assert_eq!(rr_cache.get_last_use(), now);
    }
}
