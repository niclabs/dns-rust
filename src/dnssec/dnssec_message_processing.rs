use crate::message::DnsMessage;
use crate::message::rdata::{rrsig_rdata, dnskey_rdata, ds_rdata, nsec_rdata, nsec3_rdata};

// DNSKEY, RRSIG, DS | NSEC, NSEC3 ?

fn extract_dnssec_records(dns_message: &DnsMessage) -> (Vec<DnskeyRdata>, Vec<RRSIGRdata>, Vec<DsRdata>) {
    let mut dnskey_records = Vec::new();
    let mut rrsig_records = Vec::new();
    let mut ds_records = Vec::new();

    for record in &dns_message.additional {
        match record.rdata {
            Rdata::DNSKEY(ref data) => {
                let dnskey_rdata = DnskeyRdata::new(data.flags, data.protocol, data.algorithm, data.public_key.clone());
                dnskey_records.push(dnskey_rdata);
            }
            Rdata::RRSIG(ref data) => {
                let rrsig_rdata = RrsigRdata::new(
                    data.type_covered,
                    data.algorithm,
                    data.labels,
                    data.original_ttl,
                    //datatime? expiration/inception
                    data.key_tag,
                    data.signer_name.clone(),
                    data.signature.clone()
                );
                rrsig_records.push(rrsig_rdata);
            }
            Rdata::DS(ref data) => {
                let ds_rdata = DsRdata::new(data.key_tag, data.algorithm, data.digest_type, data.digest.clone()
                );
                ds_records.push(ds_rdata);
            },
            _ => (),
        }
    }
    (dnskey_records, rrsig_records, ds_records)
}