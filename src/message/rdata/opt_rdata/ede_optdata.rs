use crate::message::rdata::opt_rdata::ede_code::EdeCode;
use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdeStruct {
    err_code: EdeCode,
    err_message: String,
}

impl EdeStruct {
    pub fn new(err_code: EdeCode, err_message: String) -> Self {
        EdeStruct{err_code, err_message}
    }
    pub fn get_err_code(&self) -> EdeCode {
        self.err_code.clone()
    }
    pub fn get_err_message(&self) -> String {
        self.err_message.clone()
    }
}

impl ToBytes for EdeStruct {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];

        let mut err_code_bytes = self.err_code.to_bytes();
        res.append(&mut err_code_bytes);

        let mut msg_bytes = self.err_message.as_bytes().to_vec();
        res.append(&mut msg_bytes);

        res
    }
}

impl FromBytes<Result<Self, &'static str>> for EdeStruct {
    fn from_bytes(bytes: Result<Self, &'static str>) -> Result<Self, &'static str> {

    }
}

