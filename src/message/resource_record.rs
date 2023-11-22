use crate::message::rdata::Rdata;
use crate::message::Rclass;
use crate::message::Rtype;
use crate::utils;

use crate::domain_name::DomainName;
use std::fmt;
use std::vec::Vec;

#[derive(Clone, PartialEq, Debug)]
/// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.1
/// An struct that represents the Resource Record secction from a dns message.
/// 
/// ```text
/// The Resource Record is composed by:
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
/// ```
pub struct ResourceRecord {
    /// Domain Name
    name: DomainName,
    /// Specifies the meaning of the data in the RDATA.
    rtype: Rtype,
    /// Specifies the class of the data in the RDATA.
    rclass: Rclass,
    /// Specifies the time interval (in seconds) that the resource record may be cached before it should be discarded.
    ttl: u32,
    /// Specifies the length in octets of the RDATA field.
    rdlength: u16,
    /// The format of this information varies according to the TYPE and CLASS of the resource record.
    rdata: Rdata,
}

/// Trait to convert struct in a vector of bytes.
pub trait ToBytes {
    /// Converts struct in a vector of bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

/// Trait to create struct from bytes and full message.
pub trait FromBytes<T> {
    /// Creates struct from bytes and full message.
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> T;
}

// Methods
impl ResourceRecord {
    /// Given a `Rdata`, creates a new `ResourceRecord` with default values and the `Rdata`.
    /// 
    ///  # Examples
    ///  ```
    ///  let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
    ///  let mut resource_record = ResourceRecord::new(txt_rdata);
    ///  assert_eq!(resource_record.name.get_name(), String::from(""));
    ///  assert_eq!(resource_record.rtype, 0);
    ///  assert_eq!(resource_record.class, 0);
    ///  assert_eq!(resource_record.ttl, 0);
    ///  assert_eq!(resource_record.rdlength, 0);
    ///  assert_eq!(
    ///     resource_record.rdata.unwrap().get_text(),
    ///     String::from("dcc")
    ///  );
    ///  ```
    pub fn new(rdata: Rdata) -> ResourceRecord {
        match rdata {
            Rdata::SomeARdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::A,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeARdata(val),
            },

            Rdata::SomeNsRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::NS,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeNsRdata(val),
            },
            Rdata::SomeCnameRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::CNAME,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeCnameRdata(val),
            },
            Rdata::SomeSoaRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::SOA,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeSoaRdata(val),
            },
            Rdata::SomePtrRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::PTR,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomePtrRdata(val),
            },
            Rdata::SomeHinfoRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::HINFO,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeHinfoRdata(val),
            },
            Rdata::SomeMxRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::MX,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeMxRdata(val),
            },
            Rdata::SomeTxtRdata(val) => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::TXT,
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: Rdata::SomeTxtRdata(val),
            },
            _ => ResourceRecord {
                name: DomainName::new(),
                rtype: Rtype::UNKNOWN(0),
                rclass: Rclass::IN,
                ttl: 0 as u32,
                rdlength: 0 as u16,
                rdata: rdata,
            },
        }
    }

    /// Given an array of bytes, creates a new `ResourceRecord`.
    /// 
    /// # Examples
    ///  ```
    ///  let bytes_msg: [u8; 23] = [
    ///      3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 5, 104,
    ///      101, 108, 108, 111,
    ///  ];

    ///  let resource_record_test = ResourceRecord::<Rdata>::from_bytes(&bytes_msg);

    ///  assert_eq!(resource_record_test.get_name().get_name(), String::from("dcc.cl"));
    ///  assert_eq!(resource_record_test.get_rtype(), 16);
    ///  assert_eq!(resource_record_test.get_class(), 1);
    ///  assert_eq!(resource_record_test.get_ttl(), 5642);
    ///  assert_eq!(resource_record_test.get_rdlength(), 5);
    ///  assert_eq!(
    ///      resource_record_test.get_rdata().unwrap().get_text(),
    ///      String::from("hello")
    ///  );
    ///  ```
    pub fn from_bytes<'a>(
        bytes: &'a [u8],
        full_msg: &'a [u8],
    ) -> Result<(ResourceRecord, &'a [u8]), &'static str> {
        let domain_name_result = DomainName::from_bytes(bytes, full_msg);

        match domain_name_result.clone() {
            Ok((domain_name,_)) => {
                utils::domain_validity_syntax(domain_name)?;
            }
            Err(e) => return Err(e),
        }

        let (name, bytes_without_name) = domain_name_result.unwrap();

        if bytes_without_name.len() < 10 {
            return Err("Format Error");
        }

        let type_code = ((bytes_without_name[0] as u16) << 8) | bytes_without_name[1] as u16;
        let rtype = Rtype::from_int_to_rtype(type_code);
        let class = ((bytes_without_name[2] as u16) << 8) | bytes_without_name[3] as u16;
        let rclass = Rclass::from_int_to_rclass(class);
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
            rtype: rtype,
            rclass: rclass,
            ttl: ttl,
            rdlength: rdlength,
            rdata: rdata,
        };

        Ok((resource_record, &bytes_without_name[end_rr_byte..]))
    }

    /// Returns a byte that represents the first byte from type code in the dns message.
    fn get_first_type_code_byte(&self) -> u8 {
        let type_code = Rtype::from_rtype_to_int(self.get_rtype());
        let first_byte = (type_code >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from type code in the dns message.
    fn get_second_type_code_byte(&self) -> u8 {
        let type_code = Rtype::from_rtype_to_int(self.get_rtype());
        let second_byte = type_code as u8;

        second_byte
    }

    /// Returns a byte that represents the first byte from class in the dns message.
    fn get_first_class_byte(&self) -> u8 {
        let class = Rclass::from_rclass_to_int(self.get_rclass());
        let first_byte = (class >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from class in the dns message.
    fn get_second_class_byte(&self) -> u8 {
        let class = Rclass::from_rclass_to_int(self.get_rclass());
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
    #[allow(dead_code)]
    fn get_first_rdlength_byte(&self) -> u8 {
        let rdlength = self.get_rdlength();
        let first_byte = (rdlength >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from rdlength in the dns message.
    #[allow(dead_code)]
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

    /// Returns a `Vec<u8>` of bytes that represent the resource record in the dns message.
    /// 
    /// # Example
    ///  ```
    ///  let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(String::from("dcc")));
    ///  let mut resource_record = ResourceRecord::new(txt_rdata);
    ///  let mut domain_name = DomainName::new();
    ///  domain_name.set_name(String::from("dcc.cl"));

    ///  resource_record.set_name(domain_name);
    ///  resource_record.set_type_code(2);
    ///  resource_record.set_class(1);
    ///  resource_record.set_ttl(5642);
    ///  resource_record.set_rdlength(3);

    ///  let bytes_msg = [
    ///      3, 100, 99, 99, 2, 99, 108, 0, 0, 2, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 3, 100, 99,
    ///     99,
    ///  ];

    ///  let rr_to_bytes = resource_record.to_bytes();

    ///  let mut i = 0;

    ///  for value in rr_to_bytes.as_slice() {
    ///     assert_eq!(*value, bytes_msg[i]);
    ///      i += 1;
    ///  }
    ///  ```
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
        let qtype = Rtype::from_rtype_to_str(self.get_rtype());
        qtype
    }

}

/// Setters
impl ResourceRecord {
    /// Sets the ame attribute with a value.
    pub fn set_name(&mut self, name: DomainName) {
        self.name = name;
    }

    /// Sets the type_code attribute with a value.
    pub fn set_type_code(&mut self, rtype: Rtype) {
        self.rtype = rtype;
    }

    /// Sets the class attribute with a value.
    pub fn set_rclass(&mut self, class: Rclass) {
        self.rclass = class;
    }

    /// Sets the ttl attribute with a value.
    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl;
    }

    /// Sets the rdlength attribute with a value.
    pub fn set_rdlength(&mut self, rdlength: u16) {
        self.rdlength = rdlength;
    }

    /// Sets the rdata attribute with a value.
    pub fn set_rdata(&mut self, rdata: Rdata) {
        self.rdata = rdata.clone();
    }
}

impl ResourceRecord {
    pub fn rr_equal(&mut self, rr: ResourceRecord) -> bool {
        let a: u16 = Rtype::from_rtype_to_int(self.get_rtype());
        let aa: u16 = Rtype::from_rtype_to_int(rr.get_rtype());
        let b: u16 = Rclass::from_rclass_to_int(self.get_rclass());
        let bb: u16 = Rclass::from_rclass_to_int(rr.get_rclass());
        let c: u16 = self.get_rdlength();
        let cc: u16 = rr.get_rdlength();
        let d: u32 = self.get_ttl();
        let dd: u32 = rr.get_ttl();
        let e: Vec<u8> = self.get_rdata().to_bytes();
        let ee: Vec<u8> = rr.get_rdata().to_bytes();
        let n: Vec<u8> = self.get_name().to_bytes();
        let nn: Vec<u8> = rr.get_name().to_bytes();
        let s1 = String::from_utf8(e);
        let s2 = String::from_utf8(ee);
        let s = String::from_utf8(n);
        let ss = String::from_utf8(nn);

        if a == aa && b == bb && c == cc && d == dd && s1 == s2 && s == ss {
            true
        } else {
            false
        }
    }
}

/// Getters
impl ResourceRecord {
    /// Returns a copy of the name attribute value
    pub fn get_name(&self) -> DomainName {
        self.name.clone()
    }

    /// Returns a copy of the `rtype` attribute value.
    pub fn get_rtype(&self) -> Rtype {
        self.rtype.clone()
    }

    /// Returns a copy of the class attribute value.
    pub fn get_rclass(&self) -> Rclass {
        self.rclass.clone()
    }

    //// Returns a copy of the ttl attribute value.
    pub fn get_ttl(&self) -> u32 {
        self.ttl.clone()
    }

    /// Returns a copy of the rdlength attribute value.
    pub fn get_rdlength(&self) -> u16 {
        self.rdlength.clone()
    }

    /// Returns a copy of the rdata attribute value.
    pub fn get_rdata(&self) -> Rdata {
        self.rdata.clone()
    }
}

impl fmt::Display for ResourceRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // We need to remove "-" from the number output.
        let name = self.get_name();
        let type_code = self.get_rtype();
        let class = self.get_rclass();

        formatter.write_fmt(format_args!(
            "RR:{} - type:{} - class:{}",
            name, Rtype::from_rtype_to_int(type_code), Rclass::from_rclass_to_int(class)
        ))
    }
}

#[cfg(test)]
mod resource_record_test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::a_ch_rdata::AChRdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::cname_rdata::CnameRdata;
    use crate::message::rdata::hinfo_rdata::HinfoRdata;
    use crate::message::rdata::mx_rdata::MxRdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::ptr_rdata::PtrRdata;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::rdata::Rdata;
    use crate::message::Rtype;
    use crate::message::Rclass;
    use std::net::IpAddr;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn constructor_a_test() {
        let mut a_rdata = Rdata::SomeARdata(ARdata::new());
        match a_rdata {
            Rdata::SomeARdata(ref mut val) => val.set_address(IpAddr::from([127, 0, 0, 1])),
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(a_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 1);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            },
            IpAddr::from([127, 0, 0, 1])
        );
    }

    #[test]
    fn constructor_ns_test() {
        let mut ns_rdata = Rdata::SomeNsRdata(NsRdata::new());

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match ns_rdata {
            Rdata::SomeNsRdata(ref mut val) => val.set_nsdname(new_domain_name),
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(ns_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 2);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        let mut cname_rdata = Rdata::SomeCnameRdata(CnameRdata::new());

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match cname_rdata {
            Rdata::SomeCnameRdata(ref mut val) => val.set_cname(new_domain_name),
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(cname_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 5);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        let mut soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());

        let mut mname_domain_name = DomainName::new();
        mname_domain_name.set_name(String::from("test.com"));

        let mut rname_domain_name = DomainName::new();
        rname_domain_name.set_name(String::from("admin.example.com"));

        match soa_rdata {
            Rdata::SomeSoaRdata(ref mut val) => {
                val.set_mname(mname_domain_name);
                val.set_rname(rname_domain_name);
                val.set_serial(1111111111 as u32)
            }
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(soa_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 6);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        let mut ptr_rdata = Rdata::SomePtrRdata(PtrRdata::new());

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match ptr_rdata {
            Rdata::SomePtrRdata(ref mut val) => val.set_ptrdname(new_domain_name),
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(ptr_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 12);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        let mut hinfo_rdata = Rdata::SomeHinfoRdata(HinfoRdata::new());

        let cpu = String::from("INTEL-386");
        let os = String::from("Windows");

        match hinfo_rdata {
            Rdata::SomeHinfoRdata(ref mut val) => {
                val.set_cpu(cpu);
                val.set_os(os)
            }
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(hinfo_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 13);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        let mut mx_rdata = Rdata::SomeMxRdata(MxRdata::new());

        let preference = 10 as u16;
        let mut exchange = DomainName::new();
        exchange.set_name(String::from("admin.example.com"));

        match mx_rdata {
            Rdata::SomeMxRdata(ref mut val) => {
                val.set_preference(preference);
                val.set_exchange(exchange)
            }
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(mx_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 15);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        let text = vec!["dcc".to_string(), "test".to_string()];

        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(text));
        let resource_record = ResourceRecord::new(txt_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 16);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
        assert_eq!(resource_record.ttl, 0);
        assert_eq!(resource_record.rdlength, 0);
        assert_eq!(
            match resource_record.get_rdata() {
                Rdata::SomeTxtRdata(val) => val.get_text(),
                _ => unreachable!(),
            },
            vec!["dcc".to_string(), "test".to_string()]
        );
    }

    #[test]
    fn constructor_other_rdata_test() {
        let mut ach_rdata = Rdata::SomeAChRdata(AChRdata::new());

        let ch_address = 1 as u16;
        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        match ach_rdata {
            Rdata::SomeAChRdata(ref mut val) => {
                val.set_domain_name(new_domain_name);
                val.set_ch_address(ch_address)
            }
            _ => unreachable!(),
        }

        let resource_record = ResourceRecord::new(ach_rdata);

        assert_eq!(resource_record.name.get_name(), String::from(""));
        assert_eq!(Rtype::from_rtype_to_int(resource_record.rtype.clone()), 0);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.rclass.clone()), 1);
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
        assert_eq!(Rtype::from_rtype_to_int(resource_record.get_rtype()), 16);

        resource_record.set_type_code(Rtype::A);

        let type_code = Rtype::from_rtype_to_int(resource_record.get_rtype());
        assert_eq!(type_code, 1 as u16);
    }

    #[test]
    fn set_and_get_class_test() {
        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);
        assert_eq!(Rclass::from_rclass_to_int(resource_record.get_rclass()), 1);

        resource_record.set_rclass(Rclass::CS);

        let class = Rclass::from_rclass_to_int(resource_record.get_rclass());
        assert_eq!(class, 2 as u16);
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

        let mx_rdata = Rdata::SomeMxRdata(MxRdata::new());
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
        let txt_rdata =
            Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string(), "uchile".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rtype::TXT);
        resource_record.set_rclass(Rclass::IN);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(4);

        let bytes_msg = [
            3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 11, 3,
            100, 99, 99, 6, 117, 99, 104, 105, 108, 101,
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
            99, 99,
        ];

        // bytes is not the full msg, but in this case it will not use inside
        let (resource_record_test, mut _other_rr_bytes) =
            ResourceRecord::from_bytes(&bytes_msg, &bytes_msg).unwrap();

        assert_eq!(
            resource_record_test.get_name().get_name(),
            String::from("dcc.cl")
        );
        assert_eq!(Rtype::from_rtype_to_int(resource_record_test.get_rtype()), 16);
        assert_eq!(Rclass::from_rclass_to_int(resource_record_test.get_rclass()), 1);
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
            0, 1,
        ];

        let (resource_record_test, _other_rr_bytes) =
            ResourceRecord::from_bytes(&bytes_msg, &bytes_msg).unwrap();

        assert_eq!(
            resource_record_test.get_name().get_name(),
            String::from("dcc.cl")
        );
        assert_eq!(Rtype::from_rtype_to_int(resource_record_test.get_rtype()), 1);
        assert_eq!(Rclass::from_rclass_to_int(resource_record_test.get_rclass()), 1);
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

    #[test]
    fn rr_equal_test() {
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let mut resource_record = ResourceRecord::new(soa_rdata);
        let soa_rdata1 = Rdata::SomeSoaRdata(SoaRdata::new());
        let mut resource_record1 = ResourceRecord::new(soa_rdata1);
        assert!(resource_record.rr_equal(resource_record1.clone()));
        resource_record1.set_rclass(Rclass::HS);
        assert_ne!(resource_record.rr_equal(resource_record1.clone()), true);
        resource_record.set_rclass(Rclass::HS);
        assert!(resource_record.rr_equal(resource_record1.clone()));
        resource_record.set_rdlength(16);
        assert_ne!(resource_record.rr_equal(resource_record1.clone()), true);
    }
}
