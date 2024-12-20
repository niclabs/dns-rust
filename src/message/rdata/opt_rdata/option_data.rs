use std::convert::TryInto;
use crate::message::rdata::opt_rdata::ede_optdata::EdeStruct;
use crate::message::rdata::opt_rdata::option_code::OptionCode;
use crate::message::resource_record::ToBytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionData {
    NSID(String),
    EDE(EdeStruct),
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
    fn from_with_opt(bytes: Vec<u8>, opt_t: OptionCode) -> OptionData {
        match opt_t {
            OptionCode::NSID => {
                OptionData::NSID(bytes.try_into().unwrap())
            },
            OptionCode::EDE => {
                EdeStruct::from()
            },
            _ => OptionData::Unknown(bytes)
        }
    }
}