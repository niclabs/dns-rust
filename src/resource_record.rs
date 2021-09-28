use std::string::String;

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
pub struct ResourceRecord<T> {
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

// Methods
impl<T> ResourceRecord<T> {
    /// Given a rdata, creates a new ResourceRecord with default values and the rdata.
    /// # Examples
    /// ```
    /// let resource_record = resource_record::ResourceRecord::new(234);
    ///
    /// assert_eq!(resource_record.name, String::from(""));
    /// assert_eq!(resource_record.type_code, 0);
    /// assert_eq!(resource_record.class, 0);
    /// assert_eq!(resource_record.ttl, 0);
    /// assert_eq!(resource_record.rdlength, 0);
    /// assert_eq!(resource_record.rdata, 234);
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
}

// Setters
impl<T> ResourceRecord<T> {
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
impl<T> ResourceRecord<T> {
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
    pub fn get_rdata(&self) -> &T {
        &self.rdata
    }
}

// Tests
mod test {
    use crate::resource_record;

    #[test]
    fn constructor_test() {
        let resource_record = resource_record::ResourceRecord::new(234);

        assert_eq!(resource_record.name, String::from(""));
        assert_eq!(resource_record.type_code, 0);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(resource_record.rdata, 234);
    }

    #[test]
    fn set_and_get_name_test() {
        let mut resource_record = resource_record::ResourceRecord::new(234);
        assert_eq!(resource_record.get_name(), String::from(""));

        resource_record.set_name(String::from("Test"));

        let name = resource_record.get_name();
        assert_eq!(name, String::from("Test"));
    }

    #[test]
    fn set_and_get_type_code_test() {
        let mut resource_record = resource_record::ResourceRecord::new(234);
        assert_eq!(resource_record.get_type_code(), 0);

        resource_record.set_type_code(1 as u16);

        let type_code = resource_record.get_type_code();
        assert_eq!(type_code, 1 as u16);
    }

    #[test]
    fn set_and_get_class_test() {
        let mut resource_record = resource_record::ResourceRecord::new(234);
        assert_eq!(resource_record.get_class(), 0);

        resource_record.set_class(1 as u16);

        let class = resource_record.get_class();
        assert_eq!(class, 1 as u16);
    }

    #[test]
    fn set_and_get_ttl_test() {
        let mut resource_record = resource_record::ResourceRecord::new(234);
        assert_eq!(resource_record.get_ttl(), 0);

        resource_record.set_ttl(12844 as u32);

        let ttl = resource_record.get_ttl();
        assert_eq!(ttl, 12844 as u32);
    }

    #[test]
    fn set_and_get_rdlength_test() {
        let mut resource_record = resource_record::ResourceRecord::new(234);
        assert_eq!(resource_record.get_rdlength(), 0);

        resource_record.set_rdlength(59 as u16);

        let rdlength = resource_record.get_rdlength();
        assert_eq!(rdlength, 59 as u16);
    }
}
