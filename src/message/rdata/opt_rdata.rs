use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use crate::message::rdata::Rdata;
use crate::message::Rclass;
use crate::message::Rtype;


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
