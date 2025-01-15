use crate::edns::opt_option::option_code::OptionCode;
use crate::edns::opt_option::option_data::OptionData;
use crate::edns::options::ede::ede_optdata::EdeOptData;
use crate::edns::options::ede::ede_code::EdeCode;
use crate::edns::options::zoneversion::ZoneversionOptData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptOption {
    option_code: OptionCode,
    option_len: u16,
    opt_data: OptionData
}

impl OptOption {
    pub fn new(option_code: OptionCode) -> Self {
        match option_code {
            OptionCode::NSID => {
                OptOption {
                    option_code,
                    option_len: 0,
                    opt_data: OptionData::NSID(String::new())
                }
            },
            OptionCode::EDE => {
                OptOption {
                    option_code,
                    option_len: 0,
                    opt_data: OptionData::EDE(EdeOptData::new())
                }

            },
            OptionCode::PADDING => {
                OptOption {
                    option_code,
                    option_len: 0,
                    opt_data: OptionData::Padding(Vec::new())
                }
            },
            OptionCode::ZONEVERSION => {
                OptOption {
                    option_code,
                    option_len: 0,
                    opt_data: OptionData::ZoneVersion(ZoneversionOptData::new())
                }
            },
            _ => {
                OptOption {
                    option_code,
                    option_len: 0,
                    opt_data: OptionData::Unknown(Vec::new())
                }
            }
        }
    }
    // getters
    pub fn get_option_code(&self) -> OptionCode {
        self.option_code
    }

    pub fn get_option_len(&self) -> u16 {
        self.option_len
    }

    pub fn get_opt_data(&self) -> OptionData {
        self.opt_data.clone()
    }

    // setters
    pub fn set_option_code(&mut self, option_code: OptionCode) {
        self.option_code = option_code;
    }

    pub fn set_option_len(&mut self, option_len: u16) {
        self.option_len = option_len;
    }

    pub fn set_opt_data(&mut self, opt_data: OptionData) {
        self.opt_data = opt_data;
    }
}

pub mod option_data {
    use crate::edns::options::ede::ede_optdata::EdeOptData;
    use crate::edns::opt_option::option_code::OptionCode;
    use crate::edns::options::zoneversion::ZoneversionOptData;
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
        ZoneVersion(ZoneversionOptData),
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
                OptionData::ZoneVersion(zoneversion) => {
                    zoneversion.to_bytes()
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
                OptionCode::ZONEVERSION => {
                    let zoneversion = ZoneversionOptData::from_bytes(&bytes).map_err(|_| "Error parsing EDE")?;
                    Ok(OptionData::ZoneVersion(zoneversion))
                },
                _ => Ok(OptionData::Unknown(bytes))
            }
        }
    }
}

pub mod option_code {
    use std::fmt;
    #[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
    /// Enum for the option code in an OPT Rdata
    pub enum OptionCode {
        NSID,
        PADDING,
        // added for rfc6975
        DAU,
        DHU,
        // added for rf8914
        EDE,
        N3U,
        ZONEVERSION,
        UNKNOWN(u16),
    }

    impl From<OptionCode> for u16 {
        fn from(option_code: OptionCode) -> u16 {
            match option_code {
                OptionCode::NSID => 3,
                OptionCode::DAU => 5,
                OptionCode::DHU => 6,
                OptionCode::N3U => 7,
                OptionCode::PADDING => 12,
                OptionCode::EDE => 15,
                OptionCode::ZONEVERSION => 19,
                OptionCode::UNKNOWN(val) => val,
            }
        }
    }

    impl From<u16> for OptionCode {
        fn from(val: u16) -> OptionCode {
            match val {
                3 => OptionCode::NSID,
                5 => OptionCode::DAU,
                6 => OptionCode::DHU,
                7 => OptionCode::N3U,
                12 => OptionCode::PADDING,
                15 => OptionCode::EDE,
                19 => OptionCode::ZONEVERSION,
                _ => OptionCode::UNKNOWN(val),
            }
        }
    }

    impl From<&str> for OptionCode {
        fn from(val: &str) -> OptionCode {
            match val {
                "NSID" => OptionCode::NSID,
                "DAU" => OptionCode::DAU,
                "DHU" => OptionCode::DHU,
                "N3U" => OptionCode::N3U,
                "EDE" => OptionCode::EDE,
                "PADDING" => OptionCode::PADDING,
                "ZONEVERSION" => OptionCode::ZONEVERSION,
                _ => OptionCode::UNKNOWN(0),
            }
        }
    }

    impl Default for OptionCode {
        fn default() -> Self {
            OptionCode::UNKNOWN(0)
        }
    }

    impl fmt::Display for OptionCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match *self {
                OptionCode::NSID => "NSID",
                OptionCode::DAU => "DAU",
                OptionCode::DHU => "DHU",
                OptionCode::N3U => "N3U",
                OptionCode::EDE => "EDE",
                OptionCode::PADDING => "PADDING",
                OptionCode::ZONEVERSION => "ZONEVERSION",
                OptionCode::UNKNOWN(_) => "UNKNOWN",
            })
        }
    }
}

#[cfg(test)]
mod option_data_tests {
    use crate::edns::opt_option::option_data::OptionData;
    use crate::edns::options::ede::ede_optdata::EdeOptData;
    use crate::edns::options::ede::ede_code::EdeCode;
    use crate::edns::opt_option::option_code::OptionCode;
    use crate::edns::opt_option::OptOption;
    use crate::message::resource_record::ToBytes;
    use crate::edns::options::zoneversion::{OpaqueString, ZoneversionOptData};
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
    fn test_option_data_zoneversion() {
        let label_count:u8 = 2;
        let type_:u8 = 0;
        let version:OpaqueString = OpaqueString::from_bytes(&[0x12, 0x34, 0x56]).unwrap();
        let zoneversion = ZoneversionOptData::new_from(label_count, type_, version);
        let option_data = OptionData::ZoneVersion(zoneversion);
        let serialized = option_data.to_bytes();
        let rebuilt = OptionData::from_bytes_with_opt_type(serialized.clone(), OptionCode::ZONEVERSION)
            .expect("ZONEVERSION reconstruction failed");
        assert_eq!(option_data, rebuilt);
        assert_eq!(serialized.len(), 5);

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

    #[test]
    fn test_set_get_option_code() {
        let mut opt = OptOption::new(OptionCode::NSID);
        assert_eq!(opt.get_option_code(), OptionCode::NSID);

        opt.set_option_code(OptionCode::EDE);
        assert_eq!(opt.get_option_code(), OptionCode::EDE);
    }

    #[test]
    fn test_set_get_option_len() {
        let mut opt = OptOption::new(OptionCode::NSID);
        assert_eq!(opt.get_option_len(), 0);

        opt.set_option_len(8);
        assert_eq!(opt.get_option_len(), 8);
    }

    #[test]
    fn test_set_get_opt_data() {
        let mut opt = OptOption::new(OptionCode::NSID);
        assert_eq!(opt.get_opt_data(), OptionData::NSID(String::new()));

        opt.set_opt_data(OptionData::NSID("example".to_string()));
        assert_eq!(opt.get_opt_data(), OptionData::NSID("example".to_string()));
    }
}