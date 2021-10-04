use crate::resource_record::FromBytes;
use crate::resource_record::ToBytes;
use std::string::String;

#[derive(Clone)]
pub struct TxtRdata {
    text: String,
}

impl ToBytes for TxtRdata {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl FromBytes<TxtRdata> for TxtRdata {
    fn from_bytes(bytes: &[u8]) -> TxtRdata {
        let mut string = String::from("");

        for byte in bytes {
            string.push(*byte as char);
        }

        let txt_rdata = TxtRdata::new(string);

        txt_rdata
    }
}

impl TxtRdata {
    pub fn new(text: String) -> TxtRdata {
        let txt_rdata = TxtRdata { text: text };

        txt_rdata
    }
}

impl TxtRdata {
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}
