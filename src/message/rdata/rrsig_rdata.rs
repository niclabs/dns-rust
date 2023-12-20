use crate::message::resource_record::{FromBytes, ToBytes};
use crate::domain_name::DomainName;

#[derive(Clone, Debug, PartialEq)]
/// Struct for RRSIG Rdata
/// [RFC 4034](https://tools.ietf.org/html/rfc4034#section-3.1)
///                      1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |        Type Covered           |  Algorithm    |     Labels    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Original TTL                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                      Signature Expiration                     |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                      Signature Inception                      |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |            Key Tag            |                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+         Signer's Name         /
/// /                                                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                                                               /
/// /                            Signature                          /
/// /                                                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

pub struct RRSIGRdata {
    type_covered: String, // RR type mnemonic
    algorithm: u8, // Unsigned decimal integer
    labels: u8, // Unsigned decimal integer
    original_ttl: u32, // Unsigned decimal integer
    signature_expiration: u32, // Unsigned decimal integer 
    signature_inception: u32, // Unsigned decimal integer
    key_tag: u16, // Unsigned decimal integer
    signer_name: DomainName, // Domain name
    signature: String, // Base64 encoding of the signature
}

impl RRSIGRdata{
    /// Constructor for RRSIGRdata
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// ```
    pub fn new() -> RRSIGRdata{
        RRSIGRdata{
            type_covered: String::new(),
            algorithm: 0,
            labels: 0,
            original_ttl: 0,
            signature_expiration: 0,
            signature_inception: 0,
            key_tag: 0,
            signer_name: DomainName::new(),
            signature: String::new(),
        }
    }
}
