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

// Settters for RRSIGRdata
impl RRSIGRdata{
    /// Setter for type_covered
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_type_covered("A".to_string());
    /// ```
    pub fn set_type_covered(&mut self, type_covered: String) {
        self.type_covered = type_covered;
    }

    /// Setter for algorithm
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_algorithm(5);
    /// ```
    pub fn set_algorithm(&mut self, algorithm: u8) {
        self.algorithm = algorithm;
    }

    /// Setter for labels
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_labels(2);
    /// ```
    pub fn set_labels(&mut self, labels: u8) {
        self.labels = labels;
    }

    /// Setter for original_ttl
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_original_ttl(3600);
    /// ```
    pub fn set_original_ttl(&mut self, original_ttl: u32) {
        self.original_ttl = original_ttl;
    }

    /// Setter for signature_expiration
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_signature_expiration(1630435200);
    /// ```
    pub fn set_signature_expiration(&mut self, signature_expiration: u32) {
        self.signature_expiration = signature_expiration;
    }

    /// Setter for signature_inception
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_signature_inception(1630435200);
    /// ```
    pub fn set_signature_inception(&mut self, signature_inception: u32) {
        self.signature_inception = signature_inception;
    }

    /// Setter for key_tag
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_key_tag(1234);
    /// ```
    pub fn set_key_tag(&mut self, key_tag: u16) {
        self.key_tag = key_tag;
    }

    /// Setter for signer_name
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_signer_name(DomainName::new("example.com").unwrap());
    /// ```
    pub fn set_signer_name(&mut self, signer_name: DomainName) {
        self.signer_name = signer_name;
    }

    /// Setter for signature
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut rrsig_rdata = RRSIGRdata::new();
    /// rrsig_rdata.set_signature("abcdefg".to_string());
    /// ```
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }
}

impl RRSIGRdata{
}
