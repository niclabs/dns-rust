use crate::message::rdata::opt_rdata::option_code::OptionCode;
use crate::message::rdata::opt_rdata::option_data::OptionData;

use super::ede_code::EdeCode;
use super::ede_optdata::EdeOptData;

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
                    opt_data: OptionData::EDE(EdeOptData::new(EdeCode::Blocked, String::new()))
                }

            },
            OptionCode::PADDING => {
                OptOption {
                    option_code,
                    option_len: 0,
                    opt_data: OptionData::Padding(Vec::new())
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