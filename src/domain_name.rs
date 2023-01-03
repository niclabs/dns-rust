use std::string::String;
use std::fmt;

//utils
use crate::utils::check_label_name;

#[derive(Clone, Default)]

// DNS domain name represented as a sequence of labels, where each label consists of
// a length octet followed by that number of octets.
// The domain name terminates with the zero length octet for the null label of the root.
pub struct DomainName {
    name: String,
}

// Methods
impl DomainName {
    // Creates a new DomainName with default name
    //
    // # Examples
    // ```
    // let domain_name = DomainName::new();
    //
    // assert_eq!(domain_name.name, String::from(""));
    // ```
    //
    pub fn new() -> Self {
        let domain_name: DomainName = DomainName {
            name: String::from(""),
        };
        domain_name
    }

    // Given an array of bytes, creates a new DomainName and returns the unused bytes
    // what happens if label is longer than 9 ? check this out
    pub fn from_bytes_no_offset(bytes: &[u8]) -> String {
        let mut name = String::from("");

        for byte in bytes {
            if *byte <= 9 && *byte >= 1 {
                name.push('.');
            } else if *byte == 0 {
                break;
            } else {
                name.push(*byte as char);
            }
        }

        name.remove(0);

        name
    }

    pub fn from_bytes<'a>(
        bytes: &'a [u8],
        full_msg: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let mut first_byte = bytes[0].clone();
        let mut domain_name_str = "".to_string();
        let mut no_domain_bytes = bytes.clone();

        while first_byte != 0 {
            let bytes_len = no_domain_bytes.len();
            let msg_compresion = first_byte.clone() >> 6;

            if msg_compresion == 3 {
                if bytes_len < 2 {
                    return Err("Format Error");
                }

                let offset: usize = (((no_domain_bytes[0].clone() as u16) << 8
                    | no_domain_bytes[1].clone() as u16)
                    & 0b0011111111111111) as usize;

                let domain_name_result =
                    DomainName::from_bytes(&full_msg[offset..], full_msg.clone());

                match domain_name_result {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                let (domain_label, _bytes) = domain_name_result.unwrap();

                let label = domain_label.get_name();

                domain_name_str.push_str(&label);
                domain_name_str.push_str(".");
                no_domain_bytes = &no_domain_bytes[2..];

                break;
            } else {
                if bytes_len < (first_byte + 1) as usize {
                    return Err("Format Error");
                }

                let label_string =
                    DomainName::from_bytes_no_offset(&no_domain_bytes[..(first_byte + 1) as usize]);

                // Checks label restrictions
                let check_label = check_label_name(label_string.clone());

                if check_label == false {
                    return Err("Format Error");
                }
                //

                domain_name_str.push_str(&label_string);
                domain_name_str.push_str(".");
                no_domain_bytes = &no_domain_bytes[(first_byte + 1) as usize..];

                first_byte = no_domain_bytes[0].clone();
            }
        }

        if first_byte == 0 {
            no_domain_bytes = &no_domain_bytes[1..];
        }

        domain_name_str.remove(domain_name_str.len() - 1);

        // Check domain name restriction, max 255 octets
        let initial_bytes_len = bytes.len();
        let final_bytes_len = no_domain_bytes.len();

        let domain_name_len = initial_bytes_len - final_bytes_len;

        if domain_name_len > 255 {
            return Err("Format Error");
        }
        //

        let mut domain_name = DomainName::new();
        domain_name.set_name(domain_name_str);

        Ok((domain_name, no_domain_bytes))
    }

    // Returns an array of bytes that represents the domain name
    pub fn to_bytes(&self) -> Vec<u8> {
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

    pub fn from_master_file(mut name: String, host_name: String) -> Self {
        let end_dot = name.ends_with(".");

        // Absolute host name
        if end_dot == true {
            // name.remove(name.len() - 1);
            return DomainName { name: name };
        } else {
            // Add the origin host_name
            let mut full_host_name = name.clone();
            full_host_name.push_str(".");
            full_host_name.push_str(&host_name);

            return DomainName {
                name: full_host_name,
            };
        }
    }
}

// Setters Domain Name
impl DomainName {
    // Sets the name attribute with a value
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

// Getters Domain Name
impl DomainName {
    // Gets the name attribute from a DomainName struct
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for DomainName {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let name = self.get_name();
        formatter.write_str(&name)
    }
}

#[cfg(test)]
mod domain_name_test {
    use super::DomainName;

    #[test]
    fn constructor_test() {
        let domain_name = DomainName::new();

        assert_eq!(domain_name.name, String::from(""));
    }

    #[test]
    fn set_and_get_name_test() {
        let mut domain_name = DomainName::new();

        assert_eq!(domain_name.name, String::from(""));

        domain_name.set_name(String::from("test.test2.com."));

        assert_eq!(domain_name.get_name(), String::from("test.test2.com."))
    }

    #[test]
    fn to_bytes_test() {
        let mut domain_name = DomainName::new();
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        domain_name.set_name(String::from("test.test2.com"));

        let bytes = domain_name.to_bytes();

        for (index, byte) in bytes.iter().enumerate() {
            assert_eq!(*byte, bytes_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        let (domain_name, _) = DomainName::from_bytes(&bytes_test, &bytes_test).unwrap();

        println!("{}",domain_name.get_name());
       
        // assert_eq!(domain_name.get_name(), String::from("test.test2.com"));
    }

    #[test]
    fn from_bytes_no_offset_test(){
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];

        let name = DomainName::from_bytes_no_offset(&bytes_test);
        let mut domain_name = DomainName::new();
        domain_name.set_name(name);
        assert_eq!(domain_name.get_name(), String::from("test.test2.com"));

    }

    #[test]
    fn from_master_file_test() {
        let mut name = String::from("poneria.ISI.EDU.");
        let mut hostname = String::from("");

        let mut domain_name = DomainName::from_master_file(name, hostname);
        assert_eq!(domain_name.get_name(), String::from("poneria.ISI.EDU."));

        name = String::from("XX");
        hostname = String::from("LCS.MIT.EDU.");

        domain_name = DomainName::from_master_file(name, hostname);
        assert_eq!(domain_name.get_name(), String::from("XX.LCS.MIT.EDU."));

    }

    #[test]
    fn fmt_test(){
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("XX.LCS.MIT.EDU."));
        assert_eq!(format!("The domain name is: {domain_name}"), "The domain name is: XX.LCS.MIT.EDU.");
    }
}