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
    pub key_tag: u16,
    pub algorithm: u8,
    pub digest_type: u8,
    pub digest: Vec<u8>,
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
        bytes.push(self.digest.len() as u8);
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
}
