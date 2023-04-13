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
use crate::message::resource_record::ResourceRecord;
//refactor
use crate::name_server::NameServer;
use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::str::SplitWhitespace;

//utils
use crate::utils::domain_validity_syntax;
use crate::utils::is_reverse_query;

#[derive(Clone)]
/// Structs that represents data from a master file
pub struct MasterFile {
    top_host: String,
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
            top_host: "".to_string(),
            origin: origin,
            last_host: "".to_string(),
            rrs: HashMap::<String, Vec<ResourceRecord>>::new(),
            class_default: "".to_string(),
            ttl_default: 0,
        };

        master_file
    }

    /// Creates a new master file given th parameters filename and origin. For listing cache contents.
    /// Set validation to true if checking validity syntax of the master file is desired.
    pub fn from_file(filename: String, origin: String, validation: bool) -> Self {
        print!("checkpint1");
        let file = File::open(filename).expect("file not found!");
        print!("checkpint1");
        let reader = BufReader::new(file);
        print!("checkpint1");
        //save origin with . at end
        let mut origin = origin;
        if origin.ends_with(".") == false {
            origin.push('.');
        }

        let mut master_file = MasterFile::new(origin);

        //representation of each RR in Master File
        let lines: Vec<String> = master_file.lines_to_vect(reader);

        //process lines and creates RR
        println!("Creating new Masterfile");
        master_file.process_lines(lines, validation);

        if validation {
            println!("Starting validation...");
            // validate presence of glue records when necessary
            master_file.check_glue_delegations();
            // look for cname loops
            master_file.check_cname_loop();
            // look for at least one NS RR at the top of the zone
            master_file.check_existence_top_ns();
            println!("Masterfile validated correctly.");
        }

        master_file
    }

    //TODO
    //OLD EXPLANATION
    // Recives buffer with all the master file and returna a vector with lines representing each  RR in the Master File.
    // Thera two option for a represented RR in a Master File:
    // - RR is represented in one line: Read the lines and add it to the returning vector
    // - RR is represented in more than one line: Read each line od the RR , join it andLeaves it in a line then adds it
    // to the returning vector
    //NEW EXPLANATION
    /// Receives a buffer with all the master file and returns a vector with lines representing each line in the Master File.
    /// The given vector has no comments and replaced special encoding.
    /// There are two options for a represented RR in a Master File:
    /// - RR is represented in one line: Reads the line and adds it to the returning vector
    /// - RR is represented in more than one line: Reads each line of the RR, an then joins it and leaves it in a line, then adds it
    /// to the returning vector  
    fn lines_to_vect(&mut self, buffer: BufReader<File>) -> Vec<String> {
        let mut last_line = "".to_string();
        let mut lines: Vec<String> = Vec::new();

        // Link lines with parenthesis and remove comments
        for line in buffer.lines() {
            let line = line.unwrap();

            // Remove comments and replace especial encoding
            let line_without_comments = MasterFile::replace_special_encoding(
                MasterFile::remove_comments(line.clone()).clone(),
            );

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

        return lines;
    }

    /// Obtains the host name and the values for creating a RR.
    /// Return class, type and absolute host name of the RR for validation.
    fn process_line_rr(&mut self, line: String) -> (String, String, String) {
        // Gets full host name
        let (full_host_name, line_left_to_process) = self.get_full_host_name(line.clone());

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

            println!("Name: {}, value: {}", full_host_name.clone(), value_type);

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

        // If line is a reverse query it will ignore it
        let is_reverse_query = is_reverse_query(full_host_name.clone());
        if is_reverse_query {
            println!("RR for Inverse querys");
        } else {
            self.process_specific_rr(
                next_line_items,
                ttl.clone(),
                class,
                rr_type.to_string(),
                full_host_name.clone(),
            );
        }

        return (this_class, this_type, full_host_name);
    }

    //TODO check if this function can be splited into more functions, because it does an excesive amount
    // of things, and check the use of process_include (cause process_include uses process_lines).
    //OLD EXPLANATION
    //  Checks all the lines in the master file.
    //  Looks for $ORIGIN control entries, changing the current origin for relative
    //  domain names to the stated name.
    //  Looks for $INCLUDE control entries, inserting the named file into
    //  the current file.
    //  Ensures there is only one SOA rr, and that it is the first rr in the master file.
    //  Ensures the remaining rr in the master file belongs to the same class (not SOA).
    //  Ensures at least one NS RR must be present at the top of the zone. !!! [MISSING]
    //NEW EXPLANATION
    ///  Checks all the lines in the Master File.    
    ///  - Looks for $ORIGIN control entries, changing the current origin for relative
    ///  domain names to the stated name.
    ///  - Looks for $INCLUDE control entries, inserting the named file into
    ///  the current file.
    ///  - Ensures that there is only one SOA RR, and it's the first RR in the Master File.
    ///  - Ensures that the remaining RR's in the Master File belongs to the same class (not SOA).
    ///  - Ensures that at least one NS RR must be present at the top of the zone. !!! [MISSING]   
    fn process_lines(&mut self, lines: Vec<String>, validity: bool) {
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

                // save origin with . at end
                if name.ends_with(".") == false {
                    name.push_str(".");
                }

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

            // replace @ for origin
            let contains_non_especial_at_sign = line.contains("@");

            let mut new_line = line.clone();
            if contains_non_especial_at_sign {
                let full_origin = self.get_origin();
                new_line = line.replace("@", &full_origin);
            }

            let (rr_class, rr_type, host_name) = self.process_line_rr(new_line);

            if validity {
                match self.host_name_master_file_validation(host_name) {
                    Err(_) => panic!("Error: host name is not valid"),
                    Ok(_) => (),
                };

                //First RR must be SOA
                if prev_rr_class == "" {
                    //first RR must be SOA
                    if rr_type != "SOA".to_string() {
                        panic!("No SOA RR is present at the top of the zone.");
                    }

                    prev_rr_class = rr_class;
                }
                //Can not exist more tha one SOA and all RR must be the same class
                else {
                    if rr_class != prev_rr_class {
                        panic!("Not all rr have the same class.");
                    }
                    if rr_type == "SOA".to_string() {
                        panic!("More than one SOA per zone.");
                    }
                }
            }
        }
    }

    // TODO: change in documentation
    // Old:
    // detect cname loops of type 1->2->1:
    // example of CNAME loop with two CNAMEs 1 -> 2 -> 1 -> 2 -> 1, etc.
    //     alias1.example.org. 3600 CNAME alias2.example.org.
    //     alias2.example.org. 3600 CNAME alias1.example.org.
    // New:
    /// Detects CNAME loop of type 1 -> 2 -> 1.
    ///
    /// Example of a CNAME loop of type 1 -> 2 -> 1, with two CNAME RRs which are pointing to
    /// each other:
    ///
    ///     alias1.example.org. 3600 CNAME alias2.example.org.
    ///
    ///     alias2.example.org. 3600 CNAME alias1.example.org.
    ///
    fn check_cname_loop(&self) {
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

            if cname_by_host.len() > 0 {
                cname_rrs.insert(hostname.to_string(), cname_by_host);
            }
        }

        for (alias, canonical) in &cname_rrs {
            let rdata = canonical[0].get_rdata();
            let canonical_name = match rdata {
                Rdata::SomeCnameRdata(val) => val.get_cname().get_name(),
                _ => unreachable!(),
            };
            match cname_rrs.get(&canonical_name.to_string()) {
                Some(val) => match val[0].get_rdata() {
                    Rdata::SomeCnameRdata(crr) => {
                        if crr.get_cname().get_name().to_string() == alias.to_string() {
                            panic!("CNAME loop detected!");
                        }
                        continue;
                    }
                    _ => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            };
        }
    }

    ///Process information of an specific type of RR and creates it,
    ///saves the RR with the absolute host name.
    fn process_specific_rr(
        &mut self,
        items: SplitWhitespace,
        ttl: u32,
        class: String,
        rr_type: String,
        full_host_name: String,
    ) {
        let origin = self.get_origin();

        let class_int = match class.clone().as_str() {
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
                let (rr, minimum) = SoaRdata::rr_from_master_file(
                    items,
                    ttl,
                    class_int,
                    full_host_name.clone(),
                    origin.clone(),
                );
                self.set_ttl_default(minimum);
                self.set_class_default(class);
                self.set_top_host(full_host_name.clone());
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

    /// Removes the comments from a line in a master file.
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

    /// Removes the "\" that precedes specific chars that are special encoding.
    fn replace_special_encoding(mut line: String) -> String {
        let ocurrences: Vec<_> = line.match_indices("\\").map(|(i, _)| i).collect();
        match ocurrences.len() {
            0 => return line,
            _ => {}
        };

        for index in ocurrences {
            let next_char_to_backslash = line.get(index + 1..index + 2).unwrap().to_string();

            /*
                \DDD where each D is a digit is the octet corresponding to
                the decimal number described by DDD. The resulting
                octet is assumed to be text and is not checked for
                special meaning.
            */
            if next_char_to_backslash >= "0".to_string()
                && next_char_to_backslash <= "9".to_string()
            {
                let oct_number_str = line.get(index + 1..index + 4).unwrap();
                let oct_number = oct_number_str.parse::<u32>().unwrap();
                let dec_str = oct_number.to_string();
                line.replace_range(index..index + 4, &dec_str);
            }
            /*
                \X where X is any character other than a digit (0-9), is
                used to quote that character so that its special meaning
                does not apply. For example, "\." can be used to place
                a dot character in a label.
            */
            else {
                let x = next_char_to_backslash.to_string();
                line.replace_range(index..index + 2, &x);
            }
        }

        return line;
    }

    // TODO: change in documentation redaction
    // OLD EXPLANATION:
    // Gets the hostname  of a line in a master file.
    //  - If there is no hostname, takes the last hostnames used.
    //  - If host name is relative changes it to full host name.
    //  - Error when theres a relative name and no origin
    // NEW EXPLANATION:
    /// Receives a line from a master file to get the hostname according to
    /// the following cases:
    /// - If there is no hostname, it takes the last host name used.
    /// - If the host name is relative, it changes it to the full host name.
    /// - If the host name is relative but no origin was given, it panics.
    ///
    /// Returns a tuple with the full host name and the line left to process.
    fn get_full_host_name(&mut self, line: String) -> (String, String) {
        let first_char = line.get(0..1).unwrap();
        let origin = self.get_origin();
        let mut full_host_name;
        let mut line_left_to_process = "".to_string();

        let mut iter = line.split_whitespace();

        // If no host name is given, uses the last host name
        if first_char == " ".to_string() {
            full_host_name = self.get_last_host();
            // line_left_to_process = line.clone();
        } else {
            full_host_name = iter.clone().next().unwrap().to_string();

            // Full host name for RRs in a hashmap
            if full_host_name.ends_with(".") == false {
                if origin == "" {
                    panic!("Error: No origin for relative name");
                } else if origin != "." {
                    full_host_name.push_str(".");
                    full_host_name.push_str(&origin);
                } else {
                    full_host_name.push_str(&origin);
                }
            }
            self.set_last_host(full_host_name.clone());
            iter.next();
        }
        for value in iter {
            line_left_to_process.push_str(value);
            line_left_to_process.push(' ');
        }
        return (full_host_name, line_left_to_process);
    }

    //TODO : check when the function recieves a random string
    //cause will take it as the TTL, could be changed to expect a number
    //OLD EXPLANATION
    // Returns whether the type is class, rr_type or ttl:
    // - 1 -> Class
    // - 2 -> Type
    // - 0 -> TTL
    //NEW EXPLANATION THAT ADJUSTS TO ITS FUNCTIONALITY
    /// Returns whether the type is class, rr_type or another string:
    /// - 1 -> Class
    /// - 2 -> Type
    /// - 0 -> another string
    /// - the another string is expected to be a number(TTL)
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

    /// Adds a new RR to the master file parsings.
    fn add_rr(&mut self, host_name: String, resource_record: ResourceRecord) {
        let mut rrs = self.get_rrs();

        let mut rrs_vec = Vec::<ResourceRecord>::new();
        match rrs.get(&host_name) {
            Some(val) => {
                // Boolean value if exists some CNAME record for the hostname
                let mut rrs_host_name_cname = false;
                for rr in val {
                    match rr.get_rdata() {
                        Rdata::SomeCnameRdata(_) => {
                            rrs_host_name_cname = true;
                        }
                        _ => continue,
                    }
                }

                match resource_record.get_rdata() {
                    // Adding a CNAME will flush older resource records
                    Rdata::SomeCnameRdata(_) => rrs_vec.push(resource_record),

                    // If already exists a CNAME record, do nothing
                    // otherwise, adds a new record
                    _ => {
                        rrs_vec = val.clone();
                        if !rrs_host_name_cname {
                            rrs_vec.push(resource_record);
                        }
                    }
                }
            }
            None => {
                rrs_vec.push(resource_record);
            }
        }

        rrs.insert(host_name, rrs_vec.to_vec());

        self.set_rrs(rrs);
    }

    //TODO: check validity, and the use of process_lines (cause process_lines uses process_include)
    //this function could be modified so it doesn't do that.
    //OLD EXPLANATION:
    // Processes an included file in the master file.
    //NEW EXPLANATION
    ///Processes an included file in the Master File.
    fn process_include(&mut self, file_name: String, domain_name: String, validity: bool) {
        // remeber the parent origin, for now the origin used is going to change
        let parent_origin = self.get_origin();
        let mut full_host_name = domain_name;

        if full_host_name.ends_with(".") == false {
            full_host_name.push_str(".");
        }

        if full_host_name != "" {
            self.set_last_host(full_host_name.clone());
            // changing origin to relative domain name of the include
            self.set_origin(full_host_name.clone());
        }

        let file = File::open(file_name.clone()).expect("file not found!");
        let reader = BufReader::new(file);

        //representation od each RR in Master File
        let lines: Vec<String> = self.lines_to_vect(reader);

        //process lines in a MF
        self.process_lines(lines, false);

        if validity {
            self.check_glue_delegations();
            self.check_cname_loop();
        }

        //sets the origin of the parent master file
        self.set_origin(parent_origin);
    }

    // TODO
    // Old explanation:
    // If RR type NS is presented and server is a subzone checks if exist RR type A for this host name (glue record).
    // New explanation:
    /// If a RR of type NS is presented and the server is a subzone, checks if a RR of type A exists
    /// for this glue record's host name.
    fn check_glue_delegations(&self) {
        let origin = self.get_origin();
        let mut rrs = self.get_rrs();
        let top_host = self.get_top_host();

        // All RR which need glue records
        rrs.remove(&origin);

        let top_host_labels: Vec<&str> = top_host.split(".").collect();
        let top_host_labels_num = top_host_labels.len();

        for rr_host in rrs.iter() {
            let name_rr = rr_host.0;
            let rrs_ns = NameServer::look_for_type_records(name_rr.clone(), rr_host.1.to_vec(), 2);

            for ns in rrs_ns.iter() {
                // Name of the server
                let ns_name = match ns.get_rdata() {
                    Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                    _ => "".to_string(),
                };
                // println!("ns server name -----> {}",ns_name.clone());
                let ns_slice: &str = &ns_name;
                let mut ns_labels: Vec<&str> = ns_slice.split(".").collect();

                while ns_labels.len() >= top_host_labels_num {
                    // subzone

                    // If they are the end part of the host is the same as the top node glue rr must exist
                    if ns_labels == top_host_labels {
                        // Find glue info for this
                        match rrs.get(ns_slice) {
                            Some(ns_rrs) => {
                                let a_rr_glue = NameServer::look_for_type_records(
                                    ns_slice.to_string(),
                                    ns_rrs.to_vec(),
                                    1,
                                );
                                if a_rr_glue.len() == 0 {
                                    panic!("Information outside authoritative node in the zone is not glue information.");
                                }
                            }
                            None => {
                                panic!("Information outside authoritative node in the zone is not glue information.");
                            }
                        }
                    }
                    ns_labels.remove(0);
                }
            }
        }
    }

    //TODO
    //OLD EXPLANATION:
    //Checks thata exist at least one RR type NS present at theh top of the zone
    //MODIFIED EXPLANATION
    ///Checks that exists at least one RR type NS present at the top of the zone.
    fn check_existence_top_ns(&self) {
        let top_host = self.get_top_host();
        let rrs_top_host = self.get_rrs().get(&top_host).unwrap().clone();

        let top_host_ns = NameServer::look_for_type_records(top_host, rrs_top_host, 2);

        let count_ns_top_host = top_host_ns.len();
        if count_ns_top_host == 0 {
            panic!("Error: No NS RR at top of the zone");
        }
    }

    //checks validity of a host in a master file cases:
    //      - wildcard
    //      - inverse query
    fn host_name_master_file_validation(&self, host_name: String) -> Result<String, &'static str> {
        //wildcard validation
        match host_name.split_once('.') {
            Some((firs_label, rest_hostname)) => {
                if firs_label.to_string() == "*" {
                    //is wildcard
                    return domain_validity_syntax(rest_hostname.to_string());
                }
            }
            _ => {
                //normal host name
                if host_name.to_string() != "*" {
                    return domain_validity_syntax(host_name.clone());
                }
            }
        }

        //inverse query
        let mut length_ip = 4;

        let mut host_to_validate = host_name.clone();
        while length_ip > 0 {
            let (label, labels) = host_to_validate.split_once('.').unwrap();

            let label_num = label.parse::<i32>();
            match label_num {
                Ok(_ok) => length_ip -= 1,
                _ => break,
            }
            host_to_validate = labels.to_string();
        }

        if length_ip == 0 {
            return domain_validity_syntax(host_to_validate);
        } else if host_name != ".".to_string() {
            return domain_validity_syntax(host_name);
        }
        return Ok(host_name);
    }
}
// Getters
impl MasterFile {
    // Gets the name top domain
    pub fn get_top_host(&self) -> String {
        self.top_host.clone()
    }

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
    // Sets the name of top domain with a new value
    pub fn set_top_host(&mut self, name: String) {
        self.top_host = name;
    }

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
mod master_file_test {
    use super::MasterFile;
    use crate::message::{
        rdata::{
            a_ch_rdata::AChRdata, a_rdata::ARdata, cname_rdata::CnameRdata,
            hinfo_rdata::HinfoRdata, mx_rdata::MxRdata, ns_rdata::NsRdata, ptr_rdata::PtrRdata,
            soa_rdata::SoaRdata, txt_rdata::TxtRdata, Rdata,
        },
        resource_record::ResourceRecord,
    };
    use std::{collections::HashMap, fs::File, io::BufReader, vec};

    #[test]
    fn constructor() {
        let origin = "uchile.cl".to_string();
        let empty_string = "".to_string();

        let master_file: MasterFile = MasterFile::new(origin.clone());

        let rr_number = master_file.rrs.len();

        assert_eq!(master_file.top_host, empty_string);
        assert_eq!(master_file.origin, origin);
        assert_eq!(master_file.last_host, empty_string);
        assert_eq!(rr_number, 0);
        assert_eq!(master_file.class_default, empty_string);
        assert_eq!(master_file.ttl_default, 0);
    }

    #[test]
    fn from_file_no_validation() {
        let filename = "test.txt".to_string();
        let origin = "uchile.cl.".to_string();
        let master_file = MasterFile::from_file(filename, origin, false);

        let rrs = master_file.get_rrs();
        assert_eq!(rrs.len(), 14);

        // To get a specific RR, it's necessary to use the absolute value.
        // The RRs values get stored in their absolute domain name, therefore it's necessary to call
        // get() function using its absolute form and not relative.
        let vec_rr_origin = rrs.get("uchile.cl.").unwrap();
        let vec_rr_delegation = rrs.get("delegation.uchile.cl.").unwrap();

        // The origin example.com gets a dot
        let vec_rr_example_include = rrs.get("example.com.").unwrap();

        assert_eq!(vec_rr_origin.len(), 6);
        assert_eq!(vec_rr_delegation.len(), 2);
        assert_eq!(vec_rr_example_include.len(), 3);
    }

    #[test]
    fn from_file_validation() {
        let filename = "test.txt".to_string();
        let origin = "uchile.cl.".to_string();
        let master_file = MasterFile::from_file(filename, origin, true);

        let rrs = master_file.get_rrs();
        assert_eq!(rrs.len(), 14);

        let vec_rr_origin = rrs.get("uchile.cl.").unwrap();
        let vec_rr_delegation = rrs.get("delegation.uchile.cl.").unwrap();
        let vec_rr_example_include = rrs.get("example.com.").unwrap();

        assert_eq!(vec_rr_origin.len(), 6);
        assert_eq!(vec_rr_delegation.len(), 2);
        assert_eq!(vec_rr_example_include.len(), 3);
    }

    #[test]
    fn lines_to_vect() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let file_name = "1034-scenario-6.1-edu.txt".to_string();
        let file = File::open(file_name.clone()).expect("file not found!");
        let reader = BufReader::new(file);
        let lines: Vec<String> = master_file.lines_to_vect(reader);

        let line_example_1 = "                NS      SRI-NIC.ARPA.".to_string();
        let line_example_2 = "UCI  172800     NS      ICS.UCI".to_string();
        let line_example_3 = "                172800  A    128.9.0.32".to_string();
        let line_example_4 = "ACHILLES.MIT.EDU.  43200  A   18.72.0.8".to_string();
        let line_example_5 = "".to_string();

        assert_eq!(lines.get(2).unwrap().clone(), line_example_1);
        assert_eq!(lines.get(5).unwrap().clone(), line_example_2);
        assert_eq!(lines.get(19).unwrap().clone(), line_example_3);
        assert_eq!(lines.get(36).unwrap().clone(), line_example_4);
        //Empty line example
        assert_eq!(lines.get(4).unwrap().clone(), line_example_5);
        //Number of elements
        assert_eq!(lines.len(), 37);
    }

    #[test]
    fn process_line_rr() {
        // Obtains the host name and related values for creating a RR.
        // Returns the class of the RR, the type of the RR and the absolute host name to which it is
        // associated for validation.

        // Test that the class and type obtained are correct.
        // The validation of the lines is not tested, since it is assumed it will be later
        // validated by process_lines() function.
        let origin = "uchile.cl.".to_string();
        let mut master_file = MasterFile::new(origin);

        // SOA: It's necessary to add SOA to the new master file before the other RRs
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();

        // NS
        let line_ns = "dcc  IN NS ns".to_string();
        let line_ns_full_host = "dcc.uchile.cl. 33 IN NS ns".to_string();
        let line_ns_default = " NS ns2".to_string();

        // A
        let line_a_subdomain = "a.b A  192.168.21.2".to_string();
        let line_a_wildcard = "*.b A  192.168.222.44".to_string();

        // CNAME
        let line_cname = "test CNAME    no-test.com.".to_string();

        // MX
        let line_mx = "     MX      10  m.uchile.cl.".to_string();

        let lines = vec![
            line_soa,
            line_ns,
            line_ns_full_host,
            line_ns_default,
            line_a_subdomain,
            line_a_wildcard,
            line_cname,
            line_mx,
        ];

        // Expected types
        let expected_soa_type = "SOA".to_string();
        let expected_ns_type = "NS".to_string();
        let expected_a_type = "A".to_string();
        let expected_cname_type = "CNAME".to_string();
        let expected_mx_type = "MX".to_string();
        let expected_rr_types = vec![
            expected_soa_type.clone(),
            expected_ns_type.clone(),
            expected_ns_type.clone(),
            expected_ns_type.clone(),
            expected_a_type.clone(),
            expected_a_type.clone(),
            expected_cname_type.clone(),
            expected_mx_type.clone(),
        ];

        // Expected classes
        let expected_in_class = "IN".to_string();
        let expected_rr_classes = vec![
            expected_in_class.clone(),
            expected_in_class.clone(),
            expected_in_class.clone(),
            expected_in_class.clone(),
            expected_in_class.clone(),
            expected_in_class.clone(),
            expected_in_class.clone(),
            expected_in_class.clone(),
        ];

        // Loop to test different values
        let mut count = 0;
        let total = lines.len() - 1;
        while count <= total {
            let line = lines.get(count).unwrap().clone();
            let (rr_class, rr_type, _rr_host_name) = master_file.process_line_rr(line.clone());

            let expected_rr_class = expected_rr_classes.get(count).unwrap().clone();
            assert_eq!(rr_class, expected_rr_class);

            let expected_rr_types = expected_rr_types.get(count).unwrap().clone();
            assert_eq!(rr_type, expected_rr_types);

            count += 1;
        }
    }

    #[test]
    fn process_line_origin() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let line_origin = "$ORIGIN test.cl".to_string();
        let vect_lines = vec![line_origin];

        master_file.process_lines(vect_lines, true);

        let desired_sintaxis = "test.cl.".to_string();

        assert_eq!(master_file.get_origin(), desired_sintaxis);
        assert_eq!(master_file.get_last_host(), desired_sintaxis);
    }

    #[test]
    fn process_lines_include() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1 = MasterFile::new(origin.clone());
        let mut master_file_2 = MasterFile::new(origin.clone());

        let soa_line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let line_include = "$INCLUDE include.txt test.com".to_string();
        let vect_lines_1 = vec![soa_line.clone(), line_include];
        master_file_1.process_lines(vect_lines_1, true);

        let file_name_example = "include.txt".to_string();
        let domain_name_example = "test.com".to_string();

        let vect_lines_2 = vec![soa_line];
        master_file_2.process_lines(vect_lines_2, true);
        master_file_2.process_include(file_name_example, domain_name_example, true);

        assert_eq!(master_file_1.get_rrs(), master_file_2.get_rrs());
    }

    #[test]
    fn process_lines_soa_existence() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin);

        let line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let vect_lines = vec![line];

        master_file_1.process_lines(vect_lines, true);
    }

    #[test]
    #[should_panic(expected = "No SOA RR is present at the top of the zone.")]
    fn process_lines_soa_panic() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin);

        let line = "EDU.   IN       NS    SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let vect_lines = vec![line];

        master_file_1.process_lines(vect_lines, true);
    }

    #[test]
    #[should_panic(expected = "Error: host name is not valid")]
    fn process_lines_host_validation_panic() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin);

        let soa_line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let hostname_line = "343ff% NS test.cl ".to_string();
        let vect_lines = vec![soa_line, hostname_line];

        master_file_1.process_lines(vect_lines, true);
    }

    #[test]
    #[should_panic(expected = "More than one SOA per zone.")]
    fn process_lines_many_soa_panic() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin);

        let soa_line_1 = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let soa_line_2 =
            "@   IN        SOA     VENERA      Action\\.domains 20   7200  600 3600000  60"
                .to_string();
        let vect_lines = vec![soa_line_1, soa_line_2];

        master_file_1.process_lines(vect_lines, true);
    }

    #[test]
    #[should_panic(expected = "Not all rr have the same class.")]
    fn process_lines_many_rr_class_panic() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin);

        let soa_line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let line_2 = " CS           NS     A.ISI.EDU.".to_string();
        let vect_lines = vec![soa_line, line_2];

        master_file_1.process_lines(vect_lines, true);
    }

    #[test]
    #[should_panic(expected = "CNAME loop detected!")]
    fn check_cname_loop_fail() {
        // Is necessary to add SOA line.
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let origin = "uchile.cl.".to_string();

        let line1_cname = "alias1 3600 CNAME alias2.uchile.cl.".to_string();
        let line2_cname = "alias2 3600 CNAME alias1".to_string();
        let lines = vec![line_soa, line1_cname, line2_cname];

        let mut master_file = MasterFile::new(origin);
        master_file.process_lines(lines, true);
        master_file.check_cname_loop();
    }

    #[test]
    #[should_panic(expected = "CNAME loop detected!")]
    fn check_cname_loop_type3_fail() {
        // Is necessary to add SOA line.
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let origin = "uchile.cl.".to_string();

        // Type 3 CNAME loop of 3 aliases pointing to each other
        let line1_cname = "alias1 3600 CNAME alias2".to_string();
        let line2_cname = "alias2 3600 CNAME alias3".to_string();
        let line3_cname = "alias3 3600 CNAME alias1".to_string();
        let lines = vec![line_soa, line1_cname, line2_cname, line3_cname];

        let mut master_file = MasterFile::new(origin);
        master_file.process_lines(lines, true);
        master_file.check_cname_loop();
    }

    #[test]
    #[should_panic(expected = "CNAME loop detected!")]
    fn check_cname_loop_type4_fail() {
        // Is necessary to add SOA line.
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let origin = "uchile.cl.".to_string();

        // Type 3 CNAME loop of 3 aliases pointing to each other
        let line1_cname = "alias1 3600 CNAME alias2".to_string();
        let line2_cname = "alias2 3600 CNAME alias3".to_string();
        let line3_cname = "alias3 3600 CNAME alias4".to_string();
        let line4_cname = "alias4 3600 CNAME alias1".to_string();
        let lines = vec![line_soa, line1_cname, line2_cname, line3_cname, line4_cname];

        let mut master_file = MasterFile::new(origin);
        master_file.process_lines(lines, true);
        master_file.check_cname_loop();
    }

    #[test]
    fn process_specific_rr_a() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let items = "26.3.0.103".split_whitespace();
        let ttl = 0;
        let class = "HS".to_string();
        let rr_type = "A".to_string();

        let a_example = ARdata::rr_from_master_file(items.clone(), ttl, 4, origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone();

        assert_eq!(wanted_rr, a_example);
    }

    #[test]
    fn process_specific_rr_ach() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let items = "A.ISI.EDU. 26".split_whitespace();
        let ttl = 0;
        let class = "CH".to_string();
        let rr_type = "A".to_string();

        let ach_example =
            AChRdata::rr_from_master_file(items.clone(), ttl, 3, origin.clone(), origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone();

        assert_eq!(wanted_rr, ach_example);
    }

    #[test]
    fn process_specific_rr_ns() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let items = "A.ISI.EDU.".split_whitespace();
        let ttl = 0;
        let class = "HS".to_string();
        let rr_type = "NS".to_string();
        let host_example = "ICS.UCI".to_string();

        //Create ns type rr
        let ns_example = NsRdata::rr_from_master_file(
            items.clone(),
            ttl,
            4,
            host_example.clone(),
            origin.clone(),
        );

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, host_example);
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("ICS.UCI").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone();

        assert_eq!(wanted_rr, ns_example);
    }

    #[test]
    fn process_specific_rr_cname() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let items = "C.ISI.EDU".split_whitespace();
        let ttl = 0;
        let class = "CS".to_string();
        let rr_type = "CNAME".to_string();

        let cname_example =
            CnameRdata::rr_from_master_file(items.clone(), ttl, 2, origin.clone(), origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone(); //use:*

        assert_eq!(wanted_rr, cname_example);
    }

    #[test]
    fn process_specific_rr_soa() {
        let origin = "uchile.cl".to_string();
        let mut master_file = MasterFile::new(origin.clone());

        let items = "VENERA      Action.domains	20	7200	600	3600000	60".split_whitespace();
        let ttl = 0;
        let class = "CH".to_string();
        let rr_type = "SOA".to_string();

        let (soa_example, number_example) =
            SoaRdata::rr_from_master_file(items.clone(), ttl, 3, origin.clone(), origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class.clone(), rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone(); //use:*

        assert_eq!(wanted_rr, soa_example);
        assert_eq!(master_file.get_ttl_default(), number_example);
        assert_eq!(master_file.get_class_default(), class);
        assert_eq!(master_file.get_top_host(), origin.clone());
    }

    #[test]
    fn process_specific_rr_ptr() {
        let origin = "uchile.cl".to_string();
        let mut master_file = MasterFile::new(origin.clone());

        let items = "SRI-NIC.ARPA.".split_whitespace();
        let ttl = 0;
        let class = "CH".to_string();
        let rr_type = "PTR".to_string();

        let ptr_example =
            PtrRdata::rr_from_master_file(items.clone(), ttl, 3, origin.clone(), origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone(); //use:*

        assert_eq!(wanted_rr, ptr_example);
    }

    #[test]
    fn process_specific_rr_hinfo() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let items = "DEC-2060 TOPS20".split_whitespace();
        let ttl = 0;
        let class = "IN".to_string();
        let rr_type = "HINFO".to_string();

        let hinfo_example = HinfoRdata::rr_from_master_file(items.clone(), ttl, 1, origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone(); //use:*

        assert_eq!(wanted_rr, hinfo_example);
    }

    #[test]
    fn process_specific_rr_mx() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let items = "10 ACC.ARPA.".split_whitespace();
        let ttl = 0;
        let class = "IN".to_string();
        let rr_type = "MX".to_string();

        let mx_example =
            MxRdata::rr_from_master_file(items.clone(), ttl, 1, origin.clone(), origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone(); //use:*

        assert_eq!(wanted_rr, mx_example);
    }

    #[test]
    fn process_specific_rr_txt() {
        let origin = "uchile.cl".to_string();
        let mut master_file = MasterFile::new(origin.clone());

        let items = "esto es un texto n.n".split_whitespace();
        let ttl = 0;
        let class = "HS".to_string();
        let rr_type = "TXT".to_string();

        let txt_example = TxtRdata::rr_from_master_file(items.clone(), ttl, 4, origin.clone());

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
        let master_rrs = master_file.get_rrs();
        let wanted_vector = master_rrs.get("uchile.cl").unwrap();
        let wanted_rr = wanted_vector.get(0).unwrap().clone(); //use:*

        assert_eq!(wanted_rr, txt_example);
    }

    #[test]
    #[should_panic]
    fn process_specific_rr_type_panic() {
        let origin = "uchile.cl".to_string();
        let mut master_file = MasterFile::new(origin.clone());

        let items = "this will panic".split_whitespace();
        let ttl = 0;
        let class = "HS".to_string();
        //Non existing type
        let rr_type = "RANDOM".to_string();

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
    }

    #[test]
    #[should_panic]
    fn process_specific_rr_class_panic() {
        let origin = "uchile.cl".to_string();
        let mut master_file = MasterFile::new(origin.clone());

        let items = "this will panic".split_whitespace();
        let ttl = 0;
        //Non existing class
        let class = "RANDOM".to_string();
        let rr_type = "TXT".to_string();

        master_file.process_specific_rr(items.clone(), ttl, class, rr_type, origin.clone());
    }

    #[test]
    fn remove_comments() {
        // Different cases of lines with or without comments
        let line_with_comment = "dcc  A  192.80.24.11 ; this is a ; line with comments".to_string();
        let line_without_rr = ";line with no RR".to_string();
        let line_without_comment = "uchile.cl A 136.7   ".to_string();
        let line_bad_syntax_comment = "uchile.cl A 136.7   ; *&(*%".to_string();
        let line_empty_comment = "uchile.cl A 136.7   ;".to_string();

        let lines = vec![
            line_with_comment,
            line_without_rr,
            line_without_comment,
            line_bad_syntax_comment,
            line_empty_comment,
        ];

        // Expected lines without comments according to each give case
        let expected_1 = "dcc  A  192.80.24.11 ";
        let expected_2 = "";
        let expected_3 = "uchile.cl A 136.7   ";
        let expected_4 = expected_3.clone();
        let expected_5 = expected_3.clone();

        let expected_lines = vec![expected_1, expected_2, expected_3, expected_4, expected_5];

        let mut count = 0;
        let total = lines.len() - 1;
        while count <= total {
            let line = lines.get(count).unwrap().clone();
            let line_without_comment = MasterFile::remove_comments(line);
            let expected = expected_lines.get(count).unwrap().clone();
            assert_eq!(line_without_comment, expected);
            count += 1;
        }
    }

    #[test]
    fn replace_special_encoding() {
        let line_1 = "a  IN  SOA VENERA  Action\\.domains 20 7200 600 3600000 60".to_string();
        let line_2 = "a  IN  SOA VENERA  Action.domains 20 7200 \\600 3600000 60".to_string();
        let line_3 = "\\@  IN  A 123.123.123.123".to_string();

        let line_1_without_special_encoding = MasterFile::replace_special_encoding(line_1);
        let line_2_without_special_encoding = MasterFile::replace_special_encoding(line_2);
        let line_3_without_special_encoding = MasterFile::replace_special_encoding(line_3);

        let expected_line_1 =
            "a  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let expected_line_2 =
            "a  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let expected_line_3 = "@  IN  A 123.123.123.123".to_string();

        assert_eq!(line_1_without_special_encoding, expected_line_1);
        assert_eq!(line_2_without_special_encoding, expected_line_2);
        assert_eq!(line_3_without_special_encoding, expected_line_3);
    }

    #[test]
    fn get_full_host_name() {
        // Origin and last host name must be set with a dot "." at the end to
        // specify those are absolute domain names. The addition of the dot
        // to these fields is not tested here.
        let origin = "uchile.cl.".to_string();
        let last_host = "test.uchile.cl.".to_string();
        let mut master_file = MasterFile::new(origin.clone());
        master_file.set_last_host(last_host.clone());

        // Different host names are given according to the following cases:
        // -    no host name is given
        // -    relative host name is given
        // -    full host name is given
        let line_1_no_host = "  A   192.168.1.1".to_string();
        let line_2_relative_host = "a NS  VENERA".to_string();
        let line_3_no_host = "  A   192.168.1.1".to_string();
        let line_4_full_host = "b.uchile.cl.  A   192.168.1.1".to_string();
        let line_5_no_host = "  A   192.168.1.1".to_string();
        let line_6_relative_subdomain = "a.b A 192.168.23.2".to_string();
        let line_7_wildcard = "*.b A 192.168.44.6".to_string();
        let line_8_wildcard = "*   A 192.168.44.6".to_string();
        let line_9_full_host = "c. A   136.7.7".to_string();

        let lines = vec![
            line_1_no_host,
            line_2_relative_host,
            line_3_no_host,
            line_4_full_host,
            line_5_no_host,
            line_6_relative_subdomain,
            line_7_wildcard,
            line_8_wildcard,
            line_9_full_host,
        ];

        // Expected full host names according to each case.
        let expected_1 = "test.uchile.cl.".to_string();
        let expected_2 = "a.uchile.cl.".to_string();
        let expected_3 = expected_2.clone();
        let expected_4 = "b.uchile.cl.".to_string();
        let expected_5 = expected_4.clone();
        let expected_6 = "a.b.uchile.cl.".to_string();
        let expected_7 = "*.b.uchile.cl.".to_string();
        let expected_8 = "*.uchile.cl.".to_string();
        let expected_9 = "c.".to_string();

        let expected_lines = vec![
            expected_1, expected_2, expected_3, expected_4, expected_5, expected_6, expected_7,
            expected_8, expected_9,
        ];

        let mut count = 0;
        let total = lines.len() - 1;
        while count <= total {
            let line = lines.get(count).unwrap().clone();
            let full_host = master_file.get_full_host_name(line).0;
            let expected = expected_lines.get(count).unwrap().clone();
            assert_eq!(full_host, expected);

            count += 1;
        }
    }

    #[test]
    fn get_value_type() {
        let master_file: MasterFile = MasterFile::new("uchile.cl".to_string());

        let line_class_1 = String::from("IN");
        let line_class_2 = String::from("CH");

        let line_type_1 = String::from("CNAME");
        let line_type_2 = String::from("TXT");

        let line_ttl_1 = String::from("604800");
        let line_ttl_2 = String::from("0");

        let line_other = String::from("SHOULD_BE_FIXED_TEST");

        assert_eq!(master_file.get_value_type(line_class_1), 1);
        assert_eq!(master_file.get_value_type(line_class_2), 1);
        assert_eq!(master_file.get_value_type(line_type_1), 2);
        assert_eq!(master_file.get_value_type(line_type_2), 2);
        assert_eq!(master_file.get_value_type(line_ttl_1), 0);
        assert_eq!(master_file.get_value_type(line_ttl_2), 0);
        assert_eq!(master_file.get_value_type(line_other), 0);
    }

    #[test]
    fn add_rr() {
        // Given a full host name and a RR, it adds the new RR to the master
        // file parsing. We assume that the given domain host name is absolute.
        // Test adding RRs of different types and different RRs.
        let origin = "uchile.cl.".to_string();

        // A type RR of a subdomain of a zone
        let host_rr_a_subdomain = "a.b.uchile.cl.".to_string();
        let rr_a_subdomain = ARdata::rr_from_master_file(
            "204.13.100.3".split_whitespace(),
            0,
            0,
            host_rr_a_subdomain.clone(),
        );
        let expected_rr_a_subdomain = rr_a_subdomain.clone();

        // Wild card
        let host_rr_ns_wildcard = "*.uchile.cl.".to_string();
        let rr_ns_wildcard = NsRdata::rr_from_master_file(
            "a".split_whitespace(),
            0,
            0,
            host_rr_ns_wildcard.clone(),
            "uchile.cl".to_string(),
        );
        let expected_ns_wildcard = rr_ns_wildcard.clone();

        // HINFO type RR of a different host name
        let host_rr_hinfo = "b.".to_string();
        let rr_hinfo = HinfoRdata::rr_from_master_file(
            "DEC-2060 TOPS20".split_whitespace(),
            0,
            1,
            host_rr_hinfo.clone(),
        );
        let expected_rr_hinfo = rr_hinfo.clone();

        // MX type RR
        let host_rr_mx = "c.uchile.cl.".to_string();
        let rr_mx = MxRdata::rr_from_master_file(
            "10 ACC.ARPA.".split_whitespace(),
            0,
            1,
            host_rr_mx.clone(),
            origin.clone(),
        );
        let expected_rr_mx = rr_mx.clone();

        // PTR type RR
        let host_rr_ptr = "dcc.cl.".to_string();
        let rr_ptr = PtrRdata::rr_from_master_file(
            "SRI-NIC.ARPA.".split_whitespace(),
            0,
            3,
            host_rr_ptr.clone(),
            origin.clone(),
        );
        let expected_rr_ptr = rr_ptr.clone();

        // SOA type RR
        let host_rr_soa = origin.clone();
        let rr_soa = SoaRdata::rr_from_master_file(
            "VENERA      Action.domains	20	7200	600	3600000	60".split_whitespace(),
            0,
            3,
            host_rr_soa.clone(),
            origin.clone(),
        )
        .0;
        let expected_rr_soa = rr_soa.clone();

        let mut master_file = MasterFile::new(origin);

        let hosts = vec![
            host_rr_a_subdomain.clone(),
            host_rr_ns_wildcard.clone(),
            host_rr_hinfo.clone(),
            host_rr_mx.clone(),
            host_rr_ptr.clone(),
            host_rr_soa.clone(),
        ];

        let rrs = vec![
            rr_a_subdomain.clone(),
            rr_ns_wildcard.clone(),
            rr_hinfo.clone(),
            rr_mx.clone(),
            rr_ptr.clone(),
            rr_soa.clone(),
        ];

        let expected_rrs = vec![
            expected_rr_a_subdomain.clone(),
            expected_ns_wildcard.clone(),
            expected_rr_hinfo.clone(),
            expected_rr_mx.clone(),
            expected_rr_ptr.clone(),
            expected_rr_soa.clone(),
        ];

        let mut count = 0;
        let total = hosts.len() - 1;
        while count <= total {
            let host = hosts.get(count).unwrap().clone();
            let rr = rrs.get(count).unwrap().clone();
            master_file.add_rr(host.clone(), rr.clone());

            let expected = expected_rrs.get(count).unwrap().clone();

            assert_eq!(master_file.get_rrs().get(&host.clone()).unwrap().len(), 1);
            assert_eq!(
                master_file
                    .get_rrs()
                    .get(&host.clone())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .clone(),
                expected
            );
            count += 1;
        }
    }

    #[test]
    fn add_rr_cname() {
        // If a CNAME RR is present at a node, no other data should be present; this ensures that
        // the data for a canonical name and its aliases cannot be different.
        let origin = "uchile.cl.".to_string();
        let mut master_file = MasterFile::new(origin.clone());

        // Create a single RR of type A, which will be associated to the canonical or primary name
        let primary_host_name = "test.uchile.cl.".to_string();
        let rr_a = ARdata::rr_from_master_file(
            "204.13.100.3".split_whitespace(),
            0,
            0,
            primary_host_name.clone(),
        );
        let expected_rr_a = rr_a.clone();

        // Create a RR of type CNAME, which will associated to an alias
        let alias1_host_name = "alias1.googleplex.edu".to_string();
        let rr_cname_1 = CnameRdata::rr_from_master_file(
            "test.uchile.cl.".split_whitespace(),
            0,
            0,
            alias1_host_name.clone(),
            origin.clone(),
        );
        let expected_rr_cname_1 = rr_cname_1.clone();

        // Create a RR of type CNAME, which will point to another alias
        let alias2_host_name = "alias2.googleplex.edu".to_string();
        let rr_cname_2 = CnameRdata::rr_from_master_file(
            "alias1.googleplex.edu".split_whitespace(),
            0,
            0,
            alias2_host_name.clone(),
            "test.uchile.cl.".to_string(),
        );
        let expected_rr_cname_2 = rr_cname_2.clone();

        master_file.add_rr(primary_host_name.clone(), rr_a.clone());
        master_file.add_rr(alias1_host_name.clone(), rr_cname_1.clone());
        master_file.add_rr(alias2_host_name.clone(), rr_cname_2.clone());

        assert_eq!(
            master_file
                .get_rrs()
                .get(&primary_host_name.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&primary_host_name.clone())
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
            expected_rr_a
        );

        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias1_host_name.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias1_host_name.clone())
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
            expected_rr_cname_1
        );

        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias2_host_name.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias2_host_name.clone())
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
            expected_rr_cname_2
        );

        // Add another RR to the alias host name
        let rr_a_2 = ARdata::rr_from_master_file(
            "204.13.100.3".split_whitespace(),
            0,
            0,
            alias1_host_name.clone(),
        );

        let rr_a_3 = ARdata::rr_from_master_file(
            "105.13.106.7".split_whitespace(),
            0,
            0,
            alias2_host_name.clone(),
        );

        master_file.add_rr(alias1_host_name.clone(), rr_a_2.clone());
        master_file.add_rr(alias2_host_name.clone(), rr_a_3.clone());

        // No RR should've been added to the aliases host name
        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias1_host_name.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias2_host_name.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias1_host_name.clone())
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
            expected_rr_cname_1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&alias2_host_name.clone())
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
            expected_rr_cname_2
        );

        // Try to add a CNAME RR to an already existing host name
        master_file.add_rr(primary_host_name.clone(), rr_cname_1.clone());
        assert_eq!(
            master_file
                .get_rrs()
                .get(&primary_host_name.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            master_file
                .get_rrs()
                .get(&primary_host_name.clone())
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
            expected_rr_cname_1
        );
    }

    #[test]
    fn process_include() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin.clone());

        let file_name_test = "include.txt".to_string();
        let domain_name_test = "uchile.cl".to_string();
        let validity_test = true;

        let soa_line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let vect_lines = vec![soa_line];
        master_file.process_lines(vect_lines, validity_test);

        master_file.process_include(file_name_test, domain_name_test, false);

        let expected_last_host = "ns2.example.com.".to_string();

        assert_eq!(master_file.get_last_host(), expected_last_host);
        assert_eq!(master_file.get_origin(), origin);
    }

    #[test]
    #[should_panic(expected = "file not found!")]
    fn process_include_no_file_panic() {
        let origin = "uchile.cl".to_string();

        //panic when the file is not in the directory
        let mut master_file = MasterFile::new(origin.clone());
        let file_name_test = "random.txt".to_string();
        let domain_name_test = "uchile.cl".to_string();
        let validity_test = true;

        master_file.process_include(file_name_test, domain_name_test, validity_test);
    }

    #[test]
    fn check_glue_delegations() {
        let origin = "uchile.cl.".to_string();
        let mut master_files = HashMap::new();

        // This function shouldn't panic in the following cases:
        // 1.   The RR of NS type given corresponds to a subdomain and has a glue record associated
        //      with its respective address.
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let line_ns_is_subdomain = "dcc  NS ns.uchile.cl.".to_string();
        let line_glue_record = "ns.uchile.cl.  A  192.80.24.11".to_string();

        let vec_lines_1 = vec![
            line_soa.clone(),
            line_ns_is_subdomain.clone(),
            line_glue_record.clone(),
        ];
        let master_file_1 = MasterFile::new(origin.clone());
        master_files.insert(vec_lines_1, master_file_1);

        // 2.   The RR of NS type is associated with the top host of the zone, in this case, the origin.
        let line_ns_top_host = "@  NS ns.uchile.cl.".to_string();

        let vec_lines_2 = vec![line_soa.clone(), line_ns_top_host.clone()];
        let master_file_2 = MasterFile::new(origin.clone());
        master_files.insert(vec_lines_2, master_file_2);

        // 3.   The RR of NS type doesn't corresponds to a subdomain of the current domain of origin.
        let line_ns_not_subdomain = "dcc  NS ns.uc.cl.".to_string();

        let vec_lines_3 = vec![line_soa.clone(), line_ns_not_subdomain.clone()];
        let master_file_3 = MasterFile::new(origin.clone());
        master_files.insert(vec_lines_3, master_file_3);

        for (vec_lines, mut master_file) in master_files {
            master_file.process_lines(vec_lines, true);
            master_file.check_glue_delegations();
        }
    }

    #[test]
    #[should_panic(
        expected = "Information outside authoritative node in the zone is not glue information."
    )]
    fn check_glue_delegations_fail() {
        // This function should panic when the RR of type NS given which corresponds to a subdomain,
        // doesn't have a glue record associated with its respective address.
        let line_soa = "@  IN  SOA VENERA  Action.domains 20 7200 600 3600000 60".to_string();
        let line_ns_sub_domain = "dcc  NS ns.uchile.cl.".to_string();

        let vec_lines = vec![line_soa.clone(), line_ns_sub_domain.clone()];

        let origin = "uchile.cl.".to_string();
        let mut master_file = MasterFile::new(origin);

        master_file.process_lines(vec_lines, true);
        master_file.check_glue_delegations();
    }

    #[test]
    #[should_panic(expected = "Error: No NS RR at top of the zone")]
    fn check_existence_top_ns_panic() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin);

        let line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA.   870729  1800 300  604800 86400".to_string();
        let vect_lines = vec![line];

        master_file_1.process_lines(vect_lines, true);
        master_file_1.check_existence_top_ns();
    }

    #[test]
    fn check_existence_top_ns() {
        let origin = "uchile.cl".to_string();

        let mut master_file_1: MasterFile = MasterFile::new(origin.clone());
        let mut master_file_2 = MasterFile::new(origin.clone());

        let edu_soa_line = "EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA. 
            870729
            1800   
            300   
            604800
            86400"
            .to_string();

        let edu_ns_line_1 = "EDU. NS      SRI-NIC.ARPA.".to_string();
        let edu_ns_line_2 = "EDU. NS      C.ISI.EDU.".to_string();

        let root_soa_line = ".       IN      SOA     SRI-NIC.ARPA. HOSTMASTER.SRI-NIC.ARPA.
            870611
            1800
            300 
            604800
            86400"
            .to_string();

        let root_ns_line_1 = ".  NS      A.ISI.EDU.".to_string();
        let root_ns_line_2 = ".    NS      C.ISI.EDU.".to_string();
        let root_ns_line_3 = ".    NS      SRI-NIC.ARPA.".to_string();

        let vect_edu_lines = vec![edu_soa_line, edu_ns_line_1, edu_ns_line_2];
        let vect_root_lines = vec![
            root_soa_line,
            root_ns_line_1,
            root_ns_line_2,
            root_ns_line_3,
        ];

        master_file_1.process_lines(vect_edu_lines, true);
        master_file_1.check_existence_top_ns();

        master_file_2.process_lines(vect_root_lines, true);
        master_file_2.check_existence_top_ns();
    }

    #[test]
    fn host_name_master_file_validation() {
        // Test for 5 possible cases
        let host_name_relative_domain = "anakena.dcc".to_string();
        let host_name_absolute_domain = "dcc.".to_string();
        let host_root = '.'.to_string();
        let host_wildcard = "*.dcc".to_string();
        let host_reverse = "1.2.168.192.IN-ADDR.ARPA".to_string();

        let vec_host_names = vec![
            host_name_relative_domain.clone(),
            host_name_absolute_domain.clone(),
            host_root.clone(),
            host_wildcard.clone(),
            host_reverse.clone(),
        ];

        let origin = "uchile.cl".to_string();
        let master_file = MasterFile::new(origin);

        for host in vec_host_names {
            let result = master_file.host_name_master_file_validation(host.clone());

            if host_wildcard == host {
                assert_eq!(Ok("dcc".to_string()), result);
            } else if host_reverse == host {
                assert_eq!(Ok("IN-ADDR.ARPA".to_string()), result);
            } else {
                assert_eq!(Ok(host), result);
            }
        }
    }

    #[test]
    fn host_name_master_file_validation_error() {
        // Test cases for bad syntax
        let host_name_empty_1 = "uchile..cl".to_string();
        let host_name_empty_2 = ".dcc".to_string();
        let host_name_syntax = "uchile.&#(*&.dcc.cl".to_string();

        let vec_host_names = vec![
            host_name_empty_1.clone(),
            host_name_empty_2.clone(),
            host_name_syntax.clone(),
        ];

        let origin = "uchile.cl".to_string();
        let master_file = MasterFile::new(origin);

        for host in vec_host_names {
            let result = master_file.host_name_master_file_validation(host.clone());

            if (host == host_name_empty_1) || (host == host_name_empty_2) {
                assert_eq!(
                    result,
                    Err("Error: Empty label is only allowed at the end of a hostname.")
                )
            } else {
                assert_eq!(
                    result,
                    Err("Error: present domain name is not syntactically correct.")
                );
            }
        }
    }

    #[test]
    fn set_and_get_top_host() {
        // Create a master file
        let origin = "cl".to_string();
        let mut master_file = MasterFile::new(origin);

        // Test default value
        let expected_default = "".to_string();
        assert_eq!(master_file.get_top_host(), expected_default);

        // Test with "dcc" as top host
        let top_host = "dcc".to_string();
        master_file.set_top_host(top_host);
        let expected_top_host = "dcc".to_string();
        assert_eq!(master_file.get_top_host(), expected_top_host);

        // Test adding a different host
        let top_host = "example".to_string();
        master_file.set_top_host(top_host);
        let expected_top_host = "example".to_string();
        assert_eq!(master_file.get_top_host(), expected_top_host);
    }

    #[test]
    fn set_and_get_origin() {
        // Create a master file
        let origin = "".to_string();
        let mut master_file = MasterFile::new(origin);

        // Test default value
        let expected_default = "".to_string();
        assert_eq!(master_file.get_origin(), expected_default);

        // Test setting "dcc" as origin
        let origin_domain = "dcc".to_string();
        let expected_domain = "dcc".to_string();
        master_file.set_origin(origin_domain);
        assert_eq!(master_file.get_origin(), expected_domain);
    }

    #[test]
    fn set_and_get_rrs() {
        // Create a master file
        let origin = "cl".to_string();
        let mut master_file = MasterFile::new(origin);

        // Test default value
        let expected_hash_rr = HashMap::<String, Vec<ResourceRecord>>::new();
        assert_eq!(master_file.get_rrs(), expected_hash_rr);

        // Create A type RR
        let a_rdata = ARdata::new();
        let rdata = Rdata::SomeARdata(a_rdata);
        let rr = ResourceRecord::new(rdata);
        let mut rrs: Vec<ResourceRecord> = Vec::new();
        rrs.push(rr);

        // Create RRs with no domain
        let mut hash_rr = HashMap::<String, Vec<ResourceRecord>>::new();
        hash_rr.insert("".to_string(), rrs.clone());

        let mut expected_hash_rr = hash_rr.clone();

        // Test before set
        assert_ne!(master_file.get_rrs(), expected_hash_rr);

        // Test after set
        master_file.set_rrs(hash_rr.clone());
        assert_eq!(master_file.get_rrs(), expected_hash_rr);

        // Create MX type RR
        let mx_rdata = MxRdata::new();
        let rdata = Rdata::SomeMxRdata(mx_rdata);
        let rr = ResourceRecord::new(rdata);
        rrs.push(rr);
        assert_eq!(rrs.len(), 2);

        // Insert with "dcc" as domain
        let domain = "dcc".to_string();
        hash_rr.insert(domain, rrs.clone());
        assert_eq!(hash_rr.len(), 2);
        master_file.set_rrs(hash_rr.clone());
        assert_ne!(master_file.get_rrs(), expected_hash_rr);

        // Test after set
        expected_hash_rr.insert("dcc".to_string(), rrs.clone());
        assert_eq!(master_file.get_rrs(), expected_hash_rr);

        // Test with different domains
        let domain_1 = "example1".to_string();
        let domain_2 = "example2".to_string();
        hash_rr.insert(domain_1, rrs.clone());
        expected_hash_rr.insert(domain_2, rrs.clone());
        assert_ne!(master_file.get_rrs(), expected_hash_rr);
        master_file.set_rrs(hash_rr.clone());
        assert_ne!(master_file.get_rrs(), expected_hash_rr);
    }

    #[test]
    fn set_and_get_last_host() {
        let origin = "uchile.cl".to_string();

        let mut master_file = MasterFile::new(origin);

        let default_last_host = "".to_string();

        let new_last_host: String = "A.ISI.EDU".to_string();

        assert_eq!(master_file.get_last_host(), default_last_host);
        master_file.set_last_host(new_last_host.clone());
        assert_eq!(master_file.get_last_host(), new_last_host);
    }

    #[test]
    fn set_and_get_class_default() {
        let origin = "uchile.cl".to_string();

        let mut master_file: MasterFile = MasterFile::new(origin);

        let default_class = "".to_string();

        let new_class = "IN".to_string();

        assert_eq!(master_file.get_class_default(), default_class);
        master_file.set_class_default(new_class.clone());
        assert_eq!(master_file.get_class_default(), new_class);
    }

    #[test]
    fn set_and_get_ttl_default() {
        let origin = "uchile.cl".to_string();

        let mut master_file: MasterFile = MasterFile::new(origin);

        let ttl_example = 604800 as u32;
        let default_ttl_example = 0 as u32;

        assert_eq!(master_file.get_ttl_default(), default_ttl_example);
        master_file.set_ttl_default(ttl_example);
        assert_eq!(master_file.get_ttl_default(), ttl_example);
    }
}
