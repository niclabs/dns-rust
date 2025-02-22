use std::hash::{Hash, Hasher};
use crate::message::resource_record::ResourceRecord;
use crate::message::rcode::Rcode;
use chrono::prelude::*;


#[derive(Clone)]
/// An structs that represents one element in the dns cache.
pub struct RRStoredData {
    // RCODE associated with the answer
    rcode: Rcode,
    /// Resource Records of the domain name
    resource_record: ResourceRecord,
    /// Mean of response time of the ip address
    response_time: u32,
    /// Time of creation of the `RRStoredData` in the Resolver's cache.
    creation_time: DateTime<Utc>,
}

impl PartialEq for RRStoredData {
    fn eq(&self, other: &Self) -> bool {
        self.resource_record == other.resource_record
    }
}

impl Eq for RRStoredData {}

impl Hash for RRStoredData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resource_record.hash(state);
    }
}

impl RRStoredData {
    // Creates a new RRStoredData struct
    //
    // # Examples
    // '''
    // let rr_cache = RRStoredData::new();
    //
    // assert_eq!(rr_cache.resource_records.len(), 0);
    // assert_eq!(rr_cache.response_time, 5);
    // '''
    //
    pub fn new(resource_record: ResourceRecord) -> Self {
        let rr_cache = RRStoredData {
            rcode: Rcode::NOERROR,
            resource_record,
            response_time: 5000,
            creation_time: Utc::now(),
        };

        rr_cache
    }

    pub fn get_absolute_ttl(&self) -> DateTime<Utc> {
        let ttl = self.resource_record.get_ttl();
        let creation_time = self.creation_time;

        creation_time + chrono::Duration::seconds(ttl as i64)
    }
}

// Getters
impl RRStoredData {
    // Gets the rcode of the stored data
    pub fn get_rcode(&self) -> Rcode {
        self.rcode
    }

    // Gets the resource record from the domain cache
    pub fn get_resource_record(&self) -> ResourceRecord {
        self.resource_record.clone()
    }

    // Gets the mean response time of the ip address of the domain name
    pub fn get_response_time(&self) -> u32 {
        self.response_time
    }

    // Gets the creation time of the domain in cache
    pub fn get_creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }
}

// Setters
impl RRStoredData {
    // Sets the rcode attribute with new value
    pub fn set_rcode(&mut self, rcode: Rcode) {
        self.rcode = rcode;
    }

    // Sets the resource record attribute with new value
    pub fn set_resource_record(&mut self, resource_record: ResourceRecord) {
        self.resource_record = resource_record;
    }

    // Sets the response time attribute with new value
    pub fn set_response_time(&mut self, response_time: u32) {
        self.response_time = response_time;
    }
}

#[cfg(test)]
mod rr_cache_test {
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::rrtype::Rrtype;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::rcode::Rcode;
    use crate::dns_cache::rr_stored_data::RRStoredData;
    use std::net::IpAddr;
    use chrono::prelude::*;

    #[test]
    fn constructor_test() {
        let ip_address: IpAddr = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rrtype::A);

        let rr_cache = RRStoredData::new(resource_record);

        assert_eq!(u16::from(rr_cache.resource_record.get_rtype()), 1);
        assert_eq!(rr_cache.response_time, 5000);
    }

    #[test]
    fn set_and_get_rcode() {
        let ip_address: IpAddr = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rrtype::A);

        let mut rr_cache = RRStoredData::new(resource_record);

        assert_eq!(rr_cache.get_rcode(), Rcode::NOERROR);

        rr_cache.set_rcode(Rcode::FORMERR);

        assert_eq!(rr_cache.get_rcode(), Rcode::FORMERR);
    }

    #[test]
    fn set_and_get_resource_record() {
        let ip_address: IpAddr = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata.clone());
        resource_record.set_type_code(Rrtype::A);

        let mut rr_cache = RRStoredData::new(resource_record);

        assert_eq!(u16::from(rr_cache.resource_record.get_rtype()), 1);

        let second_ip_address: IpAddr = IpAddr::from([127, 0, 0, 0]);
        let mut second_a_rdata = ARdata::new();

        second_a_rdata.set_address(second_ip_address);
        let second_rdata = Rdata::A(second_a_rdata);

        let mut second_resource_record = ResourceRecord::new(second_rdata);
        second_resource_record.set_type_code(Rrtype::NS);

        rr_cache.set_resource_record(second_resource_record);

        assert_eq!(u16::from(rr_cache.get_resource_record().get_rtype()), 2);
    }

    #[test]
    fn set_and_get_response_time() {
        let ip_address: IpAddr = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rrtype::A);

        let mut rr_cache = RRStoredData::new(resource_record);

        assert_eq!(rr_cache.get_response_time(), 5000);

        rr_cache.set_response_time(2000);

        assert_eq!(rr_cache.get_response_time(), 2000);
    }

    #[test]
    fn set_and_get_last_use() {
        let ip_address: IpAddr = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(Rrtype::A);

        let rr_cache = RRStoredData::new(resource_record);

        let now = Utc::now();

        assert_eq!(rr_cache.get_creation_time().timestamp(), now.timestamp());
    }
}
