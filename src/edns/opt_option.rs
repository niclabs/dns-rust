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