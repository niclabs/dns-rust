use crate::client::ClientUDPConnection;
use crate::message::{DnsMessage, ResourceRecord};
use crate::dnssec_message_processing::extract_dnssec_records;
use crate::rrset_signature::{verify_rrsig, verify_ds};
use crate::dnskey_rdata::DnskeyRdata;
use crate::client::client_error::ClientError;
use std::net::IpAddr;
use tokio::time::Duration;

pub async fn fetch_dnskey_records(domain: &str, server_addr: IpAddr, timeout_duration: Duration) -> Result<Vec<DnskeyRdata>, ClientError> {
    let conn = ClientUDPConnection::new(server_addr, timeout_duration);

    let dns_query = DnsMessage::new_query_message(
        domain.into(),
        Qtype::DNSKEY,
        Qclass::IN,
        0,
        false,
        1,
    );

    let response = conn.send(dns_query).await?;

    let dns_response = DnsMessage::from_bytes(&response)?;
    let mut dnskey_records = Vec::new();

    for record in dns_response.get_answer() {
        if let Rdata::DNSKEY(dnskey) = &record.get_rdata() {
            dnskey_records.push(dnskey.clone());
        }
    }

    Ok(dnskey_records)
}
