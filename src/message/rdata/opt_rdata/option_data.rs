use crate::message::rdata::opt_rdata::ede_optdata::EdeOptData;
use crate::message::rdata::opt_rdata::option_code::OptionCode;
use crate::message::resource_record::ToBytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionData {
    NSID(String),
    EDE(EdeOptData),
    /*
    Padding is just a sequence of bytes that MUST BE set to 0
    The figure below specifies the structure of the option in the RDATA
    of the OPT RR:

                0                       8                      16
                +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
                |                  OPTION-CODE                  |
                +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
                |                 OPTION-LENGTH                 |
                +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
                |        (PADDING) ...        (PADDING) ...     /
                +-  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -

                                 Figure 1 */
    Padding(Vec<u8>),
    Unknown(Vec<u8>),
}

impl ToBytes for OptionData {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            OptionData::NSID(s) => {
                s.clone().into_bytes()
            },
            OptionData::EDE(ede) => {
                ede.to_bytes()
            },
            OptionData::Padding(data) => {
                data.clone()
            },
            OptionData::Unknown(data) => {
                data.to_vec()
            }
        }
    }
}

impl OptionData {
    pub fn from_bytes_with_opt_type(bytes: Vec<u8>, opt_t: OptionCode) -> Result<OptionData, &'static str> {
        match opt_t {
            OptionCode::NSID => {
                let nsid = String::from_utf8(bytes).map_err(|_| "Error parsing NSID")?;
                Ok(OptionData::NSID(nsid))
            },
            OptionCode::EDE => {
                let ede = EdeOptData::from_bytes(&bytes).map_err(|_| "Error parsing EDE")?;
                Ok(OptionData::EDE(ede))
            },
            OptionCode::PADDING => {
                Ok(OptionData::Padding(bytes))
            },
            _ => Ok(OptionData::Unknown(bytes))
        }
    }
}

#[cfg(test)]
mod option_data_tests {
    use super::*;
    use crate::message::rdata::opt_rdata::ede_optdata::EdeOptData;
    use crate::message::rdata::opt_rdata::ede_code::EdeCode;
    use crate::message::rdata::opt_rdata::option_code::OptionCode;

    #[test]
    fn test_option_data_nsid() {
        let nsid_string = "testNSID".to_string();
        let option_data = OptionData::NSID(nsid_string.clone());
        let serialized = option_data.to_bytes();
        let rebuilt = OptionData::from_bytes_with_opt_type(serialized.clone(), OptionCode::NSID)
            .expect("NSID reconstruction failed");
        assert_eq!(option_data, rebuilt);
        assert_eq!(serialized.len(), nsid_string.len());
    }

    #[test]
    fn test_option_data_ede() {
        let code = EdeCode::StaleAns;
        let msg = "Stale Answer".to_string();
        let ede_data = EdeOptData::new(code, msg.clone());
        let option_data = OptionData::EDE(ede_data.clone());
        let serialized = option_data.to_bytes();
        let rebuilt = OptionData::from_bytes_with_opt_type(serialized.clone(), OptionCode::EDE)
            .expect("EDE reconstruction failed");
        assert_eq!(option_data, rebuilt);
        if let OptionData::EDE(ede_rebuilt) = rebuilt {
            assert_eq!(ede_rebuilt.get_info_code(), code);
            assert_eq!(ede_rebuilt.get_extra_text(), msg);
        } else {
            panic!("Expected OptionData::EDE variant");
        }
    }

    #[test]
    fn test_option_data_padding() {
        let padding_bytes = vec![0x00, 0x00, 0x00, 0x00];
        let option_data = OptionData::Padding(padding_bytes.clone());
        let serialized = option_data.to_bytes();
        let rebuilt = OptionData::from_bytes_with_opt_type(serialized.clone(), OptionCode::PADDING)
            .expect("Padding reconstruction failed");
        assert_eq!(option_data, rebuilt);
        assert_eq!(serialized.len(), 4);
    }

    #[test]
    fn test_option_data_unknown() {
        let unknown_bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let option_data = OptionData::Unknown(unknown_bytes.clone());
        let serialized = option_data.to_bytes();
        let rebuilt = OptionData::from_bytes_with_opt_type(serialized.clone(), OptionCode::UNKNOWN(999))
            .expect("Unknown reconstruction failed");
        assert_eq!(option_data, rebuilt);
        assert_eq!(serialized.len(), 4);
    }
}
