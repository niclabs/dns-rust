use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::rrtype::Rrtype;
use crate::message::Rclass;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use std::str::SplitWhitespace;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
/// Struct for the TSIG RData
/// [RFC 2845](https://tools.ietf.org/html/rfc2845#section-3.5)
/// [RFC 8945](https://tools.ietf.org/html/rfc8945#section-3.5)
/// ```text
///                     1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                         Algorithm Name                        /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |          Time Signed          +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                               |            Fudge              |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          MAC Size             |                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+             MAC               /
/// /                                                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          Original ID          |            Error              |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          Other Len            |                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+           Other Data          /
/// /                                                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
/// 

pub struct TSigRdata {
    /// The algorithm name in domain name format
    algorithm_name: DomainName,
    /// Seconds since 1-Jan-70 UTC
    time_signed: u64,
    /// Seconds of error permitted in time_signed
    fudge: u16,
    /// Total number of octets in MAC
    mac_size: u16,
    /// MAC of the message
    mac: Vec<u8>,
    /// Original ID of the message
    original_id: u16,
    /// expanded RCODE covering TSIG processing.
    error: u16,
    /// Length in octets of the other_data field
    other_len: u16,
    /// Other data empty unless error == BADTIME
    other_data: Vec<u8>,
}

impl ToBytes for TSigRdata{
    /// Returns a `Vec<u8>` of bytes that represents the TSig RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let algorithm_name_bytes = self.get_algorithm_name().to_bytes();

        for byte in algorithm_name_bytes.as_slice() {
            bytes.push(*byte);
        }
        
        let time_signed = self.get_time_signed();

        bytes.push((time_signed >> 40) as u8);

        bytes.push((time_signed >> 32) as u8);

        bytes.push((time_signed >> 24) as u8);

        bytes.push((time_signed >> 16) as u8);

        bytes.push((time_signed >> 8) as u8);

        bytes.push((time_signed >> 0) as u8);
        
        let fudge = self.get_fudge();

        bytes.push((fudge >> 8) as u8);

        bytes.push(fudge as u8);

        let mac_size = self.get_mac_size();

        bytes.push((mac_size >> 8) as u8);

        bytes.push(mac_size as u8);

        let mac = self.get_mac();

        for byte in mac.as_slice() {
            bytes.push(*byte);
        }

        let original_id = self.get_original_id();

        bytes.push((original_id >> 8) as u8);

        bytes.push(original_id as u8);

        let error = self.get_error();

        bytes.push((error >> 8) as u8);

        bytes.push(error as u8);

        let other_len = self.get_other_len();

        bytes.push((other_len >> 8) as u8);

        bytes.push(other_len as u8);    

        let other_data = self.get_other_data();

        for byte in other_data.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for TSigRdata{

    /// Creates a new `TSigRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let algorithm_name_result = DomainName::from_bytes(bytes, full_msg);

        match algorithm_name_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let (algorithm_name, bytes_without_algorithm_name) = algorithm_name_result.unwrap();

        let mut tsig_rdata = TSigRdata::new();

        tsig_rdata.set_algorithm_name(algorithm_name);

        tsig_rdata.set_time_signed_from_bytes(&bytes_without_algorithm_name[0..6]);

        tsig_rdata.set_fudge_from_bytes(&bytes_without_algorithm_name[6..8]);

        tsig_rdata.set_mac_size_from_bytes(&bytes_without_algorithm_name[8..10]);

        let mac_size = tsig_rdata.get_mac_size();

        let mac = bytes_without_algorithm_name[10..(10 + mac_size as usize)].to_vec();

        tsig_rdata.set_mac(mac);

        let bytes_without_mac = &bytes_without_algorithm_name[(10 + mac_size as usize)..];

        tsig_rdata.set_original_id_from_bytes(&bytes_without_mac[0..2]);

        tsig_rdata.set_error_from_bytes(&bytes_without_mac[2..4]);

        tsig_rdata.set_other_len_from_bytes(&bytes_without_mac[4..6]);

        let other_len = tsig_rdata.get_other_len();

        let other_data = bytes_without_mac[6..(6 + other_len as usize)].to_vec();

        tsig_rdata.set_other_data(other_data);

        Ok(tsig_rdata)
    }
}

impl TSigRdata {
    /// Creates a new TSigRdata with default values
    ///
    /// # Examples
    /// ```
    /// let tsig_rdata = TSigRdata::new();
    /// ```
    pub fn new() -> TSigRdata {
        TSigRdata {
            algorithm_name: DomainName::new(),
            time_signed: 0,
            fudge: 0,
            mac_size: 0,
            mac: Vec::new(),
            original_id: 0,
            error: 0,
            other_len: 0,
            other_data: Vec::new(),
        }
    }

    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
        origin: String,
    ) -> ResourceRecord{
        let mut tsig_rdata = TSigRdata::new();

        let algorithm_name_str = values.next().unwrap();
        let time_signed = values.next().unwrap().parse::<u64>().unwrap();
        let fudge = values.next().unwrap().parse::<u16>().unwrap();
        let mac_size = values.next().unwrap().parse::<u16>().unwrap();
        let mac_str = values.next().unwrap();
        let original_id = values.next().unwrap().parse::<u16>().unwrap();
        let error = values.next().unwrap().parse::<u16>().unwrap();
        let other_len = values.next().unwrap().parse::<u16>().unwrap();
        let mut other_data_str = "";
        if other_len != 0 {
            other_data_str = values.next().unwrap();
        }

        let algorithm_name = DomainName::from_master_file(algorithm_name_str.to_string(), origin.clone());
        let mac = mac_str.as_bytes().chunks(2)
            .map(|b: &[u8]| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
            .collect::<Vec<u8>>();
        let other_data = other_data_str.as_bytes().chunks(2)
            .map(|b: &[u8]| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
            .collect::<Vec<u8>>();

        tsig_rdata.set_algorithm_name(algorithm_name);
        tsig_rdata.set_time_signed(time_signed);
        tsig_rdata.set_fudge(fudge);
        tsig_rdata.set_mac_size(mac_size);
        tsig_rdata.set_mac(mac);
        tsig_rdata.set_original_id(original_id);
        tsig_rdata.set_error(error);
        tsig_rdata.set_other_len(other_len);
        tsig_rdata.set_other_data(other_data);

        let rdata = Rdata::TSIG(tsig_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rrtype::TSIG);

        let rclass = Rclass::from(class);
        resource_record.set_rclass(rclass);
        resource_record.set_ttl(ttl);
        let rdlength = algorithm_name_str.len() as u16 + 18 + mac_size + other_len;
        resource_record.set_rdlength(rdlength);

        resource_record
    }

    //DNSSEC
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        // 1. Algorithm Name (as domain name)
        bytes.extend(self.algorithm_name.to_canonical_bytes());

        // 2. Time Signed: 48-bit timestamp (u64, but only lower 48 bits used)
        let time_high = ((self.time_signed >> 32) & 0xFFFF) as u16;
        let time_low = (self.time_signed & 0xFFFFFFFF) as u32;
        bytes.extend_from_slice(&time_high.to_be_bytes());
        bytes.extend_from_slice(&time_low.to_be_bytes());

        // 3. Fudge
        bytes.extend_from_slice(&self.fudge.to_be_bytes());

        // 4. MAC Size and MAC
        bytes.extend_from_slice(&self.mac_size.to_be_bytes());
        bytes.extend_from_slice(&self.mac);

        // 5. Original ID
        bytes.extend_from_slice(&self.original_id.to_be_bytes());

        // 6. Error
        bytes.extend_from_slice(&self.error.to_be_bytes());

        // 7. Other Len and Other Data
        bytes.extend_from_slice(&self.other_len.to_be_bytes());
        bytes.extend_from_slice(&self.other_data);

        bytes
    }

    /// Set the time signed attribute from an array of bytes.
    fn set_time_signed_from_bytes(&mut self, bytes: &[u8]){

        let time_signed = (bytes[0] as u64) << 40
                                | (bytes[1] as u64) << 32 
                                | (bytes[2] as u64) << 24 
                                | (bytes[3] as u64) << 16 
                                | (bytes[4] as u64) << 8 
                                | (bytes[5] as u64) << 0;

        self.set_time_signed(time_signed);
    }

    /// Set the fudge attribute from an array of bytes.
    fn set_fudge_from_bytes(&mut self, bytes: &[u8]){
        let fudge = (bytes[0] as u16) << 8 | bytes[1] as u16;

        self.set_fudge(fudge);
    }

    /// Set the mac_size attribute from an array of bytes.
    fn set_mac_size_from_bytes(&mut self, bytes: &[u8]){
        let mac_size = (bytes[0] as u16) << 8 | bytes[1] as u16;

        self.set_mac_size(mac_size);
    }

    /// Set the original_id attribute from an array of bytes.
    fn set_original_id_from_bytes(&mut self, bytes: &[u8]){
        let original_id = (bytes[0] as u16) << 8 | bytes[1] as u16;

        self.set_original_id(original_id);
    }

    /// Set the error attribute from an array of bytes.
    fn set_error_from_bytes(&mut self, bytes: &[u8]){
        let error = (bytes[0] as u16) << 8 | bytes[1] as u16;

        self.set_error(error);
    }

    /// Set the other_len attribute from an array of bytes.
    fn set_other_len_from_bytes(&mut self, bytes: &[u8]){
        let other_len = (bytes[0] as u16) << 8 | bytes[1] as u16;

        self.set_other_len(other_len);
    }
}

/// Getters
impl TSigRdata {

    /// Gets the algorithm_name attribute from TSigRdata
    pub fn get_algorithm_name(&self) -> DomainName {
        self.algorithm_name.clone()
    }

    /// Gets the time_signed attribute from TSigRdata
    pub fn get_time_signed(&self) -> u64 {
        self.time_signed.clone()
    }

    /// Gets the fudge attribute from TSigRdata
    pub fn get_fudge(&self) -> u16 {
        self.fudge.clone()
    }
    
    /// Gets the mac_size attribute from TSigRdata
    pub fn get_mac_size(&self) -> u16 {
        self.mac_size.clone()
    }

    /// Gets the mac attribute from TSigRdata
    pub fn get_mac(&self) -> Vec<u8> {
        self.mac.clone()
    }

    /// Gets the original_id attribute from TSigRdata
    pub fn get_original_id(&self) -> u16 {
        self.original_id.clone()
    }

    /// Gets the error attribute from TSigRdata
    pub fn get_error(&self) -> u16 {
        self.error.clone()
    }

    /// Gets the other_len attribute from TSigRdata
    pub fn get_other_len(&self) -> u16 {
        self.other_len.clone()
    }

    /// Gets the other_data attribute from TSigRdata
    pub fn get_other_data(&self) -> Vec<u8> {
        self.other_data.clone()
    }
}

/// Setters
impl TSigRdata{

    /// Sets the algorithm_name attibute with a value
    pub fn set_algorithm_name(&mut self, algorithm_name: DomainName) {
        self.algorithm_name = algorithm_name;
    }

    /// Sets the time_signed attibute with a value
    pub fn set_time_signed(&mut self, time_signed: u64) {
        self.time_signed = time_signed;
    }

    /// Sets the fudge attibute with a value
    pub fn set_fudge(&mut self, fudge: u16) {
        self.fudge = fudge;
    }

    /// Sets the mac_size attibute with a value
    pub fn set_mac_size(&mut self, mac_size: u16) {
        self.mac_size = mac_size;
    }

    /// Sets the mac attibute with a value
    pub fn set_mac(&mut self, mac: Vec<u8>) {
        self.mac = mac;
    }

    /// Sets the original_id attibute with a value
    pub fn set_original_id(&mut self, original_id: u16) {
        self.original_id = original_id;
    }

    /// Sets the error attibute with a value
    pub fn set_error(&mut self, error: u16) {
        self.error = error;
    }

    /// Sets the other_len attibute with a value
    pub fn set_other_len(&mut self, other_len: u16) {
        self.other_len = other_len;
    }

    /// Sets the other_data attibute with a value
    pub fn set_other_data(&mut self, other_data: Vec<u8>) {
        self.other_data = other_data;
    }
}

impl fmt::Display for TSigRdata {
    /// Formats the record data for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut mac_str = String::new();
        for byte in self.get_mac().iter() {
            mac_str.push_str(&format!("{:X} ", byte));
        }

        let mut other_data_str = String::new();
        for byte in self.get_other_data().iter() {
            other_data_str.push_str(&format!("{:X} ", byte));
        }

        write!(f, "{} {} {} {} {} {} {} {} {}", 
        self.get_algorithm_name().get_name(),
        self.get_time_signed(),
        self.get_fudge(),
        self.get_mac_size(),
        mac_str,
        self.get_original_id(),
        self.get_error(),
        self.get_other_len(),
        other_data_str)
    }
}

#[cfg(test)]
mod tsig_rdata_test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::tsig_rdata::TSigRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test(){
        let tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.algorithm_name.get_name(), String::from(""));
        assert_eq!(tsig_rdata.time_signed, 0);
        assert_eq!(tsig_rdata.fudge, 0);
        assert_eq!(tsig_rdata.mac_size, 0);
        assert_eq!(tsig_rdata.mac, Vec::new());
        assert_eq!(tsig_rdata.original_id, 0);
        assert_eq!(tsig_rdata.error, 0);
        assert_eq!(tsig_rdata.other_len, 0);
        assert_eq!(tsig_rdata.other_data, Vec::new());
    }

    #[test]
    fn set_and_get_algorithm_name(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_algorithm_name().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test_name"));

        tsig_rdata.set_algorithm_name(domain_name);

        assert_eq!(tsig_rdata.get_algorithm_name().get_name(), String::from("test_name"));
    }

    #[test]
    fn set_and_get_time_signed(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_time_signed(), 0);

        tsig_rdata.set_time_signed(123456789);

        assert_eq!(tsig_rdata.get_time_signed(), 123456789);
    }

    #[test]
    fn set_and_get_fudge(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_fudge(), 0);

        tsig_rdata.set_fudge(1234);

        assert_eq!(tsig_rdata.get_fudge(), 1234);
    }

    #[test]
    fn set_and_get_mac_size(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_mac_size(), 0);

        tsig_rdata.set_mac_size(1234);

        assert_eq!(tsig_rdata.get_mac_size(), 1234);
    }

    #[test]
    fn set_and_get_mac(){
        let mac_str = "A1B2C3D4";
        let mac = mac_str.as_bytes().chunks(2)
            .map(|b: &[u8]| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
            .collect::<Vec<u8>>();

        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_mac(), Vec::new());

        tsig_rdata.set_mac(mac.clone());

        assert_eq!(tsig_rdata.get_mac(), mac);
    }

    #[test]
    fn set_and_get_original_id(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_original_id(), 0);

        tsig_rdata.set_original_id(1234);

        assert_eq!(tsig_rdata.get_original_id(), 1234);
    }

    #[test]
    fn set_and_get_error(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_error(), 0);

        tsig_rdata.set_error(1234);

        assert_eq!(tsig_rdata.get_error(), 1234);
    }

    #[test]
    fn set_and_get_other_len(){
        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_other_len(), 0);

        tsig_rdata.set_other_len(1234);

        assert_eq!(tsig_rdata.get_other_len(), 1234);
    }

    #[test]
    fn set_and_get_oher_data(){
        let other_data_str = "A1B2C3D4";
        let other_data = other_data_str.as_bytes().chunks(2)
            .map(|b: &[u8]| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
            .collect::<Vec<u8>>();

        let mut tsig_rdata = TSigRdata::new();

        assert_eq!(tsig_rdata.get_other_data(), Vec::new());

        tsig_rdata.set_other_data(other_data.clone());

        assert_eq!(tsig_rdata.get_other_data(), other_data);
    }

    #[test]
    fn to_bytes_test(){
        let mut tsig_rdata = TSigRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("hmac-md5.sig-alg.reg.int"));
        tsig_rdata.set_algorithm_name(domain_name);
        tsig_rdata.set_time_signed(123456789);
        tsig_rdata.set_fudge(1234);
        tsig_rdata.set_mac_size(4);
        tsig_rdata.set_mac(vec![0xA1, 0xB2, 0xC3, 0xD4]);
        tsig_rdata.set_original_id(1234);
        tsig_rdata.set_error(0);
        tsig_rdata.set_other_len(0);
        tsig_rdata.set_other_data(Vec::new());

        let bytes_to_test = tsig_rdata.to_bytes();

        let bytes = vec![
            //This is the string "hmac-md5.sig-alg.reg.int" in octal, terminated in 00
            0x8, 0x68, 0x6D, 0x61, 0x63, 0x2D, 0x6D, 0x64,
            0x35, 0x7, 0x73, 0x69, 0x67, 0x2D, 0x61, 0x6C, 0x67,
            0x3, 0x72, 0x65, 0x67, 0x3, 0x69, 0x6E, 0x74, 0x0,

            //this is the time signed 123456789 == 0x75bcd15
            0x0, 0x0, 0x7, 0x5B, 0xCD, 0x15,

            // this the fudge 1234
            0x4, 0xD2,

            // this is the macsize = 4
            0x0, 0x4,

            // this is the mac = [0xA1, 0xB2, 0xC3, 0xD4]
            0xA1, 0xB2, 0xC3, 0xD4,

            // this is the original id = 1234
            0x4, 0xD2,

            // this is the error = 0
            0x0, 0x0,

            // this is the other len = 0
            0x0, 0x0

            // No other data, so its empty!
        ];

        for i in 0..bytes.len() {
            assert_eq!(bytes_to_test[i], bytes[i]);
        }
    }

    #[test]
    fn from_bytes_test(){
        let bytes = vec![
            //This is the string "hmac-md5.sig-alg.reg.int" in octal, terminated in 00
            0x8, 0x68, 0x6D, 0x61, 0x63, 0x2D, 0x6D, 0x64,
            0x35, 0x7, 0x73, 0x69, 0x67, 0x2D, 0x61, 0x6C, 0x67,
            0x3, 0x72, 0x65, 0x67, 0x3, 0x69, 0x6E, 0x74, 0x0,

            //this is the time signed 123456789 == 0x75bcd15
            0x0, 0x0, 0x7, 0x5B, 0xCD, 0x15,

            // this the fudge 1234
            0x4, 0xD2,

            // this is the macsize = 4
            0x0, 0x4,

            // this is the mac = [0xA1, 0xB2, 0xC3, 0xD4]
            0xA1, 0xB2, 0xC3, 0xD4,

            // this is the original id = 1234
            0x4, 0xD2,

            // this is the error = 0
            0x0, 0x0,

            // this is the other len = 0
            0x0, 0x0

            // No other data, so its empty!
        ];

        let tsig_rdata_result = TSigRdata::from_bytes(&bytes, &bytes);

        let tsig_rdata = tsig_rdata_result.unwrap();

        assert_eq!(tsig_rdata.get_algorithm_name().get_name(), String::from("hmac-md5.sig-alg.reg.int"));
        assert_eq!(tsig_rdata.get_time_signed(), 123456789);
        assert_eq!(tsig_rdata.get_fudge(), 1234);
        assert_eq!(tsig_rdata.get_mac_size(), 4);
        assert_eq!(tsig_rdata.get_mac(), vec![0xA1, 0xB2, 0xC3, 0xD4]);
        assert_eq!(tsig_rdata.get_original_id(), 1234);
        assert_eq!(tsig_rdata.get_error(), 0);
        assert_eq!(tsig_rdata.get_other_len(), 0);
        assert_eq!(tsig_rdata.get_other_data(), Vec::new());
    }

    #[test]
    fn rr_from_master_file_test(){
        let resource_record = TSigRdata::rr_from_master_file(
        "hmac-md5.sig-alg.reg.int.
        123456789
        1234
        4
        A1B2C3D4
        1234
        0
        0".split_whitespace(),
        56, 
        "IN", 
        String::from("uchile.cl"),
        String::from("uchile.cl"));

        let expected_values = [String::from("hmac-md5.sig-alg.reg.int."), String::from("123456789"),
                                            String::from("1234"), String::from("4"), String::from("A1 B2 C3 D4"),
                                            String::from("1234"),String::from("0"),String::from("0")];
        
        let rdata = resource_record.get_rdata();

        match rdata {
            Rdata::TSIG(val) => assert_eq!([val.get_algorithm_name().get_name(),
            val.get_time_signed().to_string(),
            val.get_fudge().to_string(),
            val.get_mac_size().to_string(),
            val.get_mac().iter().map(|b| format!("{:X}", b)).collect::<Vec<String>>().join(" "),
            val.get_original_id().to_string(),
            val.get_error().to_string(),
            val.get_other_len().to_string()], expected_values),
            _ => {},
        }
    }
}