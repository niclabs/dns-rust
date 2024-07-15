use sha2::{Sha256, Digest};
use crypto::digest::Digest as RustDigest;
use crypto::sha1::Sha1;
use base64::encode;
use crate::message::rdata::Rdata;
use crate::message::rdata::dnskey_rdata::DnskeyRdata;
use crate::message::rdata::rrsig_rdata::{RRSIGRdata};
use crate::message::resource_record::ResourceRecord;
use crate::client::client_error::ClientError;

pub fn verify_rrsig(rrsig: &RRSIGRdata, dnskey: &DnskeyRdata, rrset: &[ResourceRecord]) -> Result<bool, ClientError> {
    let mut rrsig_data = Vec::new();
    rrsig_data.extend_from_slice(&rrsig.get_type_covered().to_be_bytes());
    rrsig_data.push(rrsig.get_algorithm());
    rrsig_data.push(rrsig.get_labels());
    rrsig_data.extend_from_slice(&rrsig.get_original_ttl().to_be_bytes());
    rrsig_data.extend_from_slice(&rrsig.get_signature_expiration().to_be_bytes());
    rrsig_data.extend_from_slice(&rrsig.get_signature_inception().to_be_bytes());
    rrsig_data.extend_from_slice(&rrsig.get_key_tag().to_be_bytes());
    rrsig_data.extend_from_slice(rrsig.get_signer_name().to_bytes()?);

    let mut rrset_sorted = rrset.to_vec();
    rrset_sorted.sort_by(|a, b| a.get_name().cmp(&b.get_name()));

    for rr in rrset_sorted.iter() {
        rrsig_data.extend_from_slice(rr.get_name().to_bytes()?);
        rrsig_data.extend_from_slice(&rr.get_ttl.to_be_bytes());
        rrsig_data.extend_from_slice(&(rr.get_rdata().to_bytes().len() as u16).to_be_bytes());
        rrsig_data.extend_from_slice(&rr.get_rdata().to_bytes()?);
    }

    let signature = rrsig.get_signature().clone();
    let hashed = Sha256::digest(&rrsig_data);

    match dnskey.algorithm {
        3 | 5 => {
            // (DSA/RSA)/SHA1
            let mut sha1 = Sha1::new();
            sha1.input(&rrsig_data);
            let digest = sha1.result_str();
            Ok(digest == encode(&signature))
        },
        8 => {
            // RSA/SHA256
            Ok(encode(&hashed) == encode(&signature))
        },
        _ => Err(ClientError::new("Unknown DNSKEY algorithm")),
    }
}

pub fn verify_ds(ds_record: &ResourceRecord, dnskey: &DnskeyRdata) -> Result<bool, ClientError> {
    if let Rdata::DS(ds_rdata) = &ds_record.get_rdata() {
        let dnskey_bytes = dnskey.to_bytes()?;
        let hashed_key = match ds_rdata.algorithm {
            1 => {
                let mut hasher = Sha1::new();
                hasher.input(&dnskey_bytes);
                hasher.result_str()
            },
            2 => {
                let hashed = Sha256::digest(&dnskey_bytes);
                encode(&hashed)
            },
            _ => return Err(ClientError::new("Unknown DS algorithm")),
        };

        Ok(ds_rdata.digest == hashed_key)
    } else {
        Err(ClientError::new("Provided record is not a DS record"))
    }
}
