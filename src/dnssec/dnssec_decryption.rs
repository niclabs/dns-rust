use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
use crypto::rsa::Rsa;
use crate::dnssec::RRset;
use crate::dnskey_rdata::DnskeyRdata;
use crate::rrsig_rdata::RRSIGRdata;

/// RFCs: [4033, 4034, 4035, 5702]

/// Verifies an RRset using the provided public key and RRSIG record.
/// Returns true if the verification is successful.
pub fn verify_rrset(rrset: &RRset, rrsig: &RRSIGRdata, public_key: &Rsa) -> Result<bool, &'static str> {
    let rrset_bytes = rrset.to_bytes();
    
    let mut hasher: Box<dyn Digest> = match rrsig.algorithm {
        1 => Box::new(Sha1::new()),
        2 => Box::new(Sha256::new()),
        _ => return Err("Algorithm not supported"),
    };
    
    hasher.input(&rrset_bytes);
    let hash = hasher.result_str();
    
    let signature = base64::decode(&rrsig.signature).map_err(|_| "Error while decoding signature")?;
    
    public_key.verify(hasher, &hash.as_bytes(), &signature).map_err(|_| "Verification failed")
}

