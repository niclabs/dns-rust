
use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ToBytes;
use crate::message::rrtype::Rrtype;

use super::resource_record::ResourceRecord;


#[derive(Clone, PartialEq, Debug)]
/*
    RFC 2181
    Each DNS Resource Record (RR) has a label, class, type, and data.  It
    is meaningless for two records to ever have label, class, type and
    data all equal - servers should suppress such duplicates if
    encountered.  It is however possible for most record types to exist
    with the same label, class and type, but with different data.  Such a
    group of records is hereby defined to be a Resource Record Set
    (RRSet).

    RFC 4034
    For the purposes of DNS security, RRs with the same owner name,class and type are sorted by
    treating the RDATA portion of the canonical form of each RR as a left-justified unsigned octet
    sequence in which the absence of an octet sorts before a zero octet.
*/
pub struct RRset {
    name: DomainName,
    rrtype: Rrtype,
    rclass: Rclass,
    /*
    RFC 2181
    Resource Records [(RR) with a label, class, type, and data] also have a time to live (TTL).
    It is possible for the RRs in an RRSet [= same label, class, and type; different data] to have different TTLs.
    No uses for this have been found that cannot be better accomplished in other ways.
    This can, however, cause partial replies (not marked "truncated") from a caching server, where the TTLs for some but not all the RRs in the RRSet have expired.
    Consequently the use of **differing TTLs in an RRSet is hereby deprecated**, the TTLs of all RRs in an RRSet MUST be the same.
    */
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

    // Add a record to the records
    pub fn add_record(&mut self, rdlen: u16, rdata: Rdata) {
        self.records.push((rdlen, rdata));
    }

    pub fn add_rr(&mut self, rr: ResourceRecord) -> Result<(), String> {
        // check if the rr type, class and name match the rrset
        if rr.get_name() != self.name {
            return Err("The name of the resource record does not match the name of the rrset".to_string());
        }
        if rr.get_rtype()!= self.rrtype {
            return Err("The rrtype of the resource record does not match the rrtype of the rrset".to_string());
        }
        if rr.get_rclass() != self.rclass {
            return Err("The rclass of the resource record does not match the rclass of the rrset".to_string());
        }
        let rdlen = rr.get_rdlength();
        let rdata = rr.get_rdata().clone();
        self.add_record(rdlen, rdata);
        Ok(())
    }

    pub fn get_rrset_from_rrs(rrs: Vec<ResourceRecord>, name: DomainName, rrtype: Rrtype, rclass: Rclass) -> Option<RRset> {
        let mut rrset = RRset::new();
        rrset.set_name(name.clone());
        rrset.set_rrtype(rrtype);
        rrset.set_rclass(rclass);
        let rrs_filtered = rrs.iter().filter(
            |rr| 
            rr.get_name() == name && 
            rr.get_rtype() == rrtype &&
            rr.get_rclass() == rclass
        );
        let mut minttl = u32::MAX;
        for rr in rrs_filtered {
            rrset.add_rr(rr.clone()).expect("This should not happen!");
            minttl = minttl.min(rr.get_ttl());
        }
        if rrset.get_records().is_empty() {
            return None;
        }
        rrset.set_ttl(minttl);
        Some(rrset)
    }
}

#[cfg(test)]
mod rrset_test {
    use std::net::IpAddr;
    use crate::message::{self, rdata::{a_rdata::ARdata, rrsig_rdata}, resource_record::ResourceRecord};
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
    #[test]
    fn see_rrsets_from_message() {
        let mut message = message::DnsMessage::new();
        let mut header = message.get_header();
        header.set_qr(true);
        header.set_rd(true);
        message.set_header(header);
        let mut question = message::Question::new();
        question.set_qname(DomainName::new_from_str("example.com"));
        question.set_rrtype(Rrtype::A);
        question.set_rclass(Rclass::IN);
        message.set_question(question);
        // Add an answer
        let mut ans_rrs = vec![];
        let ardata = ARdata::new_from_addr(IpAddr::from([93,184,215,14]));
        let rdata = Rdata::A(ardata.clone());
        let mut rr = ResourceRecord::new(rdata);
        rr.set_rdlength(ardata.to_bytes().len() as u16);
        rr.set_name(DomainName::new_from_str("example.com"));
        rr.set_type_code(Rrtype::A);
        rr.set_rclass(Rclass::IN);
        rr.set_ttl(3600);
        ans_rrs.push(rr);

        let ardata = ARdata::new_from_addr(IpAddr::from([93,184,215,15]));
        let rdata = Rdata::A(ardata.clone());
        let mut rr = ResourceRecord::new(rdata);
        rr.set_rdlength(ardata.to_bytes().len() as u16);
        rr.set_name(DomainName::new_from_str("example.com"));
        rr.set_type_code(Rrtype::A);
        rr.set_rclass(Rclass::IN);
        // This behaviour is deprecated! all the ttls should be the same in an RRset
        rr.set_ttl(2700);
        ans_rrs.push(rr);

        // add answer signature
        let mut rrsig_rdata = rrsig_rdata::RRSIGRdata::new();
        rrsig_rdata.set_type_covered(Rrtype::A);
        rrsig_rdata.set_algorithm(13); // ECDSA Curve P-256 with SHA-256
        rrsig_rdata.set_labels(2);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1731875962);
        rrsig_rdata.set_signature_inception(1730058866);
        rrsig_rdata.set_key_tag(42464);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
        rrsig_rdata.set_signature(b"\x8b\x9e\x3b\x5c\x50\x07\x44\xc2\xe4\xb1\xec\x64\x0b\xd8\xf5\xf1\x8b\xc3\x72\xc0\xd2\x13\x54\x8d\x56\x8a\xc5\x8a\x12\xb9\x99\x85\x89\xff\x01\xb8\xce\xa8\x77\x10\xb3\x89\xfc\x78\x95\x5d\x0d\x21\x76\x68\x05\xbc\xc8\xf5\xe9\x76\xcf\x40\x99\x2a\x20\x98\xc8\xd5".to_vec());
        let rdata = Rdata::RRSIG(rrsig_rdata.clone());
        let mut rr = ResourceRecord::new(rdata);
        rr.set_rdlength(rrsig_rdata.to_bytes().len() as u16);
        rr.set_name(DomainName::new_from_str("example.com"));
        rr.set_type_code(Rrtype::RRSIG);
        rr.set_rclass(Rclass::IN);
        rr.set_ttl(3600);
        ans_rrs.push(rr);
        // update the message
        message.set_answer(ans_rrs);
        message.update_header_counters();
        let rrset = RRset::get_rrset_from_rrs(message.get_answer(), DomainName::new_from_str("example.com"), Rrtype::A, Rclass::IN).unwrap();
        let mut expected_rrset = RRset::new();
        expected_rrset.set_name(DomainName::new_from_str("example.com"));
        expected_rrset.set_rrtype(Rrtype::A);
        expected_rrset.set_rclass(Rclass::IN);
        expected_rrset.set_ttl(2700);
        expected_rrset.set_records(vec![(4, Rdata::A(ARdata::new_from_addr(IpAddr::from([93,184,215,14])))),
                                        (4, Rdata::A(ARdata::new_from_addr(IpAddr::from([93,184,215,15]))))]);
        assert_eq!(expected_rrset, rrset)
    }
}