use crate::message::rdata::opt_rdata::option_code::OptionCode;
use crate::message::rdata::opt_rdata::option_data::OptionData;

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

mod option_data {
    use crate::edns::opt_option;
    use crate::message::rdata::opt_rdata::option_code::OptionCode;
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