use crate::message::rdata::a_ch_rdata::AChRdata;
use crate::message::rdata::a_rdata::ARdata;
use crate::message::rdata::cname_rdata::CnameRdata;
use crate::message::rdata::hinfo_rdata::HinfoRdata;
use crate::message::rdata::mx_rdata::MxRdata;
use crate::message::rdata::ns_rdata::NsRdata;
use crate::message::rdata::ptr_rdata::PtrRdata;
use crate::message::rdata::soa_rdata::SoaRdata;
use crate::message::rdata::txt_rdata::TxtRdata;
use crate::message::resource_record::ResourceRecord;
//refactor
use crate::name_server::zone::NSZone;
use core::num;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::SplitWhitespace;

#[derive(Clone)]
/// Structs that represents data from a master file
pub struct MasterFile {
    origin: String,
    last_host: String,
    rrs: HashMap<String, Vec<ResourceRecord>>,
    class_default: String,
    ttl_default: u32,
}


// validity checks should be performed insuring that the file is syntactically correct
fn domain_validity_syntax(domain_name: String)-> Result<String, &'static str> {
    for label in domain_name.split("."){
        if ! NSZone::check_label_name(label.to_string()) {
            return Err("Error: present domain name is not syntactically correct.");
        }
    }
    return Ok(domain_name);
}

impl MasterFile {
    /// Creates a new empty master file
    pub fn new(origin: String) -> Self {
        let master_file = MasterFile {
            origin: origin,
            last_host: "".to_string(),
            rrs: HashMap::<String, Vec<ResourceRecord>>::new(),
            class_default: "IN".to_string(),
            ttl_default: 0,
        };

        master_file
    }

    /// Creates a new master file from the parameter filename
    pub fn from_file(filename: String) -> Self {
        let file = File::open(filename).expect("file not found!");
        let reader = BufReader::new(file);

        let mut master_file = MasterFile::new("".to_string());

        let mut lines: Vec<String> = Vec::new();
        let mut last_line = "".to_string();

        // Link lines with parenthesis and remove comments
        for line in reader.lines() {
            let line = line.unwrap();

            // Remove comments
            let line_without_comments = MasterFile::remove_comments(line.clone());

            let open_parenthesis = match line_without_comments.clone().find("(") {
                Some(x) => 1,
                None => 0,
            };

            let closed_parenthesis = match line_without_comments.clone().find(")") {
                Some(x) => 1,
                None => 0,
            };

            if open_parenthesis == 1 && closed_parenthesis == 0 {
                last_line.push_str(&line_without_comments);
                continue;
                //
            } else if open_parenthesis == 0
                && closed_parenthesis == 0
                && last_line != "".to_string()
            {
                last_line.push_str(&line_without_comments);
                continue;
                //
            } else {
                last_line.push_str(&line_without_comments);
                lines.push(last_line.replace("(", "").replace(")", ""));
                last_line = "".to_string();
            }
        }

        master_file.process_lines_and_validation();

        master_file
    }
        
    /// Removes the comments from a line in a master file
    fn remove_comments(mut line: String) -> String {
        let index = line.find(";");

        let there_are_comments = match index {
            Some(x) => 1,
            None => 0,
        };

        if there_are_comments == 1 {
            line.replace_range(index.unwrap().., "");
        }

        return line;
    }

    fn replace_special_encoding(mut line: String) -> String {

        let index = match line.find("\\") {
            Some(val) => val,
            None => usize::MAX, 
        };

        if index == usize::MAX {
            return line; 
        }

        let next_char_to_backslash = line.get(index + 1..index + 2).unwrap().to_string();

        /*
            \DDD where each D is a digit is the octet corresponding to
            the decimal number described by DDD. The resulting
            octet is assumed to be text and is not checked for
            special meaning.
        */
        if next_char_to_backslash >= "0".to_string() &&  next_char_to_backslash <= "9".to_string(){
            let oct_number_str = line.get(index + 1..index + 4).unwrap();
            let oct_number = oct_number_str.parse::<u32>().unwrap();
            let dec_str = oct_number.to_string();
            line.replace_range(index..index+4, &dec_str);
        }

        /*
            \X where X is any character other than a digit (0-9), is
            used to quote that character so that its special meaning
            does not apply. For example, "\." can be used to place
            a dot character in a label.
        */
        else {
            let x = next_char_to_backslash.to_string(); 
            line.replace_range(index..index+2, &x);
        }
        return line;
    }

    /// Gets the hostname of a line in a master file. If there is no hostname, takes the last hostnames used.
    fn get_line_host_name(&mut self, line: String) -> (String, String) {
        let first_char = line.get(0..1).unwrap();
        let mut host_name = "".to_string();
        let mut line_left_to_process = "".to_string();

        if first_char == " ".to_string() {
            host_name = self.get_last_host();
            line_left_to_process = line.clone();
        } else {
            let mut iter = line.split_whitespace();
            host_name = iter.next().unwrap().to_string();

            self.set_last_host(host_name.clone());
            line_left_to_process = line.get(line.find(" ").unwrap()..).unwrap().to_string();
        }
        let valid_host_name = domain_validity_syntax(host_name).unwrap();
        return (valid_host_name, line_left_to_process);
    }

    /// Returns whether the type is class, rr_type or ttl
    fn get_value_type(&self, value: String) -> u8 {
        match value.as_str() {
            "IN" => 1,
            "CS" => 1,
            "CH" => 1,
            "HS" => 1,
            "A" => 2,
            "NS" => 2,
            "CNAME" => 2,
            "SOA" => 2,
            "PTR" => 2,
            "HINFO" => 2,
            "MX" => 2,
            "TXT" => 2,
            _ => 0,
        }
    }

    /// Process an especific type of RR
    fn process_specific_rr(
        &mut self,
        items: SplitWhitespace,
        ttl: u32,
        class: String,
        rr_type: String,
        host_name: String,
    ) {
        let origin = domain_validity_syntax(self.get_origin()).unwrap();
        let valid_host_name = domain_validity_syntax(host_name.clone()).unwrap();
        let mut full_host_name = valid_host_name.clone();

        if valid_host_name.ends_with(".") == false {
            full_host_name.push_str(".");
            full_host_name.push_str(&origin);
        }

        let class_int = match class.as_str() {
            "IN" => 1,
            "CS" => 2,
            "CH" => 3,
            "HS" => 4,
            _ => unreachable!(),
        };

        let resource_record = match rr_type.as_str() {
            "A" => {
                if class_int == 3 {
                    AChRdata::rr_from_master_file(
                        items,
                        ttl,
                        class_int,
                        full_host_name.clone(),
                        origin.clone(),
                    )
                } else {
                    ARdata::rr_from_master_file(items, ttl, class_int, full_host_name.clone())
                }
            }
            "NS" => NsRdata::rr_from_master_file(
                items,
                ttl,
                class_int,
                full_host_name.clone(),
                origin.clone(),
            ),
            "CNAME" => CnameRdata::rr_from_master_file(
                items,
                ttl,
                class_int,
                full_host_name.clone(),
                origin.clone(),
            ),
            "SOA" => {
                self.set_class_default(class.clone());
                let (rr, minimum) = SoaRdata::rr_from_master_file(
                    items,
                    ttl,
                    class_int,
                    full_host_name.clone(),
                    origin.clone(),
                );
                self.set_ttl_default(minimum);
                rr
            }
            "PTR" => PtrRdata::rr_from_master_file(
                items,
                ttl,
                class_int,
                full_host_name.clone(),
                origin.clone(),
            ),
            "HINFO" => {
                HinfoRdata::rr_from_master_file(items, ttl, class_int, full_host_name.clone())
            }
            "MX" => MxRdata::rr_from_master_file(
                items,
                ttl,
                class_int,
                full_host_name.clone(),
                origin.clone(),
            ),
            "TXT" => TxtRdata::rr_from_master_file(items, ttl, class_int, full_host_name.clone()),
            _ => unreachable!(),
        };

        self.add_rr(valid_host_name, resource_record);
    }

    /// Adds a new rr to the master file parsings
    fn add_rr(&mut self, host_name: String, resource_record: ResourceRecord) {
        let mut rrs = self.get_rrs();
        let valid_host_name = domain_validity_syntax(host_name).unwrap();
        let mut rrs_vec = match rrs.get(&valid_host_name) {
            Some(val) => val.clone(),
            None => Vec::<ResourceRecord>::new(),
        };

        rrs_vec.push(resource_record);

        rrs.insert(valid_host_name, rrs_vec.to_vec());

        self.set_rrs(rrs);
    }

    fn process_line_rr(&mut self, line: String) -> (String, String) {
        // Gets host name
        let (mut host_name, line_left_to_process) = self.get_line_host_name(line.clone());
        host_name = domain_validity_syntax(host_name).unwrap();

        // Process next values
        let mut next_line_items = line_left_to_process.split_whitespace();

        // Default values for rr
        let mut ttl = self.get_ttl_default();
        let mut class = self.get_class_default();
        let mut rr_type = "";

        let mut value = match next_line_items.next() {
            Some(val) => val,
            None => "",
        };

        while value != "" {
            let value_type = self.get_value_type(value.to_string());

            println!("Name: {}, value: {}", host_name.clone(), value_type);

            if value_type == 0 {
                // TTL
                ttl = value.parse::<u32>().unwrap();

                value = match next_line_items.next() {
                    Some(val) => val,
                    None => "",
                };
            } else if value_type == 1 {
                // Class
                class = value.to_string();

                value = match next_line_items.next() {
                    Some(val) => val,
                    None => "",
                };
            } else {
                // RRType
                rr_type = value;
                break;
            }
        }
        let (this_class, this_type) = (class.to_string(), rr_type.to_string());
        self.process_specific_rr(next_line_items, ttl, class, rr_type.to_string(), host_name);
        return (this_class, this_type);
    }

    fn process_include(&mut self, file_name: String, mut domain_name: String) -> Result<bool, &'static str> {
        let mut prev_class = "".to_string();
        domain_name = domain_validity_syntax(domain_name).unwrap();
        if domain_name != "" {
            self.set_last_host(domain_name);
        }

        let file = File::open(file_name).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let (class, rr_type) = self.process_line_rr(line);

            if prev_class == "".to_string(){
                prev_class = class;
                if rr_type != "SOA".to_string(){
                    return Err("Error: no SOA RR is present at the top of the zone."); 
                }
            }

            else{
                if class != prev_class{
                    return Err("Error: not all rr have the same class.");
                }
                if rr_type == "SOA".to_string(){
                    return Err("Error: more than one SOA per zone.");
                }
            }
        }
        return Ok(true);
    }
        //all rr should have same class
        //Exactly one SOA RR should be present at the top of the zone
        //If delegations are present and glue information is required,it should be present.
        /*
        Information present outside of the authoritative nodes in the
        zone should be glue information, rather than the result of an
        origin or similar error.
        */
    //------------------
    fn process_lines_and_validation(&mut self) -> Result<bool, &'static str> {
        let mut lines: Vec<String> = Vec::new();
        let mut last_line = "".to_string();
        
        let mut prev_rr_class = "".to_string();

        for line_with_comments in lines {

            let mut line = MasterFile::remove_comments(line_with_comments.clone());

            if line == "".to_string() {
                continue;
            }

            if line.contains("$ORIGIN") {
                let mut words = line.split_whitespace();
                words.next();
                let mut name = words.next().unwrap().to_string();
                name = domain_validity_syntax(name).unwrap();

                self.set_last_host(name.clone());
                self.set_origin(name);
                continue;
            }

            if line.contains("$INCLUDE") {
                let mut words = line.split_whitespace();
                words.next();

                let file_name = words.next().unwrap();
                let mut domain_name = words.next().unwrap_or("");
                let valid_domain_name = domain_validity_syntax(domain_name.to_string()).unwrap();
                return self.process_include(file_name.to_string(), valid_domain_name);
            }

            let contains_non_especial_at_sign = line.contains("\\@");
            
            if contains_non_especial_at_sign == false {
                let new_line = line.replace("@", &self.get_origin());
            }

            let new_line = line.replace("\\@", "@");
            let line = new_line.replace("\\", "");

            let (class, rr_type) = self.process_line_rr(line);
            
            if prev_rr_class == "".to_string(){
                prev_rr_class = class; 
                if rr_type != "SOA".to_string(){
                    return Err("Error: no SOA RR is present at the top of the zone.");
                }
            }
            else{
                if class != prev_rr_class{
                    return Err("Error: not all rr have the same class.");
                }
                if rr_type == "SOA".to_string(){
                    return Err("Error: more than one SOA per zone.");
                }
            }   
        }
        return Ok(true);
    }
}

// Getters
impl MasterFile {
    // Gets the origin name
    pub fn get_origin(&self) -> String {
        self.origin.clone()
    }

    // Gets the resource records
    pub fn get_rrs(&self) -> HashMap<String, Vec<ResourceRecord>> {
        self.rrs.clone()
    }

    // Gets the last host used
    pub fn get_last_host(&self) -> String {
        self.last_host.clone()
    }

    // Gets the default class for RR's
    pub fn get_class_default(&self) -> String {
        self.class_default.clone()
    }

    // Gets the default Ttl for RR's
    pub fn get_ttl_default(&self) -> u32 {
        self.ttl_default
    }
}

// Setters
impl MasterFile {
    // Sets the origin with a new value
    pub fn set_origin(&mut self, origin: String) {
        self.origin = origin;
    }

    // Sets the rrs with a new value
    pub fn set_rrs(&mut self, rrs: HashMap<String, Vec<ResourceRecord>>) {
        self.rrs = rrs;
    }

    // Sets the last host used
    pub fn set_last_host(&mut self, last_host: String) {
        self.last_host = last_host;
    }

    // Sets the default class for RR's
    pub fn set_class_default(&mut self, class: String) {
        self.class_default = class;
    }

    // Sets the default Ttl for RR's
    pub fn set_ttl_default(&mut self, ttl: u32) {
        self.ttl_default = ttl;
    }
}
