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
    // One or more characters
    text: String,
}

impl ToBytes for TxtRdata {
    /// Return a vec of bytes that represents the txt rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut text = self.get_text();
        let mut bytes: Vec<u8> = Vec::new();

        for _character_index in 0..text.len() {
            let character_to_byte = text.remove(0) as u8;
            bytes.push(character_to_byte);
        }

        bytes
    }
}

impl FromBytes<TxtRdata> for TxtRdata {
    /// Creates a new TxtRdata from an array of bytes
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Self {
        let mut string = String::from("");

        for byte in bytes {
            string.push(*byte as char);
        }

        let txt_rdata = TxtRdata::new(string);

        txt_rdata
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
    pub fn new(text: String) -> Self {
        let txt_rdata = TxtRdata { text: text };

        txt_rdata
    }

    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: String,
        host_name: String,
    ) -> ResourceRecord {
        let text = values.next().unwrap();

        let mut txt_rdata = TxtRdata::new(text.to_string());

        let rdata = Rdata::SomeTxtRdata(txt_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(16);

        let class_int = match class.as_str() {
            "IN" => 1,
            "CS" => 2,
            "CH" => 3,
            "HS" => 4,
            _ => unreachable!(),
        };

        resource_record.set_class(class_int);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(text.len() as u16);

        resource_record
    }
}

// Getters
impl TxtRdata {
    // Gets the text attribute
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

// Setters
impl TxtRdata {
    // Sets the text field with a value
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

mod test {
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let txt_rdata = TxtRdata::new(String::from("test"));

        assert_eq!(txt_rdata.text, String::from("test"));
    }

    #[test]
    fn set_and_get_text_test() {
        let mut txt_rdata = TxtRdata::new(String::from("test"));

        txt_rdata.set_text(String::from("second test"));

        assert_eq!(txt_rdata.get_text(), String::from("second test"));
    }

    #[test]
    fn to_bytes_test() {
        let txt_rdata = TxtRdata::new(String::from("dcc"));

        let bytes_test = [100, 99, 99];

        assert_eq!(txt_rdata.to_bytes(), bytes_test);
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 4] = [116, 101, 115, 116];

        // bytes is not the full msg, but in this case it will not use inside
        let txt_rdata = TxtRdata::from_bytes(&bytes, &bytes);

        assert_eq!(txt_rdata.get_text(), String::from("test"));
    }
}
