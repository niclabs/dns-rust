use crate::message::rdata::a_rdata::ARdata;
use crate::message::rdata::cname_rdata::CnameRdata;
use crate::message::rdata::hinfo_rdata::HinfoRdata;
use crate::message::rdata::mx_rdata::MxRdata;
use crate::message::rdata::ns_rdata::NsRdata;
use crate::message::rdata::ptr_rdata::PtrRdata;
use crate::message::rdata::soa_rdata::SoaRdata;
use crate::message::rdata::txt_rdata::TxtRdata;
use crate::message::rdata::Rdata;

use crate::domain_name::DomainName;
use std::vec::Vec;

#[derive(Clone)]
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
pub struct ResourceRecord {
    // Domain Name
    name: DomainName,
    // Specifies the meaning of the data in the RDATA
    type_code: u16,
    // Specifies the class of the data in the RDATA
    class: u16,
    // Specifies the time interval (in seconds) that the resource record may be cached before it should be discarded.
    ttl: u32,
    // Specifies the length in octets of the RDATA field
    rdlength: u16,
    // The format of this information varies according to the TYPE and CLASS of the resource record
    rdata: Rdata,
}

/// Trait to convert struct in bytes
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

/// Trait to create an struct from bytes
pub trait FromBytes<T> {
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> T;
}

// Methods
impl ResourceRecord {
    /// Given a rdata, creates a new ResourceRecord with default values and the rdata.
    /// # Examples
    /// ```
    /// let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
    /// let mut resource_record = ResourceRecord::new(txt_rdata);
    ///
    /// assert_eq!(resource_record.name.get_name(), String::from(""));
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

   pub fn new(rdata: Rdata) -> ResourceRecord {
        match rdata {
            Rdata::SomeARdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 1 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeARdata(val),
            },

            Rdata::SomeNsRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 2 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeNsRdata(val),
            },
            Rdata::SomeCnameRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 5 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeCnameRdata(val),
            },
            Rdata::SomeSoaRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 6 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeSoaRdata(val),
            },
            Rdata::SomePtrRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 12 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomePtrRdata(val),
            },
            Rdata::SomeHinfoRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 13 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeHinfoRdata(val),
            },
            Rdata::SomeMxRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 15 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeMxRdata(val),
            },
            Rdata::SomeTxtRdata(val) => ResourceRecord {
                name: DomainName::new(),
                type_code: 16 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeTxtRdata(val),
            },
            _ => ResourceRecord {
                name: DomainName::new(),
                type_code: 0 as u16,
                class: 0 as u16,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: rdata,
            },
        }
    }

   /* pub fn new(rdata: Rdata) -> ResourceRecord {
        let mut resource_record = ResourceRecord {
            name: DomainName::new(),
            type_code: 0 as u16,
            class: 0 as u16,
            ttl: 0 as u32,
            rdlength: 0 as u16,
            rdata: rdata,
        };

        resource_record
    }*/


    /// Given an array of bytes, creates a new ResourceRecord
    /// # Examples
    /// ```
    /// let bytes_msg: [u8; 23] = [
    ///     3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 5, 104,
    ///     101, 108, 108, 111,
    /// ];
    ///
    /// let resource_record_test = ResourceRecord::<Rdata>::from_bytes(&bytes_msg);
    ///
    /// assert_eq!(resource_record_test.get_name().get_name(), String::from("dcc.cl"));
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
    pub fn from_bytes<'a>(
        bytes: &'a [u8],
        full_msg: &'a [u8],
    ) -> Result<(ResourceRecord, &'a [u8]), &'static str> {
        let bytes_len = bytes.len();

        let domain_name_result = DomainName::from_bytes(bytes, full_msg.clone());

        match domain_name_result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let (name, bytes_without_name) = domain_name_result.unwrap();

        if bytes_without_name.len() < 10 {
            return Err("Format Error");
        }

        let type_code = ((bytes_without_name[0] as u16) << 8) | bytes_without_name[1] as u16;
        let class = ((bytes_without_name[2] as u16) << 8) | bytes_without_name[3] as u16;
        let ttl = ((bytes_without_name[4] as u32) << 24)
            | ((bytes_without_name[5] as u32) << 16)
            | ((bytes_without_name[6] as u32) << 8)
            | bytes_without_name[7] as u32;
        let rdlength = ((bytes_without_name[8] as u16) << 8) | bytes_without_name[9] as u16;

        let end_rr_byte = 10 + rdlength as usize;

        if bytes_without_name.len() < end_rr_byte {
            return Err("Format Error");
        }

        let mut rdata_bytes_vec = bytes_without_name[10..].to_vec();
        rdata_bytes_vec.push(bytes_without_name[0]);
        rdata_bytes_vec.push(bytes_without_name[1]);
        rdata_bytes_vec.push(bytes_without_name[2]);
        rdata_bytes_vec.push(bytes_without_name[3]);

        let rdata_result = Rdata::from_bytes(rdata_bytes_vec.as_slice(), full_msg);

        match rdata_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let rdata = rdata_result.unwrap();

        let resource_record = ResourceRecord {
            name: name,
            type_code: type_code,
            class: class,
            ttl: ttl,
            rdlength: rdlength,
            rdata: rdata,
        };

        Ok((resource_record, &bytes_without_name[end_rr_byte..]))
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
    /// let mut resource_record = ResourceRecord::new(txt_rdata);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("dcc.cl"));
    ///
    /// resource_record.set_name(domain_name);
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

        let name_bytes = self.get_name().to_bytes();

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

        let rdata_bytes = self.rdata_to_bytes();
        let rd_length: u16 = rdata_bytes.len() as u16;

        rr_bytes.push((rd_length >> 8) as u8);
        rr_bytes.push(rd_length as u8);

        for byte in rdata_bytes.as_slice() {
            rr_bytes.push(*byte);
        }

        rr_bytes
    }

    pub fn get_string_type(&self) -> String {
        let qtype = match self.get_type_code() {
            1 => "A".to_string(),
            2 => "NS".to_string(),
            5 => "CNAME".to_string(),
            6 => "SOA".to_string(),
            11 => "WKS".to_string(),
            12 => "PTR".to_string(),
            13 => "HINFO".to_string(),
            14 => "MINFO".to_string(),
            15 => "MX".to_string(),
            16 => "TXT".to_string(),
            28 => "AAAA".to_string(),
            _ => unreachable!(),
        };

        qtype
    }
}

// Setters
impl ResourceRecord {
    /// Sets the ame attribute with a value
    pub fn set_name(&mut self, name: DomainName) {
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
    pub fn set_rdata(&mut self, rdata: Rdata) {
        self.rdata = rdata.clone();
    }
}

// Getters
impl ResourceRecord {
    /// Gets the name attribute value
    pub fn get_name(&self) -> DomainName {
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
    pub fn get_rdata(&self) -> Rdata {
        self.rdata.clone()
    }
}

// Tests
mod test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::a_ch_rdata::AChRdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::cname_rdata::CnameRdata;
    use crate::message::rdata::hinfo_rdata::HinfoRdata;
    use crate::message::rdata::mx_rdata::MxRdata;
    use crate::message::rdata::ptr_rdata::PtrRdata;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn constructor_a_test() {
        let  mut a_rdata = Rdata::SomeARdata(ARdata::new());
        match a_rdata {
            Rdata::SomeARdata(ref mut val) => val.set_address([127, 0, 0, 1]),
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(a_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 1);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            },
            [127, 0, 0, 1]
        );
    }

    #[test]
    fn constructor_ns_test() {
        let  mut ns_rdata = Rdata::SomeNsRdata(NsRdata::new());

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match ns_rdata {
            Rdata::SomeNsRdata(ref mut val) => val.set_nsdname(new_domain_name),
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(ns_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 2);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => unreachable!(),
            },
            String::from("test.com")
        );
    }

    #[test]
    fn constructor_cname_test() {
        let  mut cname_rdata = Rdata::SomeCnameRdata(CnameRdata::new());

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match cname_rdata {
            Rdata::SomeCnameRdata(ref mut val) => val.set_cname(new_domain_name),
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(cname_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 5);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeCnameRdata(val) => val.get_cname().get_name(),
                _ => unreachable!(),
            },
            String::from("test.com")
        );
    }

    #[test]
    fn constructor_soa_test() {
        let  mut soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());

        let mut mname_domain_name = DomainName::new();
        mname_domain_name.set_name(String::from("test.com"));

        let mut rname_domain_name = DomainName::new();
        rname_domain_name.set_name(String::from("admin.example.com"));

        match soa_rdata {
            Rdata::SomeSoaRdata(ref mut val) => {val.set_mname(mname_domain_name);
                                                val.set_rname(rname_domain_name);
                                                val.set_serial(1111111111 as u32)},
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(soa_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 6);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeSoaRdata(val) => val.get_mname().get_name(),
                _ => unreachable!(),
            },
            String::from("test.com")
        );
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeSoaRdata(val) => val.get_rname().get_name(),
                _ => unreachable!(),
            },
            String::from("admin.example.com")
        );
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeSoaRdata(val) => val.get_serial(),
                _ => unreachable!(),
            },
            1111111111 as u32
        );
    }

    #[test]
    fn constructor_ptr_test() {
        let  mut ptr_rdata = Rdata::SomePtrRdata(PtrRdata::new());

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match ptr_rdata {
            Rdata::SomePtrRdata(ref mut val) => val.set_ptrdname(new_domain_name),
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(ptr_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 12);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomePtrRdata(val) => val.get_ptrdname().get_name(),
                _ => unreachable!(),
            },
            String::from("test.com")
        );
    }

    #[test]
    fn constructor_hinfo_test() {
        let  mut hinfo_rdata = Rdata::SomeHinfoRdata(HinfoRdata::new());

        let cpu = String::from("INTEL-386");
        let os = String::from("Windows");

        match hinfo_rdata {
            Rdata::SomeHinfoRdata(ref mut val) => { val.set_cpu(cpu);
                                                    val.set_os(os) },
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(hinfo_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 13);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeHinfoRdata(val) => val.get_cpu(),
                _ => unreachable!(),
            },
            String::from("INTEL-386")
        );
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeHinfoRdata(val) => val.get_os(),
                _ => unreachable!(),
            },
            String::from("Windows")
        );
    }

    #[test]
    fn constructor_mx_test() {
        let  mut mx_rdata = Rdata::SomeMxRdata(MxRdata::new());

        let preference = 10 as u16;
        let mut exchange = DomainName::new();
        exchange.set_name(String::from("admin.example.com"));

        match mx_rdata {
            Rdata::SomeMxRdata(ref mut val) => {val.set_preference(preference);
                                                val.set_exchange(exchange) },
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(mx_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 15);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeMxRdata(val) => val.get_preference(),
                _ => unreachable!(),
            },
            10 as u16
        );
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeMxRdata(val) => val.get_exchange().get_name(),
                _ => unreachable!(),
            },
            String::from("admin.example.com")
        );
    }

    #[test]
    fn constructor_txt_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let resource_record = ResourceRecord::new(txt_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 16);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeTxtRdata(val) => val.get_text(),
                _ => unreachable!(),
            },
            vec!["dcc".to_string()]
        );
    }

    #[test]
    fn constructor_other_rdata_test() {
        let  mut ach_rdata = Rdata::SomeAChRdata(AChRdata::new());

        let ch_address = 1 as u16;
        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match ach_rdata {
            Rdata::SomeAChRdata(ref mut val) => {val.set_domain_name(new_domain_name);
                                                val.set_ch_address(ch_address) },
            _ => unreachable!(),
        }
        
        let resource_record = ResourceRecord::new(ach_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(resource_record.type_code, 0);
        assert_eq!(resource_record.class, 0);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeAChRdata(val) => val.get_ch_address(),
                _ => unreachable!(),
            },
            1 as u16
        );
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeAChRdata(val) => val.get_domain_name().get_name(),
                _ => unreachable!(),
            },
            String::from("test.com")
        );
    }

    #[test]
    fn set_and_get_name_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_name().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("Test"));
        resource_record.set_name(domain_name);

        let name = resource_record.get_name().get_name();
        assert_eq!(name, String::from("Test"));
    }

    #[test]
    fn set_and_get_type_code_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_type_code(), 16);

        resource_record.set_type_code(1 as u16);

        let type_code = resource_record.get_type_code();
        assert_eq!(type_code, 1 as u16);
    }

    #[test]
    fn set_and_get_class_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_class(), 0);

        resource_record.set_class(1 as u16);

        let class = resource_record.get_class();
        assert_eq!(class, 1 as u16);
    }

    #[test]
    fn set_and_get_ttl_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_ttl(), 0);

        resource_record.set_ttl(12844 as u32);

        let ttl = resource_record.get_ttl();
        assert_eq!(ttl, 12844 as u32);
    }

    #[test]
    fn set_and_get_rdlength_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);
        assert_eq!(resource_record.get_rdlength(), 0);

        resource_record.set_rdlength(3 as u16);

        let rdlength = resource_record.get_rdlength();
        assert_eq!(rdlength, 3 as u16);
    }

    #[test]
    fn set_and_get_rdata_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        match resource_record.get_rdata() {
            Rdata::SomeTxtRdata(val) => assert_eq!(val.get_text()[0], "dcc".to_string()), 
            _ => unreachable!(), 
        }


        let  mut mx_rdata = Rdata::SomeMxRdata(MxRdata::new());
        resource_record.set_rdata(mx_rdata);

       
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeMxRdata(val) => val.get_preference(),
                _ => unreachable!(),
            },
            0 as u16
        );
    }

    #[test]
    fn to_bytes_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string(), "uchile".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(16);
        resource_record.set_class(1);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(4);

        let bytes_msg = [
            3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 11, 3, 100,
            99, 99, 6, 117, 99, 104, 105, 108, 101
        ];

        let rr_to_bytes = resource_record.to_bytes();

        let mut i = 0;

        for value in rr_to_bytes.as_slice() {
            assert_eq!(*value, bytes_msg[i]);
            i += 1;
        }
    }

    #[test]
    fn from_bytes_test() {

        let mut bytes_msg = [
            3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 4, 3, 100,
            99, 99
        ]; 

        // bytes is not the full msg, but in this case it will not use inside
        let (mut resource_record_test, mut _other_rr_bytes) =
            ResourceRecord::from_bytes(&bytes_msg, &bytes_msg).unwrap();

        assert_eq!(
            resource_record_test.get_name().get_name(),
            String::from("dcc.cl")
        );
        assert_eq!(resource_record_test.get_type_code(), 16);
        assert_eq!(resource_record_test.get_class(), 1);
        assert_eq!(resource_record_test.get_ttl(), 5642);
        assert_eq!(resource_record_test.get_rdlength(), 4);

        assert_eq!(
            match resource_record_test.get_rdata() {
                Rdata::SomeTxtRdata(val) => val.get_text(),
                 _ => unreachable!(),
            },
            vec!["dcc".to_string()]
        );

        bytes_msg = [
            3, 100, 99, 99, 2, 99, 108, 0, 0, 1, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 4, 127, 0,
            0, 1
        ]; 

        let (resource_record_test, _other_rr_bytes) =
            ResourceRecord::from_bytes(&bytes_msg, &bytes_msg).unwrap();

        assert_eq!(
            resource_record_test.get_name().get_name(),
            String::from("dcc.cl")
        );
        assert_eq!(resource_record_test.get_type_code(), 1);
        assert_eq!(resource_record_test.get_class(), 1);
        assert_eq!(resource_record_test.get_ttl(), 5642);
        assert_eq!(resource_record_test.get_rdlength(), 4);

        assert_eq!(
            match resource_record_test.get_rdata() {
                Rdata::SomeARdata(val) => val.get_string_address(),
                 _ => unreachable!(),
            },
            String::from("127.0.0.1")
        );
 
    }

    #[test]
    fn get_string_type_test() {
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);

        assert_eq!(resource_record.get_string_type(), String::from("A"));
    }

}
