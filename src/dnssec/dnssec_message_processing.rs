use crate::message::rdata::Rdata;
use crate::message::rdata::rrsig_rdata::RRSIGRdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::rrset::RRset;
use crate::message::rrtype::Rrtype;
use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;

use crate::message::resource_record::ToBytes;

pub fn extract_signed_rrsets(rrs: &Vec<ResourceRecord>) -> Option<Vec<(ResourceRecord, RRset)>> {
    // check if there exists RRSIG records
    let rrsigs= rrs.iter().filter(
        |rr|
            rr.get_rtype() == Rrtype::RRSIG
    );
    if rrsigs.clone().count() == 0 {
        return None;
    }
    let mut result = vec![];

    for rrsig_rr in rrsigs {
        let rclass = rrsig_rr.get_rclass();
        let Rdata::RRSIG(rrsig_data) = rrsig_rr.get_rdata() else {todo!()};
        let type_coverd = rrsig_data.get_type_covered();
        let name = rrsig_rr.get_name();
        let rrs_filtered = RRset::get_rrset_from_rrs(rrs.clone(),name, type_coverd, rclass);
        if rrs_filtered.is_none(){
            panic!("This should not happen! if a rrsig arrives, then the associated rrs should exist")
        }
        let rrs_filtered = rrs_filtered?;
        result.push((rrsig_rr.clone(), rrs_filtered));
    }
    return Some(result);
}
#[cfg(test)]
mod dnssec_message_processing_tests {
    use std::net::IpAddr;
    use crate::domain_name::DomainName;
    use crate::message;
    use crate::message::rclass::Rclass;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::{rrsig_rdata, Rdata};
    use crate::message::resource_record::{ResourceRecord, ToBytes};
    use crate::message::rrtype::Rrtype;

    #[test]
    pub fn test_extract_signed_rrsets() {
        use std::net::IpAddr;
        use crate::message::{self, rdata::{a_rdata::ARdata, rrsig_rdata}, resource_record::ResourceRecord};
        use crate::message::question::Question;
        use super::*;

        let mut message = message::DnsMessage::new();
        let mut header = message.get_header();
        header.set_qr(true);
        header.set_rd(true);
        message.set_header(header);
        let mut question = crate::message::question::Question::new();
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
        //rr.set_ttl(2700);
        rr.set_ttl(3600);
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
        let mut rrsig_rr = ResourceRecord::new(rdata);
        rrsig_rr.set_rdlength(rrsig_rdata.to_bytes().len() as u16);
        rrsig_rr.set_name(DomainName::new_from_str("example.com"));
        rrsig_rr.set_type_code(Rrtype::RRSIG);
        rrsig_rr.set_rclass(Rclass::IN);
        rrsig_rr.set_ttl(3600);
        ans_rrs.push(rrsig_rr.clone());
        // update the message
        message.set_answer(ans_rrs.clone());
        message.update_header_counters();
        let xd = extract_signed_rrsets(&ans_rrs);
        let mut xd = xd.expect("This should not happen!");

        assert_eq!(xd.len(), 1);
        let (rrsig_rr_extracted, rrset) = xd.pop().unwrap();
        let mut expected_records = vec![];
        expected_records.push((4u16, Rdata::A(ARdata::new_from_addr(IpAddr::from([93,184,215,14])))));
        expected_records.push((4u16, Rdata::A(ARdata::new_from_addr(IpAddr::from([93,184,215,15])))));

        assert_eq!(rrsig_rr_extracted, rrsig_rr);
        assert_eq!(rrset.get_records(),
                   expected_records);
    }
}
