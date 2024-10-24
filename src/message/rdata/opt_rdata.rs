pub mod option_code;

use crate::message::resource_record::{FromBytes, ToBytes};
use crate::message::rdata::opt_rdata::option_code::OptionCode;
use std::fmt;


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
    pub option: Vec<(OptionCode, u16, Vec<u8>)> // (OPTION-CODE, OPTION-LENGTH, OPTION-DATA)
}

impl ToBytes for OptRdata {
    /// Returns a `Vec<u8>` of bytes that represents the OPT RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for (option_code, option_length, option_data) in &self.option {
            bytes.extend(u16::from(*option_code).to_be_bytes());
            bytes.extend(&option_length.to_be_bytes());
            bytes.extend(option_data);
        }

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for OptRdata {
    /// Creates a new `OptRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        let mut opt_rdata = OptRdata::new();

        let mut i = 0;
        
        while i < bytes_len {

            if i + 4 > bytes_len {
                return Err("Format Error");
            }


            let option_code = OptionCode::from(u16::from_be_bytes([bytes[i], bytes[i + 1]]));
            let option_length = u16::from_be_bytes([bytes[i + 2], bytes[i + 3]]);

            i += 4;

            if i + option_length as usize > bytes_len {
                return Err("Format Error");
            }

            let option_data = bytes[i..i + option_length as usize].to_vec();

            i += option_length as usize;

            opt_rdata.option.push((option_code, option_length, option_data));
        }

        Ok(opt_rdata)
    }
}

/// Constructor and getters for OptRdata
impl OptRdata {
    pub fn new() -> Self {
        OptRdata {
            option: Vec::new(),
        }
    }

    pub fn get_option(&self) -> Vec<(OptionCode, u16, Vec<u8>)> {
        self.option.clone()
    }
}

/// Setters for OptRdata
impl OptRdata {
    pub fn set_option(&mut self, option: Vec<(OptionCode, u16, Vec<u8>)>) {
        self.option= option;
    }
}


impl fmt::Display for OptRdata {
    /// Formats the record data for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        
        if !self.option.is_empty() {
            for (option_code, option_length, option_data) in &self.option {
                result.push_str(&format!("OPTION-CODE: {}\n", option_code));
                result.push_str(&format!("OPTION-LENGTH: {}\n", option_length));
                result.push_str(&format!("OPTION-DATA: {:?} (\"{}\")\n", option_data, std::str::from_utf8(option_data).unwrap()));
            }
        }
        else {
            result.push_str("No Option");
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod opt_rdata_test{
    use super::*;

    #[test]
    fn test_opt_rdata_to_bytes() {
        let mut opt_rdata = OptRdata::new();

        opt_rdata.option.push((OptionCode::from(1), 2 as u16, vec![0x06, 0x04]));

        let result = opt_rdata.to_bytes();

        let expected_result: Vec<u8> = vec![0x00, 0x01, 0x00, 0x02, 0x06, 0x04];

        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_opt_rdata_from_bytes() {
        let mut opt_rdata = OptRdata::new();

        opt_rdata.option.push((OptionCode::from(1), 2 as u16, vec![0x06, 0x04]));

        let bytes: Vec<u8> = vec![0x00, 0x01, 0x00, 0x02, 0x06, 0x04];

        let result = OptRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(opt_rdata, result);
    }

    #[test]
    fn test_opt_rdata_from_bytes_error() {
        let bytes: Vec<u8> = vec![0x00, 0x01, 0x00, 0x02, 0x06];

        let result = OptRdata::from_bytes(&bytes, &bytes);

        assert_eq!(Err("Format Error"), result);
    }

    /// Setters and getters tests
    #[test]
    fn test_opt_rdata_setters_and_getters() {
        let mut opt_rdata = OptRdata::new();
        
        let option: Vec<(OptionCode, u16, Vec<u8>)> = vec![(OptionCode::from(1), 2 as u16, vec![0x06, 0x04])];

        opt_rdata.set_option(option.clone());

        assert_eq!(opt_rdata.get_option(), option);
        opt_rdata.set_option(option.clone());

        assert_eq!(opt_rdata.get_option(), option);
    }
}