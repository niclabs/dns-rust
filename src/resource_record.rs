use crate::rdata::Rdata;
use crate::txt_rdata;
use std::string::String;
use std::vec::Vec;

/// An struct that represents the resource record secction from a dns message
///                               1  1  1  1  1  1
/// 0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                                               /
/// /                      NAME                     /
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     CLASS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TTL                      |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                   RDLENGTH                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/// /                     RDATA                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct ResourceRecord<T: ToBytes> {
    // Domain Name
    name: String,
    // Specifies the meaning of the data in the RDATA
    type_code: u16,
    // Specifies the class of the data in the RDATA
    class: u16,
    // Specifies the time interval (in seconds) that the resource record may be cached before it should be discarded.
    ttl: u32,
    // Specifies the length in octets of the RDATA field
    rdlength: u16,
    // The format of this information varies according to the TYPE and CLASS of the resource record
    rdata: T,
}

/// Trait to convert struct in bytes
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

/// Trait to create an struct from bytes
pub trait FromBytes<T> {
    fn from_bytes(bytes: &[u8]) -> T;
}

// Methods
impl<T: Clone + ToBytes> ResourceRecord<T> {
    /// Given a rdata, creates a new ResourceRecord with default values and the rdata.
    /// # Examples
    /// ```
    /// let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
    /// let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
    ///
    /// assert_eq!(resource_record.name, String::from(""));
    /// assert_eq!(resource_record.type_code, 0);
    /// assert_eq!(resource_record.class, 0);
    /// assert_eq!(resource_record.ttl, 0);
    /// assert_eq!(resource_record.rdlength, 0);
    /// assert_eq!(
    ///    resource_record.rdata.unwrap().get_text(),
    ///    String::from("dcc")
    /// );
    /// ```
    ///
    pub fn new(rdata: T) -> ResourceRecord<T> {
        let resource_record = ResourceRecord {
            name: String::from(""),
            type_code: 0 as u16,
            class: 0 as u16,
            ttl: 0 as u32,
            rdlength: 0 as u16,
            rdata: rdata,
        };
        resource_record
    }

    /// Given an array of bytes, creates a new ResourceRecord
    /// # Examples
    /// ```
    /// let bytes_msg: [u8; 23] = [
    ///     3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 5, 104,
    ///     101, 108, 108, 111,
    /// ];
    ///
    /// let resource_record_test = resource_record::ResourceRecord::<Rdata>::from_bytes(&bytes_msg);
    ///
    /// assert_eq!(resource_record_test.get_name(), String::from("dcc.cl"));
    /// assert_eq!(resource_record_test.get_type_code(), 16);
    /// assert_eq!(resource_record_test.get_class(), 1);
    /// assert_eq!(resource_record_test.get_ttl(), 5642);
    /// assert_eq!(resource_record_test.get_rdlength(), 5);
    /// assert_eq!(
    ///     resource_record_test.get_rdata().unwrap().get_text(),
    ///     String::from("hello")
    /// );
    /// ```
    ///
    fn from_bytes(bytes: &[u8]) -> ResourceRecord<Rdata> {
        let (name, bytes_without_name) = bytes_to_name(bytes);

        let type_code = ((bytes_without_name[0] as u16) << 8) | bytes_without_name[1] as u16;
        let class = ((bytes_without_name[2] as u16) << 8) | bytes_without_name[3] as u16;
        let ttl = ((bytes_without_name[4] as u32) << 24)
            | ((bytes_without_name[5] as u32) << 16)
            | ((bytes_without_name[6] as u32) << 8)
            | bytes_without_name[7] as u32;
        let rdlength = ((bytes_without_name[8] as u16) << 8) | bytes_without_name[9] as u16;
        let rdata = from_bytes_to_rdata(&bytes_without_name[10..], type_code);

        let resource_record = ResourceRecord {
            name: name,
            type_code: type_code,
            class: class,
            ttl: ttl,
            rdlength: rdlength,
            rdata: rdata,
        };

        resource_record
    }

    /// Returns a vec of bytes that represents the domain name in a dns message
    fn name_to_bytes(&self) -> Vec<u8> {
        let name = self.get_name();
        let mut bytes: Vec<u8> = Vec::new();

        for word in name.split(".") {
            let word_length = word.len();
            bytes.push(word_length as u8);

            for character in word.chars() {
                bytes.push(character as u8);
            }
        }

        bytes.push(0 as u8);

        bytes
    }

    /// Returns a byte that represents the first byte from type code in the dns message.
    fn get_first_type_code_byte(&self) -> u8 {
        let type_code = self.get_type_code();
        let first_byte = (type_code >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from type code in the dns message.
    fn get_second_type_code_byte(&self) -> u8 {
        let type_code = self.get_type_code();
        let second_byte = type_code as u8;

        second_byte
    }

    /// Returns a byte that represents the first byte from class in the dns message.
    fn get_first_class_byte(&self) -> u8 {
        let class = self.get_class();
        let first_byte = (class >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from class in the dns message.
    fn get_second_class_byte(&self) -> u8 {
        let class = self.get_class();
        let second_byte = class as u8;

        second_byte
    }

    /// Returns a byte that represents the first byte from ttl in the dns message.
    fn get_first_ttl_byte(&self) -> u8 {
        let ttl = self.get_ttl();
        let first_byte = (ttl >> 24) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from ttl in the dns message.
    fn get_second_ttl_byte(&self) -> u8 {
        let ttl = self.get_ttl();
        let second_byte = (ttl >> 16) as u8;

        second_byte
    }

    /// Returns a byte that represents the third byte from ttl in the dns message.
    fn get_third_ttl_byte(&self) -> u8 {
        let ttl = self.get_ttl();
        let third_byte = (ttl >> 8) as u8;

        third_byte
    }

    /// Returns a byte that represents the fourth byte from ttl in the dns message.
    fn get_fourth_ttl_byte(&self) -> u8 {
        let ttl = self.get_ttl();
        let fourth_byte = ttl as u8;

        fourth_byte
    }

    /// Returns a byte that represents the first byte from rdlength in the dns message.
    fn get_first_rdlength_byte(&self) -> u8 {
        let rdlength = self.get_rdlength();
        let first_byte = (rdlength >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from rdlength in the dns message.
    fn get_second_rdlength_byte(&self) -> u8 {
        let rdlength = self.get_rdlength();
        let second_byte = rdlength as u8;

        second_byte
    }

    /// Returns a vec of bytes that represents the rdata in the dns message.
    fn rdata_to_bytes(&self) -> Vec<u8> {
        let rdata = self.get_rdata();

        rdata.to_bytes()
    }

    /// Returns a vec fo bytes that represents the resource record
    ///
    /// # Example
    /// ```
    /// let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
    /// let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
    ///
    /// resource_record.set_name(String::from("dcc.cl"));
    /// resource_record.set_type_code(2);
    /// resource_record.set_class(1);
    /// resource_record.set_ttl(5642);
    /// resource_record.set_rdlength(3);
    ///
    /// let bytes_msg = [
    ///     3, 100, 99, 99, 2, 99, 108, 0, 0, 2, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 3, 100, 99,
    ///     99,
    /// ];
    ///
    /// let rr_to_bytes = resource_record.to_bytes();
    ///
    /// let mut i = 0;
    ///
    /// for value in rr_to_bytes.as_slice() {
    ///     assert_eq!(*value, bytes_msg[i]);
    ///     i += 1;
    /// }
    /// ```
    ///
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut rr_bytes: Vec<u8> = Vec::new();

        let name_bytes = self.name_to_bytes();

        for byte in name_bytes.as_slice() {
            rr_bytes.push(*byte);
        }

        rr_bytes.push(self.get_first_type_code_byte());
        rr_bytes.push(self.get_second_type_code_byte());
        rr_bytes.push(self.get_first_class_byte());
        rr_bytes.push(self.get_second_class_byte());
        rr_bytes.push(self.get_first_ttl_byte());
        rr_bytes.push(self.get_second_ttl_byte());
        rr_bytes.push(self.get_third_ttl_byte());
        rr_bytes.push(self.get_fourth_ttl_byte());
        rr_bytes.push(self.get_first_rdlength_byte());
        rr_bytes.push(self.get_second_rdlength_byte());

        let rdata_bytes = self.rdata_to_bytes();

        println!("{:#?}", rdata_bytes);

        for byte in rdata_bytes.as_slice() {
            rr_bytes.push(*byte);
        }

        rr_bytes
    }
}

// Setters
impl<T: Clone + ToBytes> ResourceRecord<T> {
    /// Sets the ame attribute with a value
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the type_code attribute with a value
    pub fn set_type_code(&mut self, type_code: u16) {
        self.type_code = type_code;
    }

    /// Sets the class attribute with a value
    pub fn set_class(&mut self, class: u16) {
        self.class = class;
    }

    /// Sets the ttl attribute with a value
    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl;
    }

    /// Sets the rdlength attribute with a value
    pub fn set_rdlength(&mut self, rdlength: u16) {
        self.rdlength = rdlength;
    }

    /// Sets the rdata attribute with a value
    pub fn set_rdata(&mut self, rdata: T) {
        self.rdata = rdata;
    }
}

// Getters
impl<T: Clone + ToBytes> ResourceRecord<T> {
    /// Gets the name attribute value
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Gets the type_code attribute value
    pub fn get_type_code(&self) -> u16 {
        self.type_code
    }

    /// Gets the class attribute value
    pub fn get_class(&self) -> u16 {
        self.class
    }

    /// Gets the ttl attribute value
    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    /// Gets the rdlength attribute value
    pub fn get_rdlength(&self) -> u16 {
        self.rdlength
    }

    /// Gets the rdata attribute value
    pub fn get_rdata(&self) -> T {
        self.rdata.clone()
    }
}

/// Given an array of bytes, returns an String with the domain name
fn bytes_to_name(bytes: &[u8]) -> (String, &[u8]) {
    let mut name = String::from("");
    let mut index = 0;

    for byte in bytes {
        if *byte <= 9 && *byte >= 1 {
            name.push('.');
        } else if *byte == 0 {
            break;
        } else {
            name.push(*byte as char);
        }
        index += 1;
    }

    name.remove(0);

    (name, &bytes[index + 1..])
}

/// Given an array of bytes and a type code, returns a new Rdata
pub fn from_bytes_to_rdata(bytes: &[u8], type_code: u16) -> Rdata {
    let rdata = match type_code {
        16 => Rdata::SomeTxtRdata(txt_rdata::TxtRdata::from_bytes(bytes)),
        _ => unreachable!(),
    };
    rdata
}

// Tests
mod test {
    use crate::rdata::Rdata;
    use crate::rdata::Unwrap;
    use crate::resource_record;
    use crate::txt_rdata::TxtRdata;

    #[test]
    fn constructor_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let resource_record = resource_record::ResourceRecord::new(txt_rdata);

        assert_eq!(resource_record.name, String::from(""));
        assert_eq!(resource_record.type_code, 0);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            resource_record.rdata.unwrap().get_text(),
            String::from("dcc")
        );
    }

    #[test]
    fn set_and_get_name_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_name(), String::from(""));

        resource_record.set_name(String::from("Test"));

        let name = resource_record.get_name();
        assert_eq!(name, String::from("Test"));
    }

    #[test]
    fn set_and_get_type_code_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_type_code(), 0);

        resource_record.set_type_code(1 as u16);

        let type_code = resource_record.get_type_code();
        assert_eq!(type_code, 1 as u16);
    }

    #[test]
    fn set_and_get_class_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_class(), 0);

        resource_record.set_class(1 as u16);

        let class = resource_record.get_class();
        assert_eq!(class, 1 as u16);
    }

    #[test]
    fn set_and_get_ttl_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_ttl(), 0);

        resource_record.set_ttl(12844 as u32);

        let ttl = resource_record.get_ttl();
        assert_eq!(ttl, 12844 as u32);
    }

    #[test]
    fn set_and_get_rdlength_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_rdlength(), 0);

        resource_record.set_rdlength(59 as u16);

        let rdlength = resource_record.get_rdlength();
        assert_eq!(rdlength, 59 as u16);
    }

    #[test]
    fn to_bytes_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
        let mut resource_record = resource_record::ResourceRecord::new(txt_rdata);

        resource_record.set_name(String::from("dcc.cl"));
        resource_record.set_type_code(16);
        resource_record.set_class(1);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(3);

        let bytes_msg = [
            3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 3, 100,
            99, 99,
        ];

        let rr_to_bytes = resource_record.to_bytes();

        let mut i = 0;

        for value in rr_to_bytes.as_slice() {
            assert_eq!(*value, bytes_msg[i]);
            i += 1;
        }
    }

    #[test]
    fn bytes_to_resource_record_test() {
        let bytes_msg: [u8; 23] = [
            3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 5, 104,
            101, 108, 108, 111,
        ];

        let resource_record_test = resource_record::ResourceRecord::<Rdata>::from_bytes(&bytes_msg);

        assert_eq!(resource_record_test.get_name(), String::from("dcc.cl"));
        assert_eq!(resource_record_test.get_type_code(), 16);
        assert_eq!(resource_record_test.get_class(), 1);
        assert_eq!(resource_record_test.get_ttl(), 5642);
        assert_eq!(resource_record_test.get_rdlength(), 5);
        assert_eq!(
            resource_record_test.get_rdata().unwrap().get_text(),
            String::from("hello")
        );
    }
}
