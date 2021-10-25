use crate::resource_record::{FromBytes, ToBytes};
use std::string::String;

#[derive(Clone)]
/// An struct that represents the rdata for hinfo type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                  CPU                          |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                   OS                          |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///

pub struct HinfoRdata {
    // Specifies the CPU type.
    cpu: String,
    // Specifies the operating system type.
    os: String,
}

impl ToBytes for HinfoRdata {
    /// Return a vec of bytes that represents the hinfo rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut cpu = self.get_cpu();
        let mut os = self.get_os();

        for _character_index in 0..cpu.len() {
            let character_to_byte = cpu.remove(0) as u8;
            bytes.push(character_to_byte);
        }

        bytes.push(0 as u8);

        for _character_index in 0..os.len() {
            let character_to_byte = os.remove(0) as u8;
            bytes.push(character_to_byte);
        }

        bytes.push(0 as u8);

        bytes
    }

}

impl FromBytes<HinfoRdata> for HinfoRdata {
    /// Creates a new HinfoRdata from an array of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut cpu = String::from("");
        let mut os = String::from("");

        let mut string_num = 0;
        
        for byte in bytes {
            if *byte == 0{
                if string_num == 0{
                    string_num +=1;
                }
                else{
                    break; 
                }
            } 
            else if string_num == 0{
                cpu.push(*byte as char); 
            }
            else if string_num > 0{
                os.push(*byte as char);
            }
        }

        let mut hinfo_rdata = HinfoRdata::new();
        hinfo_rdata.set_cpu(cpu);
        hinfo_rdata.set_os(os); 

        hinfo_rdata
    }
}            

impl HinfoRdata {
    /// Creates a new HinfoRdata with default values.
    ///
    /// # Examples
    /// ```
    /// let hinfo_rdata = HinfoRdata::new();
    ///
    /// assert_eq!(hinfo_rdata.cpu, String::from(""));
    /// assert_eq!(hinfo_rdata.os, String::from(""));
    /// ```
    ///

    pub fn new() -> Self {
        let hinfo_rdata = HinfoRdata { 
            cpu: String::new(),
            os: String::new(),
        };
        hinfo_rdata
    }
}

// Getters
impl HinfoRdata {
    // Gets the cpu attribute
    pub fn get_cpu(&self) -> String {
        self.cpu.clone()
    }
    // Gets the os attribute
    pub fn get_os(&self) -> String {
        self.os.clone()
    }
}

// Setters
impl HinfoRdata {
    // Sets the cpu field with a value
    pub fn set_cpu(&mut self, cpu: String) {
        self.cpu = cpu;
    }
    // Sets the os field with a value
    pub fn set_os(&mut self, os: String) {
        self.os = os;
    }
}

mod test {
    use crate::rdata::hinfo_rdata::HinfoRdata;
    use crate::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let hinfo_rdata = HinfoRdata::new();

        assert_eq!(hinfo_rdata.cpu, String::from(""));
        assert_eq!(hinfo_rdata.os, String::from(""));
    }

    #[test]
    fn set_and_get_cpu_test() {
        let mut hinfo_rdata = HinfoRdata::new();

        assert_eq!(hinfo_rdata.get_cpu(), String::from(""));

        hinfo_rdata.set_cpu(String::from("test"));

        assert_eq!(hinfo_rdata.get_cpu(), String::from("test"));
    }

    #[test]
    fn set_and_get_os_test() {
        let mut hinfo_rdata = HinfoRdata::new();

        assert_eq!(hinfo_rdata.get_os(), String::from(""));

        hinfo_rdata.set_os(String::from("test"));

        assert_eq!(hinfo_rdata.get_os(), String::from("test"));
    }

    #[test]
    fn to_bytes_test() {
        let mut hinfo_rdata = HinfoRdata::new();

        hinfo_rdata.set_cpu(String::from("cpu"));
        hinfo_rdata.set_os(String::from("os"));

        let bytes_to_test: [u8; 7] = [99, 112, 117, 0, 111, 115, 0];
        let hinfo_rdata_to_bytes = hinfo_rdata.to_bytes();

        for (index, value) in hinfo_rdata_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 7] = [99, 112, 117, 0, 111, 115, 0];

        let hinfo_rdata = HinfoRdata::from_bytes(&bytes);

        assert_eq!(hinfo_rdata.get_cpu(), String::from("cpu"));
        assert_eq!(hinfo_rdata.get_os(), String::from("os"));
    }
}