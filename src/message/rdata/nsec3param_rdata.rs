use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Clone, PartialEq, Debug)]
/// Struct for the NSEC3 Rdata
/// [RFC 5155](https://tools.ietf.org/html/rfc5155#section-4.2)
/// ```text
/// 4.2.  The NSEC3PARAM Wire Format
/// The RDATA of the NSEC3PARAM RR is as shown below:
///
/// 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |   Hash Alg.   |     Flags     |          Iterations           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |  Salt Length  |                     Salt                      /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```

pub struct Nsec3ParamRdata {
    hash_algorithm: u8,
    flags: u8,
    iterations: u16,
    salt_length: u8,
    salt: String,
}

impl ToBytes for Nsec3ParamRdata {
    /// Convert the NSEC3 Rdata to bytes
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let hash_algorithm: u8 = self.get_hash_algorithm();
        bytes.push(hash_algorithm);
        let flags: u8 = self.get_flags();
        bytes.push(flags);
        let iterations: u16 = self.get_iterations();
        bytes.extend_from_slice(&iterations.to_be_bytes());
        let salt_length: u8 = self.get_salt_length();
        bytes.push(salt_length);
        let salt = self.get_salt();
        bytes.extend_from_slice(salt.as_bytes());

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for Nsec3ParamRdata {
    /// Create a new `Nsec3ParamRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let hash_algorithm = bytes[0];
        let flags = bytes[1];
        let array_bytes = [bytes[2], bytes[3]];
        let iterations = u16::from_be_bytes(array_bytes);
        let salt_length = bytes[4];
        let salt: String = String::from_utf8_lossy(&bytes[5..(5 + salt_length as usize)]).to_string();

        let nsec3_rdata = Nsec3ParamRdata::new(
            hash_algorithm,
            flags,
            iterations,
            salt_length,
            salt,
        );

        Ok(nsec3_rdata)
    }
}

impl Nsec3ParamRdata {
    /// Create a new NSEC3 Rdata
    pub fn new(
        hash_algorithm: u8,
        flags: u8,
        iterations: u16,
        salt_length: u8,
        salt: String,
    ) -> Nsec3ParamRdata {
        Nsec3ParamRdata {
            hash_algorithm,
            flags,
            iterations,
            salt_length,
            salt
        }
    }

    /// Getter for the hash_algorithm
    pub fn get_hash_algorithm(&self) -> u8 {
        self.hash_algorithm.clone()
    }

    /// Getter for the flags
    pub fn get_flags(&self) -> u8 {
        self.flags.clone()
    }

    /// Getter for the iterations
    pub fn get_iterations(&self) -> u16 {
        self.iterations.clone()
    }

    /// Getter for the salt_length
    pub fn get_salt_length(&self) -> u8 {
        self.salt_length.clone()
    }

    /// Getter for the salt
    pub fn get_salt(&self) -> String {
        self.salt.clone()
    }
}

impl Nsec3ParamRdata {
    /// Setters for the NSEC3 Rdata
    
    /// Setter for the hash_algorithm
    pub fn set_hash_algorithm(&mut self, hash_algorithm: u8) {
        self.hash_algorithm = hash_algorithm;
    }

    /// Setter for the flags
    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    /// Setter for the iterations
    pub fn set_iterations(&mut self, iterations: u16) {
        self.iterations = iterations;
    }

    /// Setter for the salt_length
    pub fn set_salt_length(&mut self, salt_length: u8) {
        self.salt_length = salt_length;
    }

    /// Setter for the salt
    pub fn set_salt(&mut self, salt: String) {
        self.salt = salt;
    }
}


#[cfg(test)]
mod nsec3_rdata_tests {
    use super::*;

    #[test]
    fn constructor(){
        let nsec3_rdata = Nsec3ParamRdata::new(1, 2, 3, 4, "salt".to_string());
        assert_eq!(nsec3_rdata.hash_algorithm, 1);
        assert_eq!(nsec3_rdata.flags, 2);
        assert_eq!(nsec3_rdata.iterations, 3);
        assert_eq!(nsec3_rdata.salt_length, 4);
        assert_eq!(nsec3_rdata.salt, "salt".to_string());
    }

    #[test]
    fn getters(){
        let nsec3_rdata = Nsec3ParamRdata::new(1, 2, 3, 4, "salt".to_string());
        assert_eq!(nsec3_rdata.get_hash_algorithm(), 1);
        assert_eq!(nsec3_rdata.get_flags(), 2);
        assert_eq!(nsec3_rdata.get_iterations(), 3);
        assert_eq!(nsec3_rdata.get_salt_length(), 4);
        assert_eq!(nsec3_rdata.get_salt(), "salt".to_string());
    }

    #[test]
    fn setters(){
        let mut nsec3_rdata = Nsec3ParamRdata::new(1, 2, 3, 4, "salt".to_string());
        nsec3_rdata.set_hash_algorithm(10);
        nsec3_rdata.set_flags(20);
        nsec3_rdata.set_iterations(30);
        nsec3_rdata.set_salt_length(40);
        nsec3_rdata.set_salt("new_salt".to_string());

        assert_eq!(nsec3_rdata.hash_algorithm, 10);
        assert_eq!(nsec3_rdata.flags, 20);
        assert_eq!(nsec3_rdata.iterations, 30);
        assert_eq!(nsec3_rdata.salt_length, 40);
        assert_eq!(nsec3_rdata.salt, "new_salt".to_string());
    }

    #[test]
    fn to_bytes(){
        let nsec3_rdata = Nsec3ParamRdata::new(1, 2, 3, 
            4, "salt".to_string());
        
        let bytes = nsec3_rdata.to_bytes();

        let expected_bytes = vec![1, 2, 0, 3, 4, 115, 97, 108, 116];

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn from_bytes(){
        let bytes = vec![1, 2, 0, 3, 4, 115, 97, 108, 116];

        let expected_nsec3_rdata = Nsec3ParamRdata::new(1, 2, 3, 
            4, "salt".to_string());
        
        let nsec3_rdata = Nsec3ParamRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(nsec3_rdata.hash_algorithm, expected_nsec3_rdata.hash_algorithm);
    }
}