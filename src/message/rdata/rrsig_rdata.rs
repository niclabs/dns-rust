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
    /// Getter for type_covered
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let type_covered = rrsig_rdata.get_type_covered();
    /// ```
    pub fn get_type_covered(&self) -> String{
        self.type_covered.clone()
    }

    /// Getter for algorithm
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let algorithm = rrsig_rdata.get_algorithm();
    /// ```
    pub fn get_algorithm(&self) -> u8{
        self.algorithm.clone()
    }

    /// Getter for labels
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let labels = rrsig_rdata.get_labels();
    /// ```
    pub fn get_labels(&self) -> u8{
        self.labels.clone()
    }

    /// Getter for original_ttl
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let original_ttl = rrsig_rdata.get_original_ttl();
    /// ```
    pub fn get_original_ttl(&self) -> u32{
        self.original_ttl.clone()
    }

    /// Getter for signature_expiration
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let signature_expiration = rrsig_rdata.get_signature_expiration();
    /// ```
    pub fn get_signature_expiration(&self) -> u32{
        self.signature_expiration.clone()
    }

    /// Getter for signature_inception
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let signature_inception = rrsig_rdata.get_signature_inception();
    /// ```
    pub fn get_signature_inception(&self) -> u32{
        self.signature_inception.clone()
    }

    /// Getter for key_tag
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let key_tag = rrsig_rdata.get_key_tag();
    /// ```
    pub fn get_key_tag(&self) -> u16{
        self.key_tag.clone()
    }

    /// Getter for signer_name
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let signer_name = rrsig_rdata.get_signer_name();
    /// ```
    pub fn get_signer_name(&self) -> DomainName{
        self.signer_name.clone()
    }

    /// Getter for signature
    /// 
    /// # Example
    /// 
    /// ```
    /// let rrsig_rdata = RRSIGRdata::new();
    /// let signature = rrsig_rdata.get_signature();
    /// ```
    pub fn get_signature(&self) -> String{
        self.signature.clone()
    }
}
