// src/dnssec.rs

pub mod dnssec {
    use super::message::{DnskeyRdata, RrsigRdata, DsRdata, DnsMessage, DnsError};
    use sha2::{Sha256, Sha512, Digest};
    use hmac::{Hmac, Mac, NewMac};
    use rust_crypto::rsa::{RSAPublicKey, RSAVerify};
    use base64::decode;

    pub fn fetch_dnskey_records(domain: &str) -> Result<Vec<DnskeyRdata>, DnsError> {
        // Completar
        Ok(vec![DnskeyRdata::new(256, 3, 8, vec![0x03, 0x01, 0x00, 0x01])])
    }

    pub fn verify_rrsig_signature(
        rrsig: &RrsigRdata,
        dnskey: &DnskeyRdata,
        signed_data: &[u8]
    ) -> Result<bool, DnsError> {
        rsa_verify(&dnskey.public_key, &rrsig.signature, signed_data)
    }

    pub fn verify_ds_record(
        ds: &DsRdata,
        dnskey: &DnskeyRdata
    ) -> Result<bool, DnsError> {
        let digest = compute_digest(ds.algorithm, &dnskey.to_bytes())?;
        Ok(digest == ds.digest)
    }

    fn rsa_verify(
        public_key: &[u8],
        signature: &[u8],
        signed_data: &[u8]
    ) -> Result<bool, DnsError> {
        let public_key = RSAPublicKey::from_der(public_key).map_err(|_| DnsError::VerificationFailed)?;
        public_key.verify(signature, signed_data).map_err(|_| DnsError::VerificationFailed)?;

        Ok(true)
    }

    fn compute_digest(
        algorithm: u8,
        data: &[u8]
    ) -> Result<Vec<u8>, DnsError> {
        match algorithm {
            1 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            },
            2 => {
                let mut hasher = Sha512::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            },
            _ => Err(DnsError::UnsupportedAlgorithm),
        }
    }
}
