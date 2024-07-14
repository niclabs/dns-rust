use crate::domain_name::DomainName;
use crate::message::rrtype::Rrtype;
use crate::message::Rclass;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};

use std::str::SplitWhitespace;
use std::string::String;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
/// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.14
/// An struct that represents the `Rdata` for txt type.
/// 
/// ```text
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   TXT-DATA                    /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
pub struct TxtRdata {
    /// One or more <character-string>s.
    text: Vec<String>,
}

impl ToBytes for TxtRdata {
    /// Returns a vec of bytes that represents the txt rdata.
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
    /// Creates a new TxtRdata from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let mut string;
        let mut txt: Vec<String> = Vec::new();
        let mut i = 0;

        string = String::from("");
        let lenght_octet = bytes[0];

        for _chars in 0..lenght_octet {
            i = i + 1;
            let byte = bytes[i];
            string.push(byte as char);
        }

        txt.push(string);

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
    pub fn new(text: Vec<String>) -> Self {
        let txt_rdata = TxtRdata { text: text };

        txt_rdata
    }

    /// Returns a `ResourceRecord` from the given values.
    /// 
    /// # Examples
    /// ```
    /// let txtrdata_rr = TxtRdata::rr_from_master_file(
    ///     "dcc uchile cl".split_whitespace(),
    ///     25,
    ///     String::from("IN"),
    ///     String::from("uchile.cl"));
    /// 
    /// assert_eq!(txtrdata_rr.get_class(), Rclass::IN);
    /// assert_eq!(txtrdata_rr.get_ttl(), 25);
    /// assert_eq!(txtrdata_rr.get_rdlength(), 3);
    /// assert_eq!(txtrdata_rr.get_name().get_name(), String::from("uchile.cl"));
    /// 
    /// let txt_rr_rdata = txtrdata_rr.get_rdata();
    /// let mut expected_text: Vec<String> = Vec::new();
    /// expected_text.push(String::from("dcc"));
    /// expected_text.push(String::from("uchile"));
    /// expected_text.push(String::from("cl"));
    /// match txt_rr_rdata {
    ///     Rdata::TXT(val) => assert_eq!(val.get_text(), expected_text),
    ///     _ => {}
    /// }
    /// ```
    pub fn rr_from_master_file(
        values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
    ) -> ResourceRecord {
        let mut text: Vec<String> = Vec::new();
        for string in values {
            text.push(string.to_string());
        }

        let rd_lenght = text.len();
        let txt_rdata = TxtRdata::new(text);

        let rdata = Rdata::TXT(txt_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rrtype::TXT);
        let rclass = Rclass::from(class);
        resource_record.set_rclass(rclass);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(rd_lenght as u16);

        resource_record
    }
}

/// Getters
impl TxtRdata {
    /// Gets the text attribute.
    pub fn get_text(&self) -> Vec<String> {
        self.text.clone()
    }
}

/// Setters
impl TxtRdata {
    /// Sets the text field with a value.
    pub fn set_text(&mut self, text: Vec<String>) {
        self.text = text;
    }
}

impl fmt::Display for TxtRdata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::new();
        for string in &self.text {
            text.push_str(string);
            text.push_str(" ");
        }
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod txt_rdata_test {
    use crate::message::Rclass;
    use crate::message::rdata::Rdata;
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
        let text = vec!["dcc test".to_string()];
        let txt_rdata = TxtRdata::new(text);

        let bytes_test = [8, 100, 99, 99, 32, 116, 101, 115, 116];

        assert_eq!(txt_rdata.to_bytes(), bytes_test);
    }

    #[test]
    fn from_bytes_test() {
        let bytes = [8, 100, 99, 99, 32, 116, 101, 115, 116];

        let txt_rdata = TxtRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(txt_rdata.get_text(), vec!["dcc test".to_string()]);
    }

    //ToDo: Revisar
    #[test]
    fn rr_from_master_file_test(){
        let txtrdata_rr = TxtRdata::rr_from_master_file(
            "dcc uchile cl".split_whitespace(),
            25,
            "IN",
            String::from("uchile.cl"));

        assert_eq!(txtrdata_rr.get_rclass(), Rclass::IN);
        assert_eq!(txtrdata_rr.get_ttl(), 25);
        assert_eq!(txtrdata_rr.get_rdlength(), 3);
        assert_eq!(txtrdata_rr.get_name().get_name(), String::from("uchile.cl"));

        let txt_rr_rdata = txtrdata_rr.get_rdata();
        let mut expected_text: Vec<String> = Vec::new();
        expected_text.push(String::from("dcc"));
        expected_text.push(String::from("uchile"));
        expected_text.push(String::from("cl"));
        match txt_rr_rdata {
            Rdata::TXT(val) => assert_eq!(val.get_text(), expected_text),
            _ => {}
        }
    }
}
