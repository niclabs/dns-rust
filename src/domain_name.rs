use std::fmt;
use std::string::String;

#[derive(Clone, Default, PartialEq, Debug, Hash, PartialOrd, Ord, Eq)]

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

    pub fn new_from_string(domain_name: String) -> Self {
        let domain_name: DomainName = DomainName { 
            name: domain_name.clone()
        };

        domain_name
    }

    pub fn new_from_str(domain_name: &str) -> Self {
        Self::new_from_string(domain_name.to_string())
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
        let mut no_domain_bytes = bytes;
       
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
                    DomainName::from_bytes(&full_msg[offset..], full_msg);

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
            } 
            else {
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
            // It means that the domain name is root
            no_domain_bytes = &no_domain_bytes[1..];
        }

        if domain_name_str.len() > 0{
            //remove last value 0
            domain_name_str.remove(domain_name_str.len() - 1);
        }

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
            // If the name is root or empty break the loop
            if name == "." || name == ""{
                break;
            }
            let word_length = word.len();
            bytes.push(word_length as u8);

            for character in word.chars() {
                bytes.push(character as u8);
            }
        }

        bytes.push(0 as u8);

        bytes
    }

    pub fn from_master_file(name: String, host_name: String) -> Self {
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

pub fn check_label_name(name: String) -> bool {
    if name.len() > 63 || name.len() == 0 {
        return false;
    }

    for (i, c) in name.chars().enumerate() {
        if i == 0 && !c.is_ascii_alphabetic() {
            return false;
        } else if i == name.len() - 1 && !c.is_ascii_alphanumeric() {
            return false;
        } else if !(c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    return true;
}

// validity checks should be performed insuring that the file is syntactically correct
pub fn domain_validity_syntax(domain_name: DomainName) -> Result<DomainName, &'static str> {
    let domain_name_string = domain_name.get_name();
    if domain_name_string.eq("@") {
        return Ok(domain_name);
    }
    let mut empty_label = false;
    for label in domain_name_string.split(".") {
        if empty_label {
            return Err("Error: Empty label is only allowed at the end of a hostname.");
        }
        if label.is_empty() {
            empty_label = true;
            continue;
        }
        if !check_label_name(label.to_string()) {
            println!("L: {}", label);
            return Err("Error: present domain name is not syntactically correct.");
        }
    }
    return Ok(domain_name);
}


#[cfg(test)]
mod domain_name_test {
    use super::DomainName;
    use super::check_label_name;
    use super::domain_validity_syntax;

    #[test]
    fn constructor_test() {
        let domain_name = DomainName::new();

        assert_eq!(domain_name.name, String::from(""));
    }

    #[test]
    fn new_from_str_test() {
        let domain_name = DomainName::new_from_str("example.com");
        assert_eq!(domain_name.name, String::from("example.com"));
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
        let bytes_test: Vec<u8> = vec![1, 70, 3, 73, 83, 73, 4, 65, 82, 80, 65, 0, 3, 70, 79, 79, 192, 0, 4, 65, 82, 80, 65, 0, 0];
        let (domain_name, no_domain_bytes) = DomainName::from_bytes(&bytes_test, &bytes_test).unwrap();

        let (new_domain_name, _) = DomainName::from_bytes(&no_domain_bytes, &bytes_test).unwrap();

        assert_eq!(domain_name.get_name(), String::from("F.ISI.ARPA"));

        assert_eq!(new_domain_name.get_name(), String::from("FOO.F.ISI.ARPA"));
    }

    #[test]
    fn from_bytes_no_offset_test() {
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
    fn fmt_test() {
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("XX.LCS.MIT.EDU."));
        assert_eq!(
            format!("The domain name is: {domain_name}"),
            "The domain name is: XX.LCS.MIT.EDU."
        );
    }

    #[test]
    #[ignore = "the domain name should be the root"]
    fn root_domain_test(){
        let domain_name = DomainName::new_from_str(".");
        let bytes = domain_name.to_bytes();
        assert_eq!(bytes, vec![0]);
        let new_domain_name = DomainName::from_bytes(&bytes, &bytes).unwrap();
        assert_eq!(new_domain_name.0.get_name(), String::from(".") );
    }

    #[test]
    fn check_label_name_empty_label() {
        let cln_empty_str = check_label_name(String::from(""));
        assert_eq!(cln_empty_str, false);
    }

    #[test]
    fn check_label_name_large_label() {
        let cln_large_str = check_label_name(String::from(
            "this-is-a-extremely-large-label-that-have-exactly--64-characters",
        ));
        assert_eq!(cln_large_str, false);
    }

    #[test]
    fn check_label_name_first_label_character() {
        let cln_symbol_str = check_label_name(String::from("-label"));
        assert_eq!(cln_symbol_str, false);

        let cln_num_str = check_label_name(String::from("0label"));
        assert_eq!(cln_num_str, false);
    }

    #[test]
    fn check_label_name_last_label_character() {
        let cln_symbol_str = check_label_name(String::from("label-"));
        assert_eq!(cln_symbol_str, false);

        let cln_num_str = check_label_name(String::from("label2"));
        assert_eq!(cln_num_str, true);
    }

    #[test]
    fn check_label_name_interior_label_characters() {
        let cln_dot_str = check_label_name(String::from("label.test"));
        assert_eq!(cln_dot_str, false);

        let cln_space_str = check_label_name(String::from("label test"));
        assert_eq!(cln_space_str, false);
    }

    #[test]
    fn check_label_name_valid_label() {
        let cln_valid_str = check_label_name(String::from("label0test"));
        assert_eq!(cln_valid_str, true);
    }

    #[test]
    fn domain_validity_syntax_empty_dom() {
        let mut expected_domain_name = DomainName::new();
        expected_domain_name.set_name(String::from(""));
        let ok = Ok(expected_domain_name.clone());
        let mut domain_name = DomainName::new();
        let empty_dom = String::from("");
        domain_name.set_name(empty_dom);

        let empty_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(empty_dom_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_valid_dom() {
        let mut expected_domain_name = DomainName::new();
        expected_domain_name.set_name(String::from("label1.label2."));
        let ok = Ok(expected_domain_name);
        let mut domain_name = DomainName::new();
        let valid_dom = String::from("label1.label2.");
        domain_name.set_name(valid_dom);

        let valid_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(valid_dom_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_wrong_middle_dom() {
        let mut domain_name = DomainName::new();
        let wrong_middle_dom = String::from("label1..label2");
        domain_name.set_name(wrong_middle_dom.clone());
        let wrong_middle_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(
            wrong_middle_dom_validity,
            Err("Error: Empty label is only allowed at the end of a hostname.")
        );
    }

    #[test]
    fn domain_validity_syntax_wrong_init_dom() {
        let mut domain_name = DomainName::new();
        let wrong_init_dom = String::from(".label");
        domain_name.set_name(wrong_init_dom);
        let wrong_init_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(
            wrong_init_dom_validity,
            Err("Error: Empty label is only allowed at the end of a hostname.")
        );
    }

    #[test]
    fn domain_validity_syntax_at_domain_name() {
        let mut domain_name = DomainName::new();
        let at_str = String::from("@");
        domain_name.set_name(at_str.clone());
        let ok = Ok(domain_name.clone());
        let at_str_validity = domain_validity_syntax(domain_name);

        assert_eq!(at_str_validity, ok);
    }

    #[test]
    fn domain_validity_syntax_syntactically_incorrect_dom() {
        let mut domain_name = DomainName::new();
        let incorrect_dom = String::from("label1.2badlabel.test");
        domain_name.set_name(incorrect_dom.clone());
        let incorrect_dom_validity = domain_validity_syntax(domain_name);

        assert_eq!(
            incorrect_dom_validity,
            Err("Error: present domain name is not syntactically correct.")
        );
    }

    #[test]
    fn domain_validity_syntax_syntactically_correct_dom() {
        let mut domain_name_1 = DomainName::new();
        let correct_dom_1 = String::from("label1.label2.test");
        domain_name_1.set_name(correct_dom_1.clone());

        let mut domain_name_2 = DomainName::new();
        let correct_dom_2 = String::from("label1.label2.test.");
        domain_name_2.set_name(correct_dom_2.clone());

        let ok_dom_1 = Ok(domain_name_1.clone());
        let ok_dom_2 = Ok(domain_name_2.clone());
        let correct_dom_1_validity = domain_validity_syntax(domain_name_1);
        let correct_dom_2_validity = domain_validity_syntax(domain_name_2);

        assert_eq!(correct_dom_1_validity, ok_dom_1);
        assert_eq!(correct_dom_2_validity, ok_dom_2);
    }
}
