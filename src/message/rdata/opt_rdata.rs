use crate::message::resource_record::{FromBytes, ToBytes};


#[derive(Clone, Debug, PartialEq)]
/// Struct for OPT Rdata
/// [RFC 6891](https://tools.ietf.org/html/rfc6891#section-6.1.2)
/// +0 (MSB)                            +1 (LSB)
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
/// 0: |                          OPTION-CODE                          |
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
/// 2: |                         OPTION-LENGTH                         |
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
/// 4: |                                                               |
/// /                          OPTION-DATA                          /
/// /                                                               /
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

pub struct OptRdata {
    pub option_code: u16,
    pub option_length: u16,
    pub option_data: Vec<u8>,
}

impl ToBytes for OptRdata {
    /// Returns a `Vec<u8>` of bytes that represents the OPT RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.option_code.to_be_bytes());
        bytes.extend_from_slice(&self.option_length.to_be_bytes());
        bytes.extend_from_slice(&self.option_data);

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for OptRdata {
    /// Creates a new `OptRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 4 {
            return Err("Format Error");
        }

        let mut opt_rdata = OptRdata::new();

        let array_bytes = [bytes[0], bytes[1]];
        let option_code = u16::from_be_bytes(array_bytes);
        opt_rdata.set_option_code(option_code);

        let array_bytes = [bytes[2], bytes[3]];
        let option_length = u16::from_be_bytes(array_bytes);
        opt_rdata.set_option_length(option_length);

        let mut option_data: Vec<u8> = Vec::new();
        for i in 4..bytes_len {
            option_data.push(bytes[i]);
        }
        opt_rdata.set_option_data(option_data);

        Ok(opt_rdata)
    }
}

/// Constructor and getters for OptRdata
impl OptRdata {
    pub fn new() -> Self {
        OptRdata {
            option_code: 0,
            option_length: 0,
            option_data: Vec::new(),
        }
    }

    pub fn get_option_code(&self) -> u16 {
        self.option_code.clone()
    }

    pub fn get_option_length(&self) -> u16 {
        self.option_length.clone()
    }

    pub fn get_option_data(&self) -> Vec<u8> {
        self.option_data.clone()
    }
}

/// Setters for OptRdata
impl OptRdata {
    pub fn set_option_code(&mut self, option_code: u16) {
        self.option_code = option_code;
    }

    pub fn set_option_length(&mut self, option_length: u16) {
        self.option_length = option_length;
    }

    pub fn set_option_data(&mut self, option_data: Vec<u8>) {
        self.option_data = option_data;
    }
}


#[cfg(test)]
mod opt_rdata_test{
    use super::*;

    #[test]
    fn test_opt_rdata_to_bytes() {
        let mut opt_rdata = OptRdata::new();
        opt_rdata.set_option_code(1 as u16);
        opt_rdata.set_option_length(2 as u16);
        opt_rdata.set_option_data(vec![0x06, 0x04]);

        let expected_result: Vec<u8> = vec![0x00, 0x01, 0x00, 0x02, 0x06, 0x04];
        let result = opt_rdata.to_bytes();

        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_opt_rdata_from_bytes() {
        let mut opt_rdata = OptRdata::new();
        opt_rdata.set_option_code(1 as u16);
        opt_rdata.set_option_length(2 as u16);
        opt_rdata.set_option_data(vec![0x06, 0x04]);

        let bytes: Vec<u8> = vec![0x00, 0x01, 0x00, 0x02, 0x06, 0x04];

        let result = OptRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(opt_rdata, result);
    }
}