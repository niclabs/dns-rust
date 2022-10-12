use crate::message::rdata::Rdata;
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
use crate::NameServer;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::SplitWhitespace;

//utils
use crate::utils::domain_validity_syntax;

#[derive(Clone)]
/// Structs that represents data from a master file
pub struct MasterFile {
    origin: String,
    last_host: String,
    rrs: HashMap<String, Vec<ResourceRecord>>,
    class_default: String,
    ttl_default: u32,
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

    // Creates a new master file from the parameter filename. For listing cache contents.
    // Set validation to true if checking validity syntax of the master file is desired.
    pub fn from_file(filename: String, validation: bool) -> Self {
        let file = File::open(filename).expect("file not found!");
        let reader = BufReader::new(file);

        let mut master_file = MasterFile::new("".to_string());

        let mut lines: Vec<String> = Vec::new();
        let mut last_line = "".to_string();

        // Link lines with parenthesis and remove comments
        for line in reader.lines() {
            let line = line.unwrap();

            // Remove comments and replace especial encoding
            let line_without_comments = MasterFile::replace_special_encoding(MasterFile::remove_comments(line.clone()).clone());
            
            let open_parenthesis = match line_without_comments.clone().find("(") {
                Some(_) => 1,
                None => 0,
            };

            let closed_parenthesis = match line_without_comments.clone().find(")") {
                Some(_) => 1,
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

        if validation {
            println!("Creating new Masterfile with validation.");
            println!("Starting validation...");
            master_file.process_lines_and_validation(lines);
            // validate presence of glue records when necessary
            master_file.check_glue_delegations();
            // look for cname loops 
            master_file.check_cname_loop();
            println!("Masterfile validated correctly.");
        }

        else {
            println!("Creating new Masterfile with no validation.");
            for line in lines {
                master_file.process_line(line);
            }
        }

        master_file
    }

  // Process a single line from a master file or include file 
    fn process_line(&mut self, line: String) {
        // Empty case
        if line == "".to_string() {
            return;
        }

        // ORIGIN case
        if line.contains("$ORIGIN") {
            let mut words = line.split_whitespace();
            words.next();
            let name = words.next().unwrap().to_string();
            self.set_last_host(name.clone());
            self.set_origin(name);

            return;
        }

        //Include case
        if line.contains("$INCLUDE") {
            let mut words = line.split_whitespace();
            words.next();

            let file_name = words.next().unwrap();
            let domain_name = words.next().unwrap_or("");
            self.process_include(file_name.to_string(), domain_name.to_string(), false);

            return;
        }

        // Replace @ for the origin domain
        let contains_non_especial_at_sign = line.contains("@");

        let mut new_line = line.clone();
        if contains_non_especial_at_sign {
            let mut full_origin = self.get_origin();
            full_origin.push_str(".");
            new_line = line.replace("@", &full_origin);
        }

        self.process_line_rr(new_line, false);
    }

    fn process_line_rr(&mut self, line: String, validity: bool) -> (String, String) {
        // Gets host name
        let (mut host_name, line_left_to_process) = self.get_line_host_name(line.clone());
        
        if validity {
            host_name = domain_validity_syntax(host_name).unwrap();
        }
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
        self.process_specific_rr(next_line_items, ttl, class, rr_type.to_string(), host_name, validity);
        return (this_class, this_type);
    }

    /*  Checks all the lines in the masterfile.    
        Looks for $ORIGIN control entries, changing the current origin for relative
        domain names to the stated name.
        Looks for $INCLUDE control entries, inserting the named file into
        the current file.
        Ensures there is only one SOA rr, and that it is the first rr in the masterfile.
        Ensures the remaining rr in the masterfile belongs to the same class (not SOA).
    */
    fn process_lines_and_validation(&mut self, lines: Vec<String>){
        
        let mut prev_rr_class = "".to_string();

        for line in lines {

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
                let domain_name = words.next().unwrap_or("");
                let valid_domain_name = domain_validity_syntax(domain_name.to_string()).unwrap();
                return self.process_include(file_name.to_string(), valid_domain_name, true);
            }
            
            let contains_non_especial_at_sign = line.contains("@");

            let mut new_line = line.clone();
            if contains_non_especial_at_sign {
                let mut full_origin = self.get_origin();
                full_origin.push_str(".");
                new_line = line.replace("@", &full_origin);
            }

            let (class, rr_type) = self.process_line_rr(new_line, true);
            
            if prev_rr_class == "".to_string(){
                prev_rr_class = class; 
                if rr_type != "SOA".to_string(){
                    panic!("No SOA RR is present at the top of the zone.");
                }
            }
            else{
                if class != prev_rr_class{
                    panic!("Not all rr have the same class.");
                }
                
                if rr_type == "SOA".to_string(){
                    panic!("More than one SOA per zone.");
                }
            }   
        }
    }

    // detect cname loops of type 1->2->1:
    /* example of CNAME loop with two CNAMEs 1 -> 2 -> 1 -> 2 -> 1, etc.
        alias1.example.org. 3600 CNAME alias2.example.org.
        alias2.example.org. 3600 CNAME alias1.example.org.
    */
    fn check_cname_loop(&self){
        
        let rrs = self.get_rrs();
        let mut cname_rrs = HashMap::<String, Vec<ResourceRecord>>::new();

        // only cnames
        for (hostname, host_rrs) in rrs {
            let mut cname_by_host = Vec::<ResourceRecord>::new();
            for host_rr in host_rrs {
                if host_rr.get_type_code() == 5 {
                    cname_by_host.push(host_rr);
                }
            }

            if cname_by_host.len()>0 {
                cname_rrs.insert(hostname.to_string(), cname_by_host);
            }
        }
        
        for (alias, canonical) in &cname_rrs {
            let rdata = canonical[0].get_rdata(); 
            let canonical_name = match rdata{
                Rdata::SomeCnameRdata(val) => val.get_cname().get_name(), 
                _ => unreachable!(), 
            };
            match cname_rrs.get(&canonical_name.to_string()) {
                Some(val) => { 
                    match val[0].get_rdata() {
                        Rdata::SomeCnameRdata(crr) => { 
                            if crr.get_cname().get_name().to_string() == alias.to_string() {
                                panic!("CNAME loop detected!"); 
                            }
                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                None => { 
                    continue;
                }
            }; 
        }
    }
    
    /// Process an specific type of RR
    fn process_specific_rr(
        &mut self,
        items: SplitWhitespace,
        ttl: u32,
        class: String,
        rr_type: String,
        host_name: String,
        validity: bool,
    ) {

        let mut origin = self.get_origin();
        let mut full_host_name = host_name.clone();

        if validity{
            origin = domain_validity_syntax(self.get_origin()).unwrap();
            let valid_host_name = domain_validity_syntax(host_name.clone()).unwrap();
            full_host_name = valid_host_name.clone();
        }
        

        if host_name.ends_with(".") == false {
            full_host_name.push_str(".");
            full_host_name.push_str(&origin);
        }

        // remove last "." from hostname 
        else {
            full_host_name.pop();
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

        self.add_rr(full_host_name, resource_record);
    }

    // Removes the comments from a line in a master file
    fn remove_comments(mut line: String) -> String {
        let index = line.find(";");

        let there_are_comments = match index {
            Some(_) => 1,
            None => 0,
        };

        if there_are_comments == 1 {
            line.replace_range(index.unwrap().., "");
        }

        return line;
    }

    // Removes the "\" that precedes specific chars that are special encoding
    fn replace_special_encoding(mut line: String) -> String {
        
        let mut ocurrences: Vec<_> = line.match_indices("\\").map(|(i, _)|i).collect();
        match ocurrences.len() {
            0 => return line,
            _ => {}, 
        };

        for index in ocurrences {

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

        }

        return line;
    }

    // Gets the hostname of a line in a master file. If there is no hostname, takes the last hostnames used.
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

    // Returns whether the type is class, rr_type or ttl
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


    /// Adds a new rr to the master file parsings
    fn add_rr(&mut self, host_name: String, resource_record: ResourceRecord) {
        let mut rrs = self.get_rrs();
        
        let mut rrs_vec = Vec::<ResourceRecord>::new();
        //println!("add_rr -> {}",host_name);
        match rrs.get(&host_name) {
            Some(val) => {
                // Boolean value if exists some CNAME record for the hostname
                let mut rrs_host_name_cname = false;
                for rr in val {
                    match rr.get_rdata() {
                        Rdata::SomeCnameRdata(_) => {
                            rrs_host_name_cname = true;
                        },
                        _ => continue
                    }
                }

                match resource_record.get_rdata() {
                    // Adding a CNAME will flush older resource records
                    Rdata::SomeCnameRdata(_) => {rrs_vec.push(resource_record); println!("aaaaaaa")},

                    // If already exists a CNAME record, do nothing
                    // otherwise, adds new record
                    _ => {
                        rrs_vec = val.clone();
                        if !rrs_host_name_cname {
                            rrs_vec.push(resource_record);
                        }
                    }
                }
            },
            None => {
                rrs_vec.push(resource_record);
            }
        }

        rrs.insert(host_name, rrs_vec.to_vec());

        self.set_rrs(rrs);
    }

    // Processes an included file in the master file. 
    fn process_include(&mut self, file_name: String, mut domain_name: String, validity: bool){
        
        let mut prev_class = "".to_string();
        // remeber the parent origin, for now the origin used is going to change
        let parent_origin = self.get_origin();
        
        if validity{
            domain_name = domain_validity_syntax(domain_name.clone()).unwrap();
        }
        
        if domain_name != "" {
            self.set_last_host(domain_name.clone());
        }

        // changing origin to relative domain name of the include
        self.set_origin(domain_name.clone());

        let file = File::open(file_name.clone()).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let line_without_comments = MasterFile::replace_special_encoding(MasterFile::remove_comments(line.clone()).clone());
            self.process_line(line);
        }

        if validity {
            self.check_glue_delegations();
            self.check_cname_loop();
        }
        
        //sets the origin of the parent master file
        self.set_origin(parent_origin);
        
    }

    // Master file: If delegations are present and glue information is required,it should be present.
    fn check_glue_delegations(&self) {
        let origin = self.get_origin();
        let rrs = self.get_rrs();

        let origin_labels: Vec<&str> = origin.split(".").collect();
        let origin_labels_num = origin_labels.len();

        let origin_ns_rr: Vec<ResourceRecord> = match rrs.get(&origin) {
            Some(origin_rrs) => {
                NameServer::look_for_type_records(origin.clone(), origin_rrs.to_vec(), 2)
            },
            None => {
                Vec::<ResourceRecord>::new()
            },
        };


        for ns in origin_ns_rr {

            let ns_name = match ns.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => "".to_string(),
            };
            
            let ns_slice: &str = &ns_name;
            let mut ns_labels: Vec<&str> = ns_slice.split(".").collect();

            while ns_labels.len() >= origin_labels_num {
                // subzone
                if ns_labels == origin_labels {
                    // find glue info for this 
                    match rrs.get(ns_slice){
                        Some(ns_rrs) => {
                            let a_rr_glue = NameServer::look_for_type_records(ns_slice.to_string(), ns_rrs.to_vec(), 1);
                            if a_rr_glue.len() == 0 {
                                panic!("Information outside authoritative node in the zone is not glue information.");
                            }
                        },
                        None => {
                            panic!("Information outside authoritative node in the zone is not glue information.");
                        },
                    } 
                }
                ns_labels.remove(0);
            }
        }
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

#[cfg(test)]
mod master_file_test{
    use std::collections::btree_map::Iter;

    use crate::message::{rdata::{a_rdata::ARdata, cname_rdata::CnameRdata, Rdata}, resource_record::ResourceRecord};

    use super::MasterFile;

    #[test]
    fn remove_comments_test(){
        let line = "dcc  A  192.80.24.11 ; this is a ; line with comments".to_string();
        let line_without_rr = ";line with no RR".to_string();

        let line_without_comments =  MasterFile::remove_comments(line);
        let line_no_rr_without_comments = MasterFile::remove_comments(line_without_rr);
        
        assert_eq!(line_without_comments, "dcc  A  192.80.24.11 ");
        assert_eq!(line_no_rr_without_comments, "");
    }

    #[test]
    fn replace_special_encoding_test(){

        let line1 = r"a  IN  SOA VENERA  Action\.domains 20 7200 600 3600000 60".to_string();
        let line2 = r"a  IN  SOA VENERA  Action.domains 20 7200 \600 3600000 60".to_string();
        let line3= r"\@  IN  A 123.123.123.123".to_string();
        //let line = r"a  IN  SOA VENERA  Action\.domains 20 7200 \600 3600000 60".to_string(); 
        
        let line_without_special_enc1 = MasterFile::replace_special_encoding(line1); 
        let line_without_special_enc2 = MasterFile::replace_special_encoding(line2); 
        let line_without_special_enc3 = MasterFile::replace_special_encoding(line3); 

        assert_eq!(line_without_special_enc1, "a  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60");
        assert_eq!(line_without_special_enc2, "a  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60");
        assert_eq!(line_without_special_enc3, "@  IN  A 123.123.123.123");
    }

    #[test]
    fn get_line_host_name_test(){
        //look for host name and sets last_host

        let origin = "uchile.cl".to_string();
        let mut masterFile = MasterFile::new(origin.clone());
        masterFile.set_last_host("test.uchile.cl".to_string());
        
        
        let line_1 = "  A   192.168.1.1".to_string();             //without host
        let line_2 = "a NS  VENERA".to_string();    //with host
        let line_3 = "  A   192.168.1.1".to_string(); 
        let line_4 = "a.uchile.cl  A   192.168.1.1".to_string(); 
        let line_5 = "  A   192.168.1.1".to_string(); 
        

        let case1 = masterFile.get_line_host_name(line_1) ;
        let case2 = masterFile.get_line_host_name(line_2) ;
        let case3 = masterFile.get_line_host_name(line_3);
        let case4 = masterFile.get_line_host_name(line_4) ;
        let case5 = masterFile.get_line_host_name(line_5);

        assert_eq!(case1.0, "test.uchile.cl".to_string());
        assert_eq!(case2.0,"a".to_string() );
        assert_eq!(case3.0, "a".to_string());
        assert_eq!(case4.0, "a.uchile.cl".to_string());
        assert_eq!(case5.0, "a.uchile.cl".to_string());

    }

    #[test]
    fn get_value_type_test(){

        let rest_line = "   IN     NS      VENERA".split_whitespace();

        let origin = "uchile.cl".to_string();
        let mut masterFile = MasterFile::new(origin);
        masterFile.set_last_host("test".to_string());

        let true_values = vec![1,2,0];
        
        let mut i = 0;
        for value in rest_line {
            let type_value =  masterFile.get_value_type(value.to_string());
            assert_eq!(type_value,true_values[i]);
            i+=1;
        }
    }

    #[test]
    fn add_rr_test() {
        let mut master_file = MasterFile::new("uchile.cl".to_string());
        let new_a1_record = ARdata::rr_from_master_file(
            "204.13.100.3".split_whitespace(),
            0,
            0,
            "test.uchile.cl.".to_string());

        let new_cname1_record = CnameRdata::rr_from_master_file("test.googleplex.edu".split_whitespace(),
            0,
            0,
            "test.uchile.cl".to_string(),
            "test.uchile.cl".to_string());

        let new_a2_record = ARdata::rr_from_master_file(
            "204.13.100.3".split_whitespace(),
            0,
            0,
            "test.uchile.cl.".to_string());

        let new_cname2_record = CnameRdata::rr_from_master_file("test.googleplex.com".split_whitespace(),
            0,
            0,
            "test.uchile.cl".to_string(),
            "test.uchile.cl".to_string());

        // Always have just 1 RR CNAME
        master_file.add_rr("test".to_string(), new_a1_record.clone());
        assert_eq!(master_file.get_rrs().get("test").unwrap().len(), 1);
        master_file.add_rr("test".to_string(), new_cname1_record);
        assert_eq!(master_file.get_rrs().get("test").unwrap().len(), 1);
        master_file.add_rr("test".to_string(), new_a2_record);
        assert_eq!(master_file.get_rrs().get("test").unwrap().len(), 1);
        master_file.add_rr("test".to_string(), new_cname2_record);
        assert_eq!(master_file.get_rrs().get("test").unwrap().len(), 1);

        //case 2  same host diferentes ways
        master_file.add_rr("test.uchile.cl".to_string(), new_a1_record);
        assert_eq!(master_file.get_rrs().get("test").unwrap().len(), 1);
        assert_eq!(master_file.get_rrs().get("test.uchile.cl").unwrap().len(), 1);

    }


    #[test]
    fn process_line_rr_test(){
        //gets ttl, clas, type 
        //same as process_line_rr_no_validation(..)
        let line_ns = "dcc 33 IN NS ns.test.cl".to_string();
        let line_ns_default = " NS ns.test.cl".to_string();

        let mut masterFile = MasterFile::new("uchile.cl".to_string());
        masterFile.set_last_host("test".to_string());
        masterFile.set_origin("uchile.cl".to_string());
        masterFile.set_ttl_default(33);
        masterFile.set_class_default("IN".to_string());

        let (host,rest_line) = masterFile.process_line_rr(line_ns,true);
        let (host_default,rest_line_default) = masterFile.process_line_rr(line_ns_default,true);

        let rrs = masterFile.get_rrs();
        let vec_test2_rr = rrs.get("dcc.uchile.cl").unwrap();

        for rr in vec_test2_rr.iter() {
            let type_rr = rr.get_type_code();
            let class_rr = rr.get_class();
            let ttl_rr = rr.get_ttl();
            let name_rr = rr.get_name().get_name();

            assert_eq!(type_rr,2); 
            assert_eq!(class_rr,1);
            assert_eq!(ttl_rr,33);
            assert_eq!(name_rr,"dcc.uchile.cl".to_string());
    
        }

    }


    #[test]
    fn process_line_rr_no_validation_test(){
        //gets ttl, clas, type 
        let line_ns = "dcc 33 IN NS ns.test.cl".to_string();
        let line_ns_default = " NS ns.test.cl".to_string();

        let line_a ="           A       192.80.24.11".to_string();
        let line_mx = "     24         MX      20      VAXA".to_string();


        let mut masterFile = MasterFile::new("uchile.cl".to_string());
        masterFile.set_last_host("test".to_string());
        masterFile.set_origin("uchile.cl".to_string());
        masterFile.set_ttl_default(33);
        masterFile.set_class_default("IN".to_string());

        masterFile.process_line_rr(line_ns,false);
        masterFile.process_line_rr(line_ns_default,false);
        masterFile.process_line_rr(line_a, false);
        masterFile.process_line_rr(line_mx,false);

        let rrs = masterFile.get_rrs();
        let vec_test2_rr = rrs.get("dcc.uchile.cl").unwrap();

        let true_val = vec![(2,1,33),(2,1,33),(1,1,33),(15,1,24)];
        let mut i =0;
        for rr in vec_test2_rr.iter() {
            let type_rr = rr.get_type_code();
            let class_rr = rr.get_class();
            let ttl_rr = rr.get_ttl();
            let name_rr = rr.get_name().get_name();
            
            assert_eq!(type_rr,true_val[i].0); 
            assert_eq!(class_rr,true_val[i].1);
            assert_eq!(ttl_rr,true_val[i].2);
            assert_eq!(name_rr,"dcc.uchile.cl".to_string());
            i+=1;
    
        }


    }

    #[test]
    fn process_specific_rr_test(){
        //identifies class, type create RR
        let values_soa = "VENERA      Action.domains	20	7200	600	3600000	60".split_whitespace();
        let values_mx = "20      VAXA".split_whitespace();
        let values_a = "192.80.24.10".split_whitespace();
        let values_ns = "A.ISI.EDU.".split_whitespace();

        let mut masterFile = MasterFile::new("uchile.cl".to_string());
        masterFile.set_last_host("test".to_string());
        masterFile.set_origin("uchile.cl".to_string());
        
        //MX
        masterFile.process_specific_rr(
            values_mx,
            0,
            "IN".to_string(),
            "MX".to_string(),
            "test2".to_string(),
            true);
        
        //A
        masterFile.process_specific_rr(
            values_a,
            0,
            "IN".to_string(),
            "A".to_string(),
            "test2".to_string(),
            true);    

        
        //SOA
        masterFile.process_specific_rr(
            values_soa,
            0,
            "IN".to_string(),
            "SOA".to_string(),
            "test2".to_string(),
            true);

        //ns
        masterFile.process_specific_rr(
            values_ns,
            0,
            "IN".to_string(),
            "NS".to_string(),
            "test2".to_string(),
            true);
            
        let rrs = masterFile.get_rrs();
        let vec_test2_rr = rrs.get("test2.uchile.cl").unwrap();

        let vect_true_val = vec![(15,1),(1,1),(6,1),(2,1)];
        let mut i = 0 ;
        for rr in vec_test2_rr.iter() {
            let type_rr = rr.get_type_code();
            let class_rr = rr.get_class();

            assert_eq!(type_rr,vect_true_val[i].0); 
            assert_eq!(class_rr,vect_true_val[i].1);  
            i +=1;
        }
        
    }

    #[test]
    fn process_especific_rr_no_validation_test(){
        //identifies class, type create RR
        let values_soa = "VENERA      Action.domains	20	7200	600	3600000	60".split_whitespace();
        let values_mx = "20      VAXA".split_whitespace();
        let values_a = "192.80.24.10".split_whitespace();
        let values_ns = "A.ISI.EDU.".split_whitespace();


        let mut masterFile = MasterFile::new("uchile.cl".to_string());
        masterFile.set_last_host("test".to_string());
        masterFile.set_origin("uchile.cl".to_string());
        
        //MX
        masterFile.process_specific_rr(
            values_mx,
            0,
            "IN".to_string(),
            "MX".to_string(),
            "test2".to_string(),
            false);
        
        //A
        masterFile.process_specific_rr(
            values_a,
            0,
            "IN".to_string(),
            "A".to_string(),
            "test2".to_string(),
            false);    

        
        //SOA
        masterFile.process_specific_rr(
            values_soa,
            0,
            "IN".to_string(),
            "SOA".to_string(),
            "test2".to_string(),
            false);

        //ns
        masterFile.process_specific_rr(
            values_ns,
            0,
            "IN".to_string(),
            "NS".to_string(),
            "test2".to_string(),
            false);

        //process_especific_rr_no_validation create RR and saves it  with full host name
        let rrs = masterFile.get_rrs();

    
        
        let vec_test2_rr = rrs.get("test2.uchile.cl").unwrap();
        

        let a = "test2.uchile.cl".to_string();
        let vect_true_val = vec![(15,1),(1,1),(6,1),(2,1)];
        let mut i = 0 ;
        for rr in vec_test2_rr.iter() {
            let type_rr = rr.get_type_code();
            let class_rr = rr.get_class();

            assert_eq!(type_rr,vect_true_val[i].0); 
            assert_eq!(class_rr,vect_true_val[i].1);  
            i +=1;
        }
         
    }


    
    #[test]
    fn process_lines_and_validation_test(){

        //basic line
        let line_a = "a             A       192.80.24.11".to_string();
        
        //at line
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        
        //without host line
        let line_no_host = " NS  a".to_string();
        
        //subzone line
        let line_sub_zone = "a.b     A       111.222.333.444".to_string();

        //* line
        let line_special = "*.dcc    A       111.222.333.555".to_string();

        let vec_lines = vec![line_soa,line_a,line_no_host];
        //let vec_lines = vec![line_soa,line_a,line_no_host,line_sub_zone,];

        let mut master_file = MasterFile::new("uchile.cl".to_string());
        
        master_file.process_lines_and_validation(vec_lines);

        let  rrs = master_file.get_rrs();
        assert_eq!(rrs.len(),2);

        let vect_1 =rrs.get("uchile.cl").unwrap(); 
        //assert_eq!(vect_1.first().unwrap().get_name().get_name(),"uchile.cl");
       
        let vect_2 = rrs.get("a.uchile.cl").unwrap();

        let true_rdata = vec!["192.80.24.11","a.uchile.cl"];

        let mut  i =0;
        for val in vect_2{
            let rdata  = match val.get_rdata(){ 
                Rdata::SomeARdata(v) => v.get_string_address(),
                Rdata::SomeNsRdata(v) => v.get_nsdname().to_string(),
                _=>"none".to_string(),
            };
            //println!("***{}",rdata);
            assert_eq!(rdata, true_rdata[i]);
            i +=1;
        }

    }


    #[test]
    fn process_line_test() {
        //dafault values
        let line_normal = "a             A       192.80.24.11".to_string();

        //without host name
        let line_whithout_host: String = "  A 192.168.100.115".to_string();

        //with at
        let at_line_case1 = "@             A       192.80.24.12".to_string();
        let at_line_case2 = "www   IN   MX 20  @".to_string();
        //let at_line_case3 = "@   IN   CNAME   @.dom".to_string();

        //subzone line
        let line_sub_zone = "a.b     A       111.222.333.444".to_string();

        //* line
        let line_special = "*.dcc    A       111.222.333.555".to_string();


        let mut master_file = MasterFile::new("uchile.cl".to_string());
        master_file.process_line(line_normal);
        master_file.process_line(line_whithout_host);
        master_file.process_line(at_line_case1);
        master_file.process_line(at_line_case2);
        //master_file.process_line(line_sub_zone); 
        //master_file.process_line(line_special); 


        let mut rrs = master_file.get_rrs();

        let vect_origin = rrs.get("uchile.cl").unwrap();
        let vect_a = rrs.get("a.uchile.cl").unwrap();
        let vect_www = rrs.get("www.uchile.cl").unwrap();
        
        let last_host = master_file.get_last_host();
        assert_eq!(vect_origin.len(),1);
        assert_eq!(vect_a.len(),2);
        assert_eq!(vect_www.len(),1);

        let true_val = vec!["192.80.24.11".to_string(),"192.80.24.12".to_string(),"a.uchile.cl".to_string(),"192.168.100.115".to_string()];

        let mut i = 0;
        for (host, vect) in rrs{

            for rr in vect{
                let somedata  = match rr.get_rdata(){ 
                    Rdata::SomeARdata(v) => v.get_string_address(),
                    Rdata::SomeNsRdata(v) => v.get_nsdname().to_string(),
                    Rdata::SomeNsRdata(v) => v.get_nsdname().to_string(),
                    Rdata::SomeMxRdata(v) => v.get_exchange().get_name(),
                    _=>"none".to_string(),
                };

                if host == "uchile.cl".to_string(){
                    //Rdata of RR A replace @
                    assert_eq!(somedata,"192.80.24.12".to_string());
                }
                else if host =="www.uchile.cl" {
                    //Rdata of RR MX replace @
                   assert_eq!(somedata, "uchile.cl".to_string());                    
                }
                else {//host a
                    //Rdata of RR A without host and normal 
                    assert!(somedata == "192.168.100.115".to_string() || somedata == "192.80.24.11".to_string());
                }
                i +=1;

            }
        }
    }

    #[test]
    #[should_panic (expected = "Information outside authoritative node in the zone is not glue information.")]
    fn check_glue_delegations_test_fail(){
        
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let line_ns = "@  NS ns".to_string();
        let vec_lines = vec![line_soa,line_ns];
        
        let mut master_file = MasterFile::new("uchile.cl".to_string());
                
        master_file.process_lines_and_validation(vec_lines);

        let rrs = master_file.get_rrs();
        /*
        for keys in rrs.keys() {
            println!("nombre de host -> {}", keys);
        } */
       master_file.check_glue_delegations(); 
    }
    
    #[test]
    fn check_glue_delegations_test(){
        
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let line_ns = "@  NS ns".to_string(); //doest acept if is -> "uchile.cl  NS ns"
        let line_a = "ns  A  192.80.24.11".to_string();
        
        let vec_lines = vec![line_soa,line_ns,line_a];
        
        let mut master_file = MasterFile::new("uchile.cl".to_string());
                
        master_file.process_lines_and_validation(vec_lines);

        let rrs = master_file.get_rrs();
        master_file.check_glue_delegations();    
    }

}
