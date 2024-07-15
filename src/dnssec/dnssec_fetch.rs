use crate::message::DnsMessage;
use crate::message::rdata::Rdata;
use crate::message::rdata::dnskey_rdata::DnskeyRdata;
use crate::message::resource_record::ResourceRecord;
use crate::dnssec::dnssec_message_processing::extract_dnssec_records;
use crate::dnssec::rrset_signature::{verify_rrsig, verify_ds};

use crate::client::client_error::ClientError;

pub async fn fetch_dnskey_records(dns_response: &DnsMessage) -> Result<Vec<DnskeyRdata>, ClientError> {
    let mut dnskey_records = Vec::new();

    for record in dns_response.get_answer() {
        if let Rdata::DNSKEY(dnskey) = &record.get_rdata() {
            dnskey_records.push(dnskey.clone());
        }
    }

    Ok(dnskey_records)
}
