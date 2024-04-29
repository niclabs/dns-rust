use core::fmt;

use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Clone, PartialEq, Debug)]
/// Struct for the DS Rdata
/// [RFC 4034](https://tools.ietf.org/html/rfc4034#section-5.1)
///   0                   1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
///   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |           Key Tag             |  Algorithm    |  Digest Type  |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  /                                                               /
///  /                            Digest                             /
///  /                                                               /
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


pub struct DsRdata {
    pub key_tag: u16, //the key tag of the DNSKEY RR referred
    pub algorithm: u8, //the algorithm number of the DNSKEY RR referred to by the DS record.
    pub digest_type: u8, //the algorithm to construct the digest
    pub digest: Vec<u8>, //digest = digest_algorithm( DNSKEY owner name | DNSKEY RDATA);
}

impl ToBytes for DsRdata{
    /// Function to convert a DsRdata struct to bytes
    /// # Arguments
    /// * `&self` - The DsRdata
    /// # Return
    /// * `Vec<u8>` - The DsRdata as bytes
    /// # Panics
    /// If the digest is longer than 255 bytes
    /// # Examples
    /// ```
    /// let ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
    /// let ds_rdata_bytes = ds_rdata.to_bytes();
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.key_tag.to_be_bytes());
        bytes.push(self.algorithm);
        bytes.push(self.digest_type);
        if self.digest.len() > 255 {
            panic!("Digest is longer than 255 bytes");
        }
        bytes.extend_from_slice(&self.digest);
        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for DsRdata{
    /// Function to convert bytes to a DsRdata struct
    /// # Arguments
    /// * `bytes` - The bytes to convert
    /// * `full_msg` - The full message
    /// # Return
    /// * `Result<DsRdata, &'static str>` - The result with the DsRdata (or error)
    /// # Examples
    /// ```
    /// let ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
    /// let ds_rdata_bytes = ds_rdata.to_bytes();
    /// let ds_rdata = DsRdata::from_bytes(&ds_rdata_bytes, &ds_rdata_bytes).unwrap();
    /// ```
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 4 {
            return Err("Format error");
        }
        let key_tag = u16::from_be_bytes([bytes[0], bytes[1]]);
        let algorithm = bytes[2];
        let digest_type = bytes[3];
        let digest = bytes[4..].to_vec();
        let digest_len = digest.len();
        if digest_len > 255 {
            return Err("Format error");
        }
        Ok(DsRdata {
            key_tag,
            algorithm,
            digest_type,
            digest,
        })
    }
}

impl DsRdata {
    /// Constructor
    /// # Arguments
    /// * `key_tag` - u16
    /// * `algorithm` - u8
    /// * `digest_type` - u8
    /// * `digest` - Vec<u8>
    /// # Return
    /// * `DsRdata` - DsRdata
    /// # Examples
    /// ```
    /// let ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
    /// ```
    pub fn new(key_tag: u16, algorithm: u8, digest_type: u8, digest: Vec<u8>) -> DsRdata {
        DsRdata {
            key_tag,
            algorithm,
            digest_type,
            digest,
        }
    }

    /// Getter for the key_tag field
    pub fn get_key_tag(&self) -> u16 {
        self.key_tag.clone()
    }

    /// Getter for the algorithm field
    pub fn get_algorithm(&self) -> u8 {
        self.algorithm.clone()
    }

    /// Getter for the digest_type field
    pub fn get_digest_type(&self) -> u8 {
        self.digest_type.clone()
    }

    /// Getter for the digest field
    pub fn get_digest(&self) -> Vec<u8> {
        self.digest.clone()
    }
}

impl DsRdata{
    /// Setter for the key_tag field
    pub fn set_key_tag(&mut self, key_tag: u16) {
        self.key_tag = key_tag;
    }

    /// Setter for the algorithm field
    pub fn set_algorithm(&mut self, algorithm: u8) {
        self.algorithm = algorithm;
    }

    /// Setter for the digest_type field
    pub fn set_digest_type(&mut self, digest_type: u8) {
        self.digest_type = digest_type;
    }

    /// Setter for the digest field
    pub fn set_digest(&mut self, digest: Vec<u8>) {
        self.digest = digest;
    }
}

impl fmt::Display for DsRdata {
    /// Function to format a DsRdata struct
    /// # Arguments
    /// * `&self` - The DsRdata
    /// # Return
    /// * `String` - The formatted DsRdata
    /// # Examples
    /// ```
    /// let ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
    /// println!("{}", ds_rdata);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.key_tag, self.algorithm, self.digest_type, self.digest.iter().map(|b| format!("{:02x}", b)).collect::<String>())
    }
}

#[cfg(test)]
mod ds_rdata_test{
    use super::*;

    #[test]
    fn get_and_set_key_tag(){
        let mut ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
        ds_rdata.set_key_tag(1);
        assert_eq!(ds_rdata.get_key_tag(), 1);
    }

    #[test]
    fn get_and_set_algorithm(){
        let mut ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
        ds_rdata.set_algorithm(1);
        assert_eq!(ds_rdata.get_algorithm(), 1);
    }

    #[test]
    fn get_and_set_digest_type(){
        let mut ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
        ds_rdata.set_digest_type(1);
        assert_eq!(ds_rdata.get_digest_type(), 1);
    }

    #[test]
    fn get_and_set_digest(){
        let mut ds_rdata = DsRdata::new(0, 0, 0, vec![0]);
        ds_rdata.set_digest(vec![1, 2, 3]);
        assert_eq!(ds_rdata.get_digest(), vec![1, 2, 3]);
    }


    #[test]
    fn to_bytes_test(){
        let ds_rdata = DsRdata::new(0, 0, 0, vec![1, 2, 3]);
        let ds_rdata_bytes = ds_rdata.to_bytes();
        assert_eq!(ds_rdata_bytes, vec![0, 0, 0, 0, 1, 2, 3]);
    }

    #[test]
    fn from_bytes_test(){
        let ds_rdata_bytes = vec![0, 0, 0, 0, 1, 2, 3];
        let ds_rdata = DsRdata::from_bytes(&ds_rdata_bytes, &ds_rdata_bytes).unwrap();
        assert_eq!(ds_rdata.get_key_tag(), 0);
        assert_eq!(ds_rdata.get_algorithm(), 0);
        assert_eq!(ds_rdata.get_digest_type(), 0);
        assert_eq!(ds_rdata.get_digest(), vec![1, 2, 3]);
    }

    #[test]
    fn from_bytes_error_test(){
        let ds_rdata_bytes = vec![1, 2, 3];
        let ds_rdata = DsRdata::from_bytes(&ds_rdata_bytes, &ds_rdata_bytes);
        assert_eq!(ds_rdata, Err("Format error"));
    }

    #[test]
    #[should_panic]
    fn to_bytes_error_test(){
        let ds_rdata = DsRdata::new(0, 0, 0, (0..=255).collect());
        let _ds_rdata_bytes = ds_rdata.to_bytes();
    }

    #[test]
    fn to_bytes_and_back(){
        let ds_rdata = DsRdata::new(0, 0, 0, vec![1, 2, 3]);
        let ds_rdata_bytes = ds_rdata.to_bytes();
        let ds_rdata = DsRdata::from_bytes(&ds_rdata_bytes, &ds_rdata_bytes).unwrap();
        assert_eq!(ds_rdata.get_key_tag(), 0);
        assert_eq!(ds_rdata.get_algorithm(), 0);
        assert_eq!(ds_rdata.get_digest_type(), 0);
        assert_eq!(ds_rdata.get_digest(), vec![1, 2, 3]);
    }

    #[test]
    fn to_bytes_min_values(){
        let ds_rdata = DsRdata::new(0, 0, 0, Vec::new());
        let ds_rdata_bytes = ds_rdata.to_bytes();
        assert_eq!(ds_rdata_bytes, vec![0, 0, 0, 0]);
    }

    #[test]
    fn from_bytes_min_values(){
        let ds_rdata_bytes = vec![0, 0, 0, 0];
        let ds_rdata = DsRdata::from_bytes(&ds_rdata_bytes, &ds_rdata_bytes).unwrap();
        assert_eq!(ds_rdata.get_key_tag(), 0);
        assert_eq!(ds_rdata.get_algorithm(), 0);
        assert_eq!(ds_rdata.get_digest_type(), 0);
        assert_eq!(ds_rdata.get_digest(), Vec::new());
    }

    #[test]
    fn to_bytes_max_values(){
        let ds_rdata = DsRdata::new(65535, 255, 255, vec![255]);
        let ds_rdata_bytes = ds_rdata.to_bytes();
        assert_eq!(ds_rdata_bytes, vec![255, 255, 255, 255, 255]);
    }

    #[test]
    fn from_bytes_max_values(){
        let ds_rdata_bytes = vec![255, 255, 255, 255, 255];
        let ds_rdata = DsRdata::from_bytes(&ds_rdata_bytes, &ds_rdata_bytes).unwrap();
        assert_eq!(ds_rdata.get_key_tag(), 65535);
        assert_eq!(ds_rdata.get_algorithm(), 255);
        assert_eq!(ds_rdata.get_digest_type(), 255);
        assert_eq!(ds_rdata.get_digest(), vec![255]);
    }
}
