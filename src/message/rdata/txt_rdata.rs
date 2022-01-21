use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};

use std::str::SplitWhitespace;
use std::string::String;

#[derive(Clone)]
/// An struct that represents the rdata for txt type.
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   TXT-DATA                    /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct TxtRdata {
    // One or more <character-string>s.
    text: Vec<String>,
}

impl ToBytes for TxtRdata {
    /// Return a vec of bytes that represents the txt rdata
    fn to_bytes(&self) -> Vec<u8> {
        let text = self.get_text();
        let mut bytes: Vec<u8> = Vec::new();

        for mut string in text {
            let lenght_octet = string.len();
            bytes.push(lenght_octet as u8);
            for _character_index in 0..string.len() {
                let character_to_byte = string.remove(0) as u8;
                bytes.push(character_to_byte);
            }
        }

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for TxtRdata {
    /// Creates a new TxtRdata from an array of bytes
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let mut string = String::from("");
        let mut txt: Vec<String> = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            let mut lenght_octet = bytes[i];
            i += 1;
            for _chars in 0..lenght_octet {
                let byte = bytes[i];
                string.push(byte as char);
                i += 1;
            }
            txt.push(string);
            string = String::from("");
        }

        let txt_rdata = TxtRdata::new(txt);

        Ok(txt_rdata)
    }
}

impl TxtRdata {
    /// Creates a new TxtRdata.
    ///
    /// # Examples
    /// ```
    /// let txt_rdata = TxtRdata::new(String::from("test"));
    ///
    /// assert_eq!(txt_rdata.text, String::from("test"));
    /// ```
    ///
    pub fn new(text: Vec<String>) -> Self {
        let txt_rdata = TxtRdata { text: text };

        txt_rdata
    }

    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: u16,
        host_name: String,
    ) -> ResourceRecord {
        let mut text: Vec<String> = Vec::new();
        for string in values {
            text.push(string.to_string());
        }

        let rd_lenght = text.len();
        let mut txt_rdata = TxtRdata::new(text);

        let rdata = Rdata::SomeTxtRdata(txt_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(16);
        resource_record.set_class(class);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(rd_lenght as u16);

        resource_record
    }
}

// Getters
impl TxtRdata {
    // Gets the text attribute
    pub fn get_text(&self) -> Vec<String> {
        self.text.clone()
    }
}

// Setters
impl TxtRdata {
    // Sets the text field with a value
    pub fn set_text(&mut self, text: Vec<String>) {
        self.text = text;
    }
}

mod test {
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let text = vec!["constructor".to_string(), "test".to_string()];
        let txt_rdata = TxtRdata::new(text);

        assert_eq!(
            txt_rdata.text,
            vec!["constructor".to_string(), "test".to_string()]
        );
    }

    #[test]
    fn set_and_get_text_test() {
        let mut txt_rdata = TxtRdata::new(vec!["".to_string()]);
        txt_rdata.set_text(vec!["test".to_string()]);

        assert_eq!(txt_rdata.get_text(), vec!["test".to_string()]);
    }

    #[test]
    fn to_bytes_test() {
        let text = vec!["dcc".to_string(), "test".to_string()];
        let txt_rdata = TxtRdata::new(text);

        let bytes_test = [3, 100, 99, 99, 4, 116, 101, 115, 116];

        assert_eq!(txt_rdata.to_bytes(), bytes_test);
    }

    #[test]
    fn from_bytes_test() {
        let bytes_test: [u8; 9] = [3, 100, 99, 99, 4, 116, 101, 115, 116];

        // bytes is not the full msg, but in this case it will not use inside
        let txt_rdata = TxtRdata::from_bytes(&bytes_test, &bytes_test).unwrap();

        assert_eq!(
            txt_rdata.get_text(),
            vec!["dcc".to_string(), "test".to_string()]
        );
    }
}
