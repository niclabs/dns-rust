use crate::message::resource_record::{FromBytes, ToBytes};
use crate::domain_name::DomainName;
use crate::message::type_rtype::Rtype;

use std::fmt;

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
    type_covered: Rtype, // RR type mnemonic
    algorithm: u8, // Unsigned decimal integer
    labels: u8, // Unsigned decimal integer, represents the number of layers in the siger name
    original_ttl: u32, // Unsigned decimal integer
    signature_expiration: u32, // Unsigned decimal integer 
    signature_inception: u32, // Unsigned decimal integer
    key_tag: u16, // Unsigned decimal integer
    signer_name: DomainName, // Domain name
    signature: String, // Base64 encoding of the signature
}

impl ToBytes for RRSIGRdata {
    /// Returns a `Vec<u8>` of bytes that represents the RRSIG RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let type_covered = u16::from(self.type_covered.clone());
        bytes.extend_from_slice(&type_covered.to_be_bytes());

        bytes.push(self.algorithm);
        bytes.push(self.labels);
        bytes.extend_from_slice(&self.original_ttl.to_be_bytes());
        bytes.extend_from_slice(&self.signature_expiration.to_be_bytes());
        bytes.extend_from_slice(&self.signature_inception.to_be_bytes());
        bytes.extend_from_slice(&self.key_tag.to_be_bytes());

        let signer_name = self.signer_name.to_bytes();
        bytes.extend_from_slice(&signer_name);

        let signature = self.signature.clone();
        bytes.extend_from_slice(&signature.into_bytes());

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for RRSIGRdata {
    /// Creates a new `RRSIGRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len <= 18 {
            return Err("Format Error");
        }

        let mut rrsig_rdata = RRSIGRdata::new();

        let array_bytes = [bytes[0], bytes[1]];
        let type_covered_int = u16::from_be_bytes(array_bytes);
        let type_covered = Rtype::from_int_to_rtype(type_covered_int);
        rrsig_rdata.set_type_covered(type_covered);

        let algorithm = bytes[2];
        rrsig_rdata.set_algorithm(algorithm);

        let labels = bytes[3];
        rrsig_rdata.set_labels(labels);

        let array_bytes = [bytes[4], bytes[5], bytes[6], bytes[7]];
        let original_ttl = u32::from_be_bytes(array_bytes);
        rrsig_rdata.set_original_ttl(original_ttl);

        let array_bytes = [bytes[8], bytes[9], bytes[10], bytes[11]];
        let signature_expiration = u32::from_be_bytes(array_bytes);
        rrsig_rdata.set_signature_expiration(signature_expiration);

        let array_bytes = [bytes[12], bytes[13], bytes[14], bytes[15]];
        let signature_inception = u32::from_be_bytes(array_bytes);
        rrsig_rdata.set_signature_inception(signature_inception);

        let array_bytes = [bytes[16], bytes[17]];
        let key_tag = u16::from_be_bytes(array_bytes);
        rrsig_rdata.set_key_tag(key_tag);

        let mut signer_name: Vec<u8> = Vec::new();
        let mut i = 18;
        while bytes[i] != 0 {
            signer_name.push(bytes[i]);
            i += 1;
        }
        signer_name.push(bytes[i]);


        //create the DomainName
        let mut signer_name = DomainName::from_bytes(&signer_name, _full_msg).unwrap();

        //check if labels is less or equal to the number of labels in the signer name
        let mut signer_name_string = signer_name.0.get_name();
        //if the signer_name in string format is the root, then labels must be 0
        if signer_name_string == "" {
            signer_name_string = ".".to_string();
            signer_name.0.set_name(signer_name_string);
            if labels != 0 {
                panic!("Labels is not zero when signer name is root");
            }
        }
        // if the signer_name is not the root, then labels must be less or equal 
        //than labels of the signer name
        else{
            let number_of_subdomains = signer_name_string.split(".").count() as u8;
            if labels > number_of_subdomains {
                //println!("Labels: {} > number of labels in the signer name : {}", labels, number_of_subdomains);
                return Err("Labels is greater than number of labels in the signer name");
            }
        }
        rrsig_rdata.set_signer_name(signer_name.0);

        
        let mut signature: Vec<u8> = Vec::new();
        i += 1;
        while i < bytes_len{
            signature.push(bytes[i]);
            i += 1;
        }
        let signature = String::from_utf8(signature).unwrap();
        rrsig_rdata.set_signature(signature);

        Ok(rrsig_rdata)
        }
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
            type_covered: Rtype::A,
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
    pub fn get_type_covered(&self) -> Rtype{
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
    pub fn set_type_covered(&mut self, type_covered: Rtype) {
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

impl fmt::Display for RRSIGRdata {
    /// Formats the record data for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {} {} {}", 
        u16::from(self.get_type_covered()), 
        self.get_algorithm(), 
        self.get_labels(), 
        self.get_original_ttl(), 
        self.get_signature_expiration(), 
        self.get_signature_inception(), 
        self.get_key_tag(), 
        self.get_signer_name().get_name(), 
        self.get_signature())
    }
}

#[cfg(test)]
mod rrsig_rdata_test{
    use super::*;

    #[test]
    fn constructor_test(){
        let rrsig_rdata = RRSIGRdata::new();

        assert_eq!(rrsig_rdata.type_covered, Rtype::A);
        assert_eq!(rrsig_rdata.algorithm, 0);
        assert_eq!(rrsig_rdata.labels, 0);
        assert_eq!(rrsig_rdata.original_ttl, 0);
        assert_eq!(rrsig_rdata.signature_expiration, 0);
        assert_eq!(rrsig_rdata.signature_inception, 0);
        assert_eq!(rrsig_rdata.key_tag, 0);
        assert_eq!(rrsig_rdata.signer_name, DomainName::new());
        assert_eq!(rrsig_rdata.signature, String::new());
    }

    #[test]
    fn setters_and_getters_test(){
        let mut rrsig_rdata = RRSIGRdata::new();

        assert_eq!(rrsig_rdata.get_type_covered(), Rtype::A);
        assert_eq!(rrsig_rdata.get_algorithm(), 0);
        assert_eq!(rrsig_rdata.get_labels(), 0);
        assert_eq!(rrsig_rdata.get_original_ttl(), 0);
        assert_eq!(rrsig_rdata.get_signature_expiration(), 0);
        assert_eq!(rrsig_rdata.get_signature_inception(), 0);
        assert_eq!(rrsig_rdata.get_key_tag(), 0);
        assert_eq!(rrsig_rdata.get_signer_name(), DomainName::new());
        assert_eq!(rrsig_rdata.get_signature(), String::new());

        rrsig_rdata.set_type_covered(Rtype::CNAME);
        rrsig_rdata.set_algorithm(5);
        rrsig_rdata.set_labels(2);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1630435200);
        rrsig_rdata.set_signature_inception(1630435200);
        rrsig_rdata.set_key_tag(1234);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
        rrsig_rdata.set_signature(String::from("abcdefg"));

        assert_eq!(rrsig_rdata.get_type_covered(), Rtype::CNAME);
        assert_eq!(rrsig_rdata.get_algorithm(), 5);
        assert_eq!(rrsig_rdata.get_labels(), 2);
        assert_eq!(rrsig_rdata.get_original_ttl(), 3600);
        assert_eq!(rrsig_rdata.get_signature_expiration(), 1630435200);
        assert_eq!(rrsig_rdata.get_signature_inception(), 1630435200);
        assert_eq!(rrsig_rdata.get_key_tag(), 1234);
        assert_eq!(rrsig_rdata.get_signer_name(), DomainName::new_from_str("example.com"));
        assert_eq!(rrsig_rdata.get_signature(), String::from("abcdefg"));
    }

    #[test]
    fn to_bytes(){
        let mut rrsig_rdata = RRSIGRdata::new();
        rrsig_rdata.set_type_covered(Rtype::CNAME);
        rrsig_rdata.set_algorithm(5);
        rrsig_rdata.set_labels(2);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1630435200);
        rrsig_rdata.set_signature_inception(1630435200);
        rrsig_rdata.set_key_tag(1234);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
        rrsig_rdata.set_signature(String::from("abcdefg"));

        let expected_result: Vec<u8> = vec![0, 5, //typed covered
        5, //algorithm
        2, //Labels
        0, 0, 14, 16, //TTL
        97, 46, 119, 128,//signature expiration
        97, 46, 119, 128, //signature inception
        4, 210, //key tag
        7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0, //domain name
        97, 98, 99, 100, 101, 102, 103]; //signature

        let result = rrsig_rdata.to_bytes();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn from_bytes(){
        let bytes_test: Vec<u8> = vec![0, 5, 5, 2, 0, 0, 14, 16, 97, 46, 119, 128, 97,
         46, 119, 128, 4, 210, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0, 97, 
         98, 99, 100, 101, 102, 103];

        let mut rrsig_rdata = RRSIGRdata::new();
        rrsig_rdata.set_type_covered(Rtype::CNAME);
        rrsig_rdata.set_algorithm(5);
        rrsig_rdata.set_labels(2);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1630435200);
        rrsig_rdata.set_signature_inception(1630435200);
        rrsig_rdata.set_key_tag(1234);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
        rrsig_rdata.set_signature(String::from("abcdefg"));

        let result = RRSIGRdata::from_bytes(&bytes_test, &bytes_test).unwrap();

        assert_eq!(result, rrsig_rdata);
    }

    #[test]
    fn from_bytes_error(){
        let bytes_test: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            , 11, 12, 13, 14, 15, 16, 17, 18];

        let result = RRSIGRdata::from_bytes(&bytes_test, &bytes_test);

        assert_eq!(result, Err("Format Error"));
    }

    #[test]
    fn from_bytes_max_values() {
        let bytes_test: Vec<u8> = vec![255, 255, //typed covered
        255, //algorithm
        2, //labels
        255, 255, 255, 255, //TTL
        255, 255, 255, 255, //Signature expiration
        255, 255, 255, 255, //Signature Inception
        255, 255,  // key tag
        7, 101, 120, 97, 109, 112, 108, //domain name
        101, 3, 99, 111, 109, 0, 
        97, 98, 99, 100, 101, 102, 103]; //signature

       let mut rrsig_rdata = RRSIGRdata::new();
       rrsig_rdata.set_type_covered(Rtype::UNKNOWN(65535));
       rrsig_rdata.set_algorithm(255);
       rrsig_rdata.set_labels(2);
       rrsig_rdata.set_original_ttl(4294967295);
       rrsig_rdata.set_signature_expiration(4294967295);
       rrsig_rdata.set_signature_inception(4294967295);
       rrsig_rdata.set_key_tag(65535);
       rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
       rrsig_rdata.set_signature(String::from("abcdefg"));

       if let Ok(result) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
           assert_eq!(result, rrsig_rdata);
       }
       else {
            assert!(false, "error");
       }
    
    }

    #[test]
    fn to_bytes_max_values() {
        let bytes_test: Vec<u8> = vec![255, 255, //typed covered
        255, //algorithm
        2, //labels
        255, 255, 255, 255, //TTL
        255, 255, 255, 255, //Signature expiration
        255, 255, 255, 255, //Signature Inception
        255, 255,  // key tag
        7, 101, 120, 97, 109, 112, 108, //domain name
        101, 3, 99, 111, 109, 0, 
        97, 98, 99, 100, 101, 102, 103]; //signature

       let mut rrsig_rdata = RRSIGRdata::new();
       rrsig_rdata.set_type_covered(Rtype::UNKNOWN(65535));
       rrsig_rdata.set_algorithm(255);
       rrsig_rdata.set_labels(2);
       rrsig_rdata.set_original_ttl(4294967295);
       rrsig_rdata.set_signature_expiration(4294967295);
       rrsig_rdata.set_signature_inception(4294967295);
       rrsig_rdata.set_key_tag(65535);
       rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
       rrsig_rdata.set_signature(String::from("abcdefg"));

       let result = rrsig_rdata.to_bytes();
    
       assert_eq!(result, bytes_test);
    }
    
    #[test]
    fn from_bytes_min_values() {
        let bytes_test: Vec<u8> = vec![0, 0, //typed covered
        0, //algorithm
        0, //labels
        0, 0, 0, 0, //TTL
        0, 0, 0, 0, //Signature expiration
        0, 0, 0, 0, //Signature Inception
        0, 0, // key tag
        0, //empty string in signer name
        0]; //signature

       let mut rrsig_rdata = RRSIGRdata::new();
       rrsig_rdata.set_type_covered(Rtype::UNKNOWN(0));
       rrsig_rdata.set_algorithm(0);
       rrsig_rdata.set_labels(0);
       rrsig_rdata.set_original_ttl(0);
       rrsig_rdata.set_signature_expiration(0);
       rrsig_rdata.set_signature_inception(0);
       rrsig_rdata.set_key_tag(0);
       rrsig_rdata.set_signer_name(DomainName::new_from_str("."));
       rrsig_rdata.set_signature(String::from("\0"));

       if let Ok(result) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
         assert_eq!(result, rrsig_rdata);
       }
       else {
        assert!(false, "error");
       }
    
    }

    #[test]
    fn to_bytes_min_values() {
        let bytes_test: Vec<u8> = vec![0, 0, //typed covered
        0, //algorithm
        0, //labels
        0, 0, 0, 0, //TTL
        0, 0, 0, 0, //Signature expiration
        0, 0, 0, 0, //Signature Inception
        0, 0, // key tag
        0,  //empty string in signer name
        0]; //signautre 

       let mut rrsig_rdata = RRSIGRdata::new();
       rrsig_rdata.set_type_covered(Rtype::UNKNOWN(0));
       rrsig_rdata.set_algorithm(0);
       rrsig_rdata.set_labels(0);
       rrsig_rdata.set_original_ttl(0);
       rrsig_rdata.set_signature_expiration(0);
       rrsig_rdata.set_signature_inception(0);
       rrsig_rdata.set_key_tag(0);
       rrsig_rdata.set_signer_name(DomainName::new_from_str(""));
       rrsig_rdata.set_signature(String::from("\0"));

       let result = rrsig_rdata.to_bytes();

       assert_eq!(result, bytes_test);
    }

    #[test]
    #[should_panic]
    fn from_bytes_wrong_labels_small_signer_name(){
        let bytes_test: Vec<u8> = vec![0, 5, //typed covered
        5, //algorithm
        3, //Labels
        0, 0, 14, 16, //TTL
        97, 46, 119, 128,//signature expiration
        97, 46, 119, 128, //signature inception
        4, 210, //key tag
        7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0, //domain name = example.com
        97, 98, 99, 100, 101, 102, 103]; //signature

        if let Err(error) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
            assert_eq!("{}", error);
        }
        else {
            assert!(false, "Test shoud have been panic bacuase the number of labels is wrong");
        }
    }

    #[test]
    #[should_panic]
    fn from_bytes_wrong_labels_big_signer_name(){
        let bytes_test: Vec<u8> = vec![0, 5, //typed covered
        5, //algorithm
        9, //Labels
        0, 0, 14, 16, //TTL
        97, 46, 119, 128,//signature expiration
        97, 46, 119, 128, //signature inception
        4, 210, //key tag
        3, 119, 119, 119, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 
        2, 101, 115, 2, 109, 120, 2, 97, 114, 2, 117, 115, 2, 117, 107, 0, //domain name = www.example.com.es.mx.ar.us.uk
        97, 98, 99, 100, 101, 102, 103]; //signature

        if let Err(error) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
            panic!("{}", error);
        }
        else {
            assert!(false, "Test shoud have been panic bacuase the number of labels is wrong");
        }
    }

    #[test]
    fn from_bytes_good_labels_big_signer_name(){
        let bytes_test: Vec<u8> = vec![0, 5, //typed covered
        5, //algorithm
        8, //Labels
        0, 0, 14, 16, //TTL
        97, 46, 119, 128,//signature expiration
        97, 46, 119, 128, //signature inception
        4, 210, //key tag
        3, 119, 119, 119, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 
        2, 101, 115, 2, 109, 120, 2, 97, 114, 2, 117, 115, 2, 117, 107, 0, //domain name = www.example.com.es.mx.ar.us.uk
        97, 98, 99, 100, 101, 102, 103]; //signature

        let mut rrsig_rdata = RRSIGRdata::new();
        rrsig_rdata.set_type_covered(Rtype::CNAME);
        rrsig_rdata.set_algorithm(5);
        rrsig_rdata.set_labels(8);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1630435200);
        rrsig_rdata.set_signature_inception(1630435200);
        rrsig_rdata.set_key_tag(1234);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("www.example.com.es.mx.ar.us.uk"));
        rrsig_rdata.set_signature(String::from("abcdefg"));


        if let Ok(rrsig_data_from_bytes) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
            assert_eq!(rrsig_rdata, rrsig_data_from_bytes);
        }
        else {
            assert!(false, "error");
        }
    }

    #[test]
    #[should_panic]
    fn from_bytes_wrong_labels_root_signer_name(){
        let bytes_test: Vec<u8> = vec![0, 5, //typed covered
        5, //algorithm
        1, //Labels
        0, 0, 14, 16, //TTL
        97, 46, 119, 128,//signature expiration
        97, 46, 119, 128, //signature inception
        4, 210, //key tag
        0, // signer name = .
        97, 98, 99, 100, 101, 102, 103]; //signature

        if let Err(error) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
            panic!("{}", error);
        }
        else {
            assert!(false, "Test should have panic bacuase the number of labels is wrong");
        }
    }

    #[test]
    fn from_bytes_good_labels_root_signer_name(){
        let bytes_test: Vec<u8> = vec![0, 5, //typed covered
        5, //algorithm
        0, //Labels
        0, 0, 14, 16, //TTL
        97, 46, 119, 128,//signature expiration
        97, 46, 119, 128, //signature inception
        4, 210, //key tag
        0, //domain name = .
        97, 98, 99, 100, 101, 102, 103]; //signature


        let mut rrsig_rdata = RRSIGRdata::new();
        rrsig_rdata.set_type_covered(Rtype::CNAME);
        rrsig_rdata.set_algorithm(5);
        rrsig_rdata.set_labels(0);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1630435200);
        rrsig_rdata.set_signature_inception(1630435200);
        rrsig_rdata.set_key_tag(1234);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("."));
        rrsig_rdata.set_signature(String::from("abcdefg"));

        if let Ok(rrsig_data_from_bytes) = RRSIGRdata::from_bytes(&bytes_test, &bytes_test) {
            assert_eq!(rrsig_rdata, rrsig_data_from_bytes);
        }
        else {
            assert!(false, "error");
        }
    }


} 
