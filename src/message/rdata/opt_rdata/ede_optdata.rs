use crate::message::rdata::opt_rdata::ede_code::EdeCode;
use crate::message::resource_record::{FromBytes, ToBytes};

/*
                                             1   1   1   1   1   1
     0   1   2   3   4   5   6   7   8   9   0   1   2   3   4   5
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
0: |                            OPTION-CODE                        |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
2: |                           OPTION-LENGTH                       |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
4: | INFO-CODE                                                     |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
6: / EXTRA-TEXT ...                                                /
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
*/
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
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let err_code = EdeCode::from_bytes(&bytes[0..2]).expect("Error parsing EdeCode");
        let err_message = String::from_utf8(bytes[2..].to_vec()).unwrap();
        Ok(EdeStruct::new(err_code, err_message))
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


