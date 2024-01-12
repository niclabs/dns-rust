use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Clone, Debug, PartialEq)]
/// Struct for DNSKEY Rdata
///                       1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |              Flags            |    Protocol   |   Algorithm   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                                                               /
/// /                            Public Key                         /
/// /                                                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// 
/// [RFC 4034](https://tools.ietf.org/html/rfc4034#section-2.1.)

pub struct DnskeyRdata {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: u8,
    pub public_key: Vec<u8>,
}

impl ToBytes for DnskeyRdata {
    /// Returns a `Vec<u8>` of bytes that represents the DNSKEY RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.push(self.protocol);
        bytes.push(self.algorithm);
        bytes.extend_from_slice(&self.public_key);

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for DnskeyRdata {
    /// Creates a new `DnskeyRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len <= 3 {
            return Err("Format Error");
        }
        if bytes_len == 4 {
            return Err("Public key not assigned");
        }

        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());

        let array_bytes = [bytes[0], bytes[1]];
        let flags = u16::from_be_bytes(array_bytes);
        dnskey_rdata.set_flags(flags);

        let protocol = bytes[2];
        dnskey_rdata.set_protocol(protocol);

        let algorithm = bytes[3];
        dnskey_rdata.set_algorithm(algorithm);

        let mut public_key: Vec<u8> = Vec::new();
        for i in 4..bytes_len {
            public_key.push(bytes[i]);
        }
        dnskey_rdata.set_public_key(public_key);

        Ok(dnskey_rdata)
    }
}

/// Constructor for DnskeyRdata and getter's for the fields
impl DnskeyRdata {
    /// Constructs a new `DnskeyRdata` with default values.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// ```
    pub fn new(flags: u16, protocol: u8, algorithm: u8, public_key: Vec<u8>) -> DnskeyRdata {
        DnskeyRdata {
            flags,
            protocol,
            algorithm,
            public_key,
        }
    }

    /// Get the flags of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// assert_eq!(dnskey_rdata.get_flags(), 0);
    /// ```
    pub fn get_flags(&self) -> u16 {
        self.flags.clone()
    }

    /// Get the protocol of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// assert_eq!(dnskey_rdata.get_protocol(), 0);
    /// ```
    pub fn get_protocol(&self) -> u8 {
        self.protocol.clone()
    }

    /// Get the algorithm of the DNSKEY RDATA. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// assert_eq!(dnskey_rdata.get_algorithm(), 0);
    /// ```
    pub fn get_algorithm(&self) -> u8 {
        self.algorithm.clone()
    }

    /// Get the public key of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// assert_eq!(dnskey_rdata.get_public_key(), Vec::new());
    /// ```
    pub fn get_public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }
}

/// Setters for DnskeyRdata
impl DnskeyRdata {
    /// Set the flags of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// dnskey_rdata.set_flags(1);
    /// assert_eq!(dnskey_rdata.get_flags(), 1);
    /// ```
    pub fn set_flags(&mut self, flags: u16) {
        self.flags = flags;
    }

    /// Set the protocol of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// dnskey_rdata.set_protocol(1);
    /// assert_eq!(dnskey_rdata.get_protocol(), 1);
    /// ```
    pub fn set_protocol(&mut self, protocol: u8) {
        self.protocol = protocol;
    }

    /// Set the algorithm of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// dnskey_rdata.set_algorithm(1);
    /// assert_eq!(dnskey_rdata.get_algorithm(), 1);
    /// ```
    pub fn set_algorithm(&mut self, algorithm: u8) {
        self.algorithm = algorithm;
    }

    /// Set the public key of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
    /// dnskey_rdata.set_public_key(vec![0x01, 0x02]);
    /// assert_eq!(dnskey_rdata.get_public_key(), vec![0x01, 0x02]);
    /// ```
    pub fn set_public_key(&mut self, public_key: Vec<u8>) {
        self.public_key = public_key;
    }
}

#[cfg(test)]
mod dnskey_rdata_test{
    use std::vec;

    use super::*;

    #[test]
    fn setters_and_getters_test(){
        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
        dnskey_rdata.set_flags(1);
        dnskey_rdata.set_protocol(2);
        dnskey_rdata.set_algorithm(3);
        dnskey_rdata.set_public_key(vec![1, 2]);

        assert_eq!(dnskey_rdata.get_flags(), 1);
        assert_eq!(dnskey_rdata.get_protocol(), 2);
        assert_eq!(dnskey_rdata.get_algorithm(), 3);
        assert_eq!(dnskey_rdata.get_public_key(), vec![1, 2]);
    }

    #[test]
    fn to_bytes(){
        let dnskey_rdata = DnskeyRdata::new(1, 2, 3, vec![1,2]);

        let bytes_test: Vec<u8> = vec![0, 1, 2, 3, 1, 2];

        assert_eq!(dnskey_rdata.to_bytes(), bytes_test);
    }

    #[test]
    fn from_bytes(){
        let dnskey_rdata = DnskeyRdata::new(1, 2, 3, vec![1,2]);

        let bytes_test: Vec<u8> = vec![0, 1, 2, 3, 1, 2];

        let result = DnskeyRdata::from_bytes(&bytes_test, &bytes_test).unwrap();

        assert_eq!(dnskey_rdata, result);
    }

    #[test]
    fn from_bytes_error(){
        let bytes_test: Vec<u8> = vec![0, 1, 2];

        let result = DnskeyRdata::from_bytes(&bytes_test, &bytes_test);

        assert_eq!(Err("Format Error"), result);
    }

    #[test]
    fn max_values_from_bytes(){
        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
        //Max value of 2 bytes is 65535 (16 ones in the 2 bytes)
        dnskey_rdata.set_flags(65535);
        dnskey_rdata.set_protocol(255);
        dnskey_rdata.set_algorithm(255);
        dnskey_rdata.set_public_key(vec![255, 255]);

        let bytes_test: Vec<u8> = vec![255, 255, 255, 255, 255, 255];

        if let Ok(result)= DnskeyRdata::from_bytes(&bytes_test, &bytes_test) {
            assert_eq!(dnskey_rdata, result);
        } 
        else {
            assert!(false, "Error");
        }
    }

    #[test]
    fn max_values_to_bytes(){
        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
        //Max value of 2 bytes is 65535 (16 ones in the 2 bytes)
        dnskey_rdata.set_flags(65535);
        dnskey_rdata.set_protocol(255);
        dnskey_rdata.set_algorithm(255);
        dnskey_rdata.set_public_key(vec![255, 255]);

        let bytes_test: Vec<u8> = vec![255, 255, 255, 255, 255, 255];

        assert_eq!(dnskey_rdata.to_bytes(), bytes_test);
    }

    #[test]
    fn min_values_from_bytes(){
        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
        dnskey_rdata.set_flags(0);
        dnskey_rdata.set_protocol(0);
        dnskey_rdata.set_algorithm(0);
        dnskey_rdata.set_public_key(vec![0, 0]);

        let bytes_test: Vec<u8> = vec![0, 0, 0, 0, 0, 0];

        if let Ok(result)= DnskeyRdata::from_bytes(&bytes_test, &bytes_test) {
            assert_eq!(dnskey_rdata, result);
        } 
        else {
            assert!(false, "Error");
        }
    }

    #[test]
    fn min_values_to_bytes(){
        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
        dnskey_rdata.set_flags(0);
        dnskey_rdata.set_protocol(0);
        dnskey_rdata.set_algorithm(0);
        dnskey_rdata.set_public_key(vec![0, 0]);

        let bytes_test: Vec<u8> = vec![0, 0, 0, 0, 0, 0];

        assert_eq!(dnskey_rdata.to_bytes(), bytes_test);
    }

    #[test]
    fn missing_public_key_from_bytes() {
        //Bytes array missing the public key
        let bytes_test: Vec<u8> = vec![0, 1, 3, 5]; 

        let result = DnskeyRdata::from_bytes(&bytes_test, &bytes_test);

        assert_eq!(Err("Public key not assigned"), result);

    }
}