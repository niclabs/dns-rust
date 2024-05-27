use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
use crypto::rsa::Rsa;
use crate::dnssec::RRset;
use crate::dnskey_rdata::DnskeyRdata;
use crate::rrsig_rdata::RRSIGRdata;
use base64;

/// RFCs: [4033, 4034, 4035, 4509]

/// Signs a RRset using the private_key given.
/// Returns a RRSIG that contains the signature.
pub fn sign_rrset(rrset: &RRset, private_key: &Rsa, algorithm: u8) -> Result<RRSIGRdata, &'static str> {
    let rrset_bytes = rrset.to_bytes();
    
    let mut hasher: Box<dyn Digest> = match algorithm {
        1 => Box::new(Sha1::new()),
        2 => Box::new(Sha256::new()),
        _ => return Err("Algorithm not supported"),
    };
    
    hasher.input(&rrset_bytes);
    let hash = hasher.result_str();
    
    let signature = private_key.sign(hasher, &hash.as_bytes()).map_err(|_| "Error while signing")?;
    
    let rrsig = RRSIGRdata {
        type_covered: rrset.get_type(),
        algorithm,
        labels: rrset.get_labels(),
        original_ttl: rrset.get_ttl(),
        signature_expiration: get_expiration_time(),
        signature_inception: get_inception_time(),
        key_tag: calculate_key_tag(private_key),
        signer_name: rrset.get_name().clone(),
        signature: base64::encode(&signature),
    };
    
    Ok(rrsig)
}

// Gets the expiration time for the signature.
fn get_expiration_time() -> u32 {
    // Supposing sign validity = 1 day (86400 seconds)
    let now = std::time::SystemTime::now();
    let expiration_duration = std::time::Duration::new(86400, 0);
    let expiration_time = now + expiration_duration;
    expiration_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32
}

// Gets the inception time for the signature.
fn get_inception_time() -> u32 {
    // Assuming current time
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32
}

// Calculates the key tag for the public key.
fn calculate_key_tag(private_key: &Rsa) -> u16 {
    let public_key_der = private_key.to_public_key_der().unwrap();
    let mut hasher = Sha1::new();
    hasher.input(&public_key_der);
    let hash = hasher.result_str();
    u16::from_be_bytes([hash.as_bytes()[18], hash.as_bytes()[19]])
}

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

