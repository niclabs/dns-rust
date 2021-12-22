use crate::message::rdata::a_rdata::ARdata;
use crate::message::resource_record::ResourceRecord;
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
    // ELIMINAR HOSTS Y USAR RRS
    hosts: Vec<String>,
    rrs: HashMap<String, Vec<ResourceRecord>>,
}

impl MasterFile {
    /// Creates a new empty master file
    pub fn new(origin: String) -> Self {
        let master_file = MasterFile {
            origin: origin,
            hosts: Vec::new(),
            rrs: HashMap::<String, Vec<ResourceRecord>>::new(),
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

        for line in lines {
            master_file.process_line(line);
        }

        master_file
    }

    fn process_line(&mut self, line: String) {
        // Empty case
        if line == "".to_string() {
            return;
        }

        // ORIGIN case
        if line.contains("$ORIGIN") {
            let mut words = line.split_whitespace();
            words.next();
            self.set_origin(words.next().unwrap().to_string());

            return;
        }

        //Falta Include case

        // Replace @ for the origin domain
        let new_line = line.replace("@", &self.get_origin());

        // Backslash replace
        let line = new_line.replace("\\", "");

        self.process_line_rr(line);

        //println!("{:#?}", line);
    }

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

    fn get_last_host(&self) -> String {
        let hosts = self.get_hosts();

        match hosts.last() {
            Some(x) => x.to_string(),
            None => self.get_origin(),
        }
    }

    fn get_line_host_name(&self, line: String) -> (String, String) {
        let first_char = line.get(0..1).unwrap();
        let mut host_name = "".to_string();
        let mut line_left_to_process = "".to_string();

        if first_char == " ".to_string() {
            host_name = self.get_last_host();
            line_left_to_process = line.clone();
        } else {
            let mut iter = line.split_whitespace();
            host_name = iter.next().unwrap().to_string();
            line_left_to_process = line.get(line.find(" ").unwrap()..).unwrap().to_string();
        }

        return (host_name, line_left_to_process);
    }

    fn add_host(&mut self, host: String) {
        let mut hosts = self.get_hosts();
        hosts.push(host);

        self.set_hosts(hosts);
    }

    fn process_line_rr(&mut self, line: String) {
        // Gets host name
        let (host_name, line_left_to_process) = self.get_line_host_name(line.clone());

        // Process next values
        let mut next_line_items = line.split_whitespace();

        // Default values for rr
        let mut ttl = 0;
        let mut class = "";
        let mut rr_type = "";

        for value in next_line_items {
            let value_type = self.get_value_type(value.to_string());

            if value_type == 0 {
                // TTL
                ttl = value.parse::<u32>().unwrap();
            } else if value_type == 1 {
                // Class
                class = value;
            } else {
                // RRType
                rr_type = value;
                break;
            }
        }

        self.process_especific_rr(
            next_line_items,
            ttl,
            class.to_string(),
            rr_type.to_string(),
            host_name,
        );
    }

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

    fn process_especific_rr(
        &mut self,
        items: SplitWhitespace,
        ttl: u32,
        class: String,
        rr_type: String,
        host_name: String,
    ) {
        let resource_record = match rr_type.as_str() {
            "A" => ARdata::rr_from_master_file(items, ttl, class, host_name.clone()),
            "NS" => 2,
            "CNAME" => 2,
            "SOA" => 2,
            "PTR" => 2,
            "HINFO" => 2,
            "MX" => 2,
            "TXT" => 2,
        };

        self.add_rr(host_name, resource_record);
    }

    fn add_rr(&mut self, host_name: String, resource_record: ResourceRecord) {
        let rrs = self.get_rrs();

        let mut rrs_vec = match rrs.get(&host_name) {
            Some(val) => val,
            None => &Vec::<ResourceRecord>::new(),
        };

        rrs_vec.push(resource_record);

        rrs.insert(host_name, rrs_vec.to_vec());
    }

    /*
    fn process_backslashs(&mut self, line: String) {
        // is there backslash?
        let index = match line.find("\\") {
            Some(val) => val,
            None => -1,
        };

        if index == -1 {
            return line;
        }

        let next_char_to_backslash = line.get(index + 1..index + 2);

        let parse_to_numb = next_char_to_backslash.parse::<f64>();

        let is_numb = match parse_to_numb {
            Ok(ok) => 1,
            Err(e) => 0,
        };

        if is_numb == 1 {
            let oct_number_str = line.get(index + 1..index + 4);
            let oct_number = oct_number_str.parse::<u32>().unwrap();
        }

        line.replace("\\", "");

        return line;
    }
    */
}

// Getters
impl MasterFile {
    // Gets the origin name
    pub fn get_origin(&self) -> String {
        self.origin.clone()
    }

    // Gets the hosts
    pub fn get_hosts(&self) -> Vec<String> {
        self.hosts.clone()
    }

    // Gets the resource records
    pub fn get_rrs(&self) -> HashMap<String, Vec<ResourceRecord>> {
        self.rrs.clone()
    }
}

// Setters
impl MasterFile {
    // Sets the origin with a new value
    pub fn set_origin(&mut self, origin: String) {
        self.origin = origin;
    }

    // Sets the hosts with a new value
    pub fn set_hosts(&mut self, hosts: Vec<String>) {
        self.hosts = hosts;
    }

    // Sets the rrs with a new value
    pub fn set_rrs(&mut self, rrs: HashMap<String, Vec<ResourceRecord>>) {
        self.rrs = rrs;
    }
}
