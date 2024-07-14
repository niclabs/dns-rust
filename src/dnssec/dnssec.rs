use crate::message::{DnsMessage, Rdata, ResourceRecord};
use crate::dnssec_message_processing::extract_dnssec_records;
use crate::rrset_signature::{verify_rrsig, verify_ds};
use crate::message::rdata::DnskeyRdata;
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
