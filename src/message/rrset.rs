
use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ToBytes;
use crate::message::rrtype::Rrtype;
/*
RFC 2181
Resource Records [(RR) with a label, class, type, and data] also have a time to live (TTL).
It is possible for the RRs in an RRSet [= same label, class, and type; different data] to have different TTLs.
No uses for this have been found that cannot be better accomplished in other ways.
This can, however, cause partial replies (not marked "truncated") from a caching server, where the TTLs for some but not all the RRs in the RRSet have expired.
Consequently the use of **differing TTLs in an RRSet is hereby deprecated**, the TTLs of all RRs in an RRSet MUST be the same.
*/
pub struct RRset {
    name: DomainName,
    rrtype: Rrtype,
    rclass: Rclass,
    ttl: u32,
    //Pair rdlen, rdata
    records: Vec<(u16, Rdata)>
}

impl ToBytes for RRset {
    // This might be for the digest see https://datatracker.ietf.org/doc/html/rfc4035#section-5.3.1
    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }
}

impl RRset {
    pub fn new() -> RRset {
        RRset {
            name: DomainName::new(),
            rrtype: Rrtype::A,
            rclass: Rclass::IN,
            ttl: 0,
            records: Vec::new()
        }
    }
    // Getters

    pub fn get_name(&self) -> DomainName {
        self.name.clone()
    }
    pub fn get_rrtype(&self) -> Rrtype {
        self.rrtype
    }
    pub fn get_rclass(&self) -> Rclass {
        self.rclass
    }
    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }
    pub fn get_records(&self) -> Vec<(u16, Rdata)> {
        self.records.clone()
    }


    // Setters
    pub fn set_name(&mut self, name: DomainName) {
        self.name = name;
    }
    pub fn set_rrtype(&mut self, rrtype: Rrtype) {
        self.rrtype = rrtype;
    }
    pub fn set_rclass(&mut self, rclass: Rclass) {
        self.rclass = rclass;
    }
    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl;
    }
    pub fn set_records(&mut self, records: Vec<(u16, Rdata)>) {
        self.records = records;
    }
    
}

#[cfg(test)]
mod rrset_test {
    use std::net::IpAddr;
    use crate::message::rdata::a_rdata::ARdata;
    use super::*;
    #[test]
    fn constructor_test() {
        let rrset = RRset::new();
        assert_eq!(rrset.name, DomainName::new());
        assert_eq!(rrset.rrtype, Rrtype::A);
        assert_eq!(rrset.rclass, Rclass::IN);
        assert_eq!(rrset.ttl, 0);
        assert_eq!(rrset.records, Vec::new());
    }
    #[test]
    fn constructor_test_2(){
        let mut vec= vec![];
        let a_data_1 = ARdata::new_from_addr(IpAddr::from([192,168,0,1]));
        let rdata1 = (4u16, Rdata::A(a_data_1));
        vec.push(rdata1);
        let a_data_2 = ARdata::new_from_addr(IpAddr::from([192,168,0,100]));
        let rdata2 = (4u16, Rdata::A(a_data_2));
        vec.push(rdata2);
        let ttl = 3600;
        let domain_name = DomainName::new_from_str("example.com");
        let rrtype = Rrtype::A;
        let rclass = Rclass::IN;

        let mut rrset = RRset::new(
        );
        rrset.set_name(domain_name.clone());
        rrset.set_rrtype(rrtype);
        rrset.set_rclass(rclass);
        rrset.set_ttl(ttl);
        rrset.set_records(vec.clone());

        assert_eq!(rrset.get_name(), domain_name);
        assert_eq!(rrset.get_rrtype(), rrtype);
        assert_eq!(rrset.get_rclass(), rclass);
        assert_eq!(rrset.get_ttl(), ttl);
        assert_eq!(rrset.get_records(), vec);
    }
}