use crate::message::DnsMessage;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;

pub fn extract_dnssec_records(dns_response: &DnsMessage) -> (Vec<ResourceRecord>, Vec<ResourceRecord>) {
    let answers = dns_response.get_answer();
    let additionals = dns_response.get_additional();

    let mut dnskey_records = Vec::new();
    let mut rrsig_records = Vec::new();

    for record in answers.iter().chain(additionals.iter()) {
        match record.get_rdata() {
            Rdata::DNSKEY(_) => dnskey_records.push(record.clone()),
            Rdata::RRSIG(_) => rrsig_records.push(record.clone()),
            _ => {}
        }
    }

    (dnskey_records, rrsig_records)
}
