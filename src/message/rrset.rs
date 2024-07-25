use crate::message::Rtype;
use crate::message::Rclass;
use crate::message::resource_record::ResourceRecord;
use std::collections::HashSet;

/// Represents a set of resource records (RRset).
#[derive(Debug)]
pub struct RRset {
    /// The name of the domain associated with this RRset.
    name: String,
    /// The type of resource record in this RRset.
    rtype: Rtype,
    /// The class of resource record in this RRset.
    rclass: Rclass,
    /// The time to live (TTL) value for records in this RRset.
    ttl: u32,
    /// The set of resource records belonging to this RRset.
    records: HashSet<ResourceRecord>,
}

impl RRset {
    /// Creates a new RRset.
    pub fn new(name: String, rtype: Rtype, rclass: Rclass, ttl: u32) -> RRset {
        RRset {
            name,
            rtype,
            rclass,
            ttl,
            records: HashSet::new(),
        }
    }

    /// Adds a resource record to this RRset.
    pub fn add_record(&mut self, record: ResourceRecord) {
        self.records.insert(record);
    }

    /// Gets the name of the domain associated with this RRset.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the type of resource record in this RRset.
    pub fn get_type(&self) -> Rtype {
        self.rtype
    }

    /// Gets the class of resource record in this RRset.
    pub fn get_class(&self) -> Rclass {
        self.rclass
    }

    /// Gets the time to live (TTL) value for records in this RRset.
    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    /// Gets the set of resource records belonging to this RRset.
    pub fn get_records(&self) -> &HashSet<ResourceRecord> {
        &self.records
    }

    /// Gets the labels of the domain associated with this RRset.
    pub fn get_labels(&self) -> usize {
        self.name.split('.').count()
    }

    /// Serializes the RRset to a byte array for signing.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for record in &self.records {
            bytes.extend(record.to_bytes());  // Assuming ResourceRecord has a to_bytes method
        }
        bytes
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Rtype;
    use crate::message::Rclass;
    use crate::message::resource_record::{ResourceRecord, Rdata, ARdata, NsRdata, CnameRdata};
    use std::net::IpAddr;
    use std::collections::HashSet;

    #[test]
    fn test_create_rrset() {
        let name = "example.com".to_string();
        let rtype = Rtype::A;
        let rclass = Rclass::IN;
        let ttl = 3600;

        let rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        assert_eq!(rrset.get_name(), &name);
        assert_eq!(rrset.get_type(), Rtype::A);
        assert_eq!(rrset.get_class(), Rclass::IN);
        assert_eq!(rrset.get_ttl(), 3600);
        assert_eq!(rrset.get_labels(), 2);
        assert!(rrset.get_records().is_empty());
    }

    #[test]
    fn test_add_record() {
        let name = "example.com".to_string();
        let rtype = Rtype::A;
        let rclass = Rclass::IN;
        let ttl = 3600;

        let mut rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        let mut a_rdata = Rdata::A(ARdata::new());
        match a_rdata {
            Rdata::A(ref mut val) => val.set_address(IpAddr::from([127, 0, 0, 1])),
            _ => unreachable!(),
        }

        let record = ResourceRecord::new(a_rdata);
        rrset.add_record(record);

        assert_eq!(rrset.get_records().len(), 1);
    }

    #[test]
    fn test_get_name() {
        let name = "example.com".to_string();
        let rtype = Rtype::A;
        let rclass = Rclass::IN;
        let ttl = 3600;

        let rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        assert_eq!(rrset.get_name(), &name);
    }

    #[test]
    fn test_get_type() {
        let name = "example.com".to_string();
        let rtype = Rtype::NS;
        let rclass = Rclass::IN;
        let ttl = 3600;

        let rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        assert_eq!(rrset.get_type(), Rtype::NS);
    }

    #[test]
    fn test_get_class() {
        let name = "example.com".to_string();
        let rtype = Rtype::MX;
        let rclass = Rclass::CH;
        let ttl = 3600;

        let rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        assert_eq!(rrset.get_class(), Rclass::CH);
    }

    #[test]
    fn test_get_ttl() {
        let name = "example.com".to_string();
        let rtype = Rtype::A;
        let rclass = Rclass::IN;
        let ttl = 7200;

        let rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        assert_eq!(rrset.get_ttl(), 7200);
    }

    #[test]
    fn test_get_labels() {
        let name = "sub.example.com".to_string();
        let rtype = Rtype::A;
        let rclass = Rclass::IN;
        let ttl = 3600;

        let rrset = RRset::new(name.clone(), rtype, rclass, ttl);

        assert_eq!(rrset.get_labels(), 3);
    }
}
