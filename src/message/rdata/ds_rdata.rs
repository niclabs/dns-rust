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
