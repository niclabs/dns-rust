use crate::edns::opt_option::option_code::OptionCode;
use crate::edns::opt_option::option_data::OptionData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptOption {
    option_code: OptionCode,
    option_len: u16,
    opt_data: OptionData
}

impl OptOption {
    pub fn new(option_code: OptionCode, option_len: u16, opt_data: OptionData) -> Self {
        OptOption {
            option_code, option_len, opt_data,
        }
    }

    pub fn get_option_code(&self) -> OptionCode {
        self.option_code
    }

    pub fn get_option_len(&self) -> u16 {
        self.option_len
    }

    pub fn get_opt_data(&self) -> OptionData {
        self.opt_data.clone()
    }
}

pub mod option_data {
    use crate::edns::options::ede::ede_optdata::EdeOptData;
    use crate::edns::opt_option::option_code::OptionCode;
    use crate::message::resource_record::ToBytes;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum OptionData {
        NSID(String),
        EDE(EdeOptData),
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
                OptionData::Unknown(data) => {
                    data.to_vec()
                }
            }
        }
    }

    impl OptionData {
        pub fn from_with_opt_type(bytes: Vec<u8>, opt_t: OptionCode) -> Result<OptionData, &'static str> {
            match opt_t {
                OptionCode::NSID => {
                    let nsid = String::from_utf8(bytes).map_err(|_| "Error parsing NSID")?;
                    Ok(OptionData::NSID(nsid))
                },
                OptionCode::EDE => {
                    let ede = EdeOptData::from_bytes(&bytes).map_err(|_| "Error parsing EDE")?;
                    Ok(OptionData::EDE(ede))
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
        UNKNOWN(u16),
        // added for rfc6975
        DAU,
        DHU,
        // added for rf8914
        EDE,
        N3U
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
                OptionCode::UNKNOWN(_) => "UNKNOWN",
            })
        }
    }
}