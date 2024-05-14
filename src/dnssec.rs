use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::message::{rdata::rrsig_rdata::RRSIGRdata, DnsMessage};
use crate::message::rdata::Rdata;
use crate::message::rdata::rrsig::RRSIG;
use crate::message::rdata::dnskey::DNSKEY;
use openssl::rsa::Rsa;

// A.1. DNSSEC Algorithm Types
//  The DNSKEY, RRSIG, and DS RRs use an 8-bit number to identify the
//  security algorithm being used. These values are stored in the
//  "Algorithm number" field in the resource record RDATA.
//  Some algorithms are usable only for zone signing (DNSSEC), some only
//  for transaction security mechanisms (SIG(0) and TSIG), and some for
//  both. Those usable for zone signing may appear in DNSKEY, RRSIG, and
//  DS RRs. Those usable for transaction security would be present in
//  SIG(0) and KEY RRs, as described in [RFC2931].
//  Zone
//  Value Algorithm [Mnemonic] Signing References Status
//  ----- -------------------- --------- ---------- ---------
//  0 reserved
//  1 RSA/MD5 [RSAMD5] n [RFC2537] NOT RECOMMENDED
//  2 Diffie-Hellman [DH] n [RFC2539] -
//  3 DSA/SHA-1 [DSA] y [RFC2536] OPTIONAL
//  4 Elliptic Curve [ECC] TBA -
//  5 RSA/SHA-1 [RSASHA1] y [RFC3110] MANDATORY
//  252 Indirect [INDIRECT] n -
//  253 Private [PRIVATEDNS] y see below OPTIONAL
//  254 Private [PRIVATEOID] y see below OPTIONAL
//  255 reserved

//  The following algorithm types are defined in this document:
//  0 - RSA/MD5 [RFC2537]
//  1 - Diffie-Hellman [RFC2539]
//  2 - DSA/SHA-1 [RFC2536]
//  3 - Elliptic Curve [TBA]
//  4 - RSA/SHA-1 [RFC3110]
//  5 - Indirect [RFC4034]

fn get_algorithm_name(algorithm: u8) -> &'static str {
    match algorithm {
        1 => "RSA/MD5",
        2 => "Diffie-Hellman",
        3 => "DSA/SHA-1",
        4 => "Elliptic Curve",
        5 => "RSA/SHA-1",
        _ => "Unknown",
    }
}

// Once the resolver has validated the RRSIG RR as described in Section
//  5.3.1 and reconstructed the original signed data as described in
//  Section 5.3.2, the validator can attempt to use the cryptographic
//  signature to authenticate the signed data, and thus (finally!)
//  authenticate the RRset.

//  The Algorithm field in the RRSIG RR identifies the cryptographic
//  algorithm used to generate the signature. The signature itself is
//  contained in the Signature field of the RRSIG RDATA, and the public
//  key used to verify the signature is contained in the Public Key field
//  of the matching DNSKEY RR(s) (found in Section 5.3.1). [RFC4034]
//  provides a list of algorithm types and provides pointers to the
//  documents that define each algorithm’s use.

//The Funcion Verifies the zone's DNSKEY RR set by successfully decrypting the RR set's RRSig using the zone's Public Key signing key 

fn decrypt_signature(rrset: &Vec<Rdata>, rrsig: &RRSIG, dnskey: &DNSKEY) -> bool {
    //The Algorithm field in the RRSIG RR identifies the cryptographic algorithm used to generate the signature.
    algoritm = rrsig.algorithm;
    // The signature itself is contained in the Signature field of the RRSIG RDATA
    signature = rrsig.signature;
    // and the public key used to verify the signature is contained in the Public Key field of the matching DNSKEY RR(s)
    public_key = dnskey.public_key;

    // If the Labels field of the RRSIG RR is not equal to the number of labels in the RRset’s fully qualified owner name, then the RRset is invalid
    if rrsig.labels != rrset[0].owner.labels() {
        return false;
    }

//     If the resolver accepts the RRset as authentic, the validator MUST
//  set the TTL of the RRSIG RR and each RR in the authenticated RRset to
//  a value no greater than the minimum of:
//  o the RRset’s TTL as received in the response;
//  o the RRSIG RR’s TTL as received in the response;
//  o the value in the RRSIG RR’s Original TTL field; and
//  o the difference of the RRSIG RR’s Signature Expiration time and the
//  current time. 

    // If the Signer’s Name field in the RRSIG RR is not equal to the owner name of the RRset, then the RRset is invalid
    if rrsig.signer_name != rrset[0].owner {
        return false;
    }

    // If the Key Tag in the RRSIG RR does not match the Key Tag in the DNSKEY RR, then the RRset is invalid
    if rrsig.key_tag != dnskey.key_tag {
        return false;
    }



}