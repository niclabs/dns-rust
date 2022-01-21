use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::name_server::master_file::MasterFile;

#[derive(Clone)]
/// Structs that represents data from a zone
pub struct NSZone {
    name: String,
    // Ip to ask the SOA RR data for refreshing
    ip_address_for_refresh_zone: String,
    value: Vec<ResourceRecord>,
    children: Vec<NSZone>,
    subzone: bool,
    glue_rrs: Vec<ResourceRecord>,
}

impl NSZone {
    pub fn new() -> Self {
        let ns_zone = NSZone {
            name: "".to_string(),
            ip_address_for_refresh_zone: "".to_string(),
            value: Vec::<ResourceRecord>::new(),
            children: Vec::<NSZone>::new(),
            subzone: false,
            glue_rrs: Vec::<ResourceRecord>::new(),
        };

        ns_zone
    }

    pub fn from_file(file_name: String, ip_address_for_refresh_zone: String) -> Self {
        let master_file_parsed = MasterFile::from_file(file_name);
        let origin = master_file_parsed.get_origin();
        let mut rrs = master_file_parsed.get_rrs();

        let origin_rrs = rrs.remove(&origin).unwrap();

        let mut ns_zone = NSZone::new();
        ns_zone.set_name(origin);
        ns_zone.set_ip_address_for_refresh_zone(ip_address_for_refresh_zone);
        ns_zone.set_value(origin_rrs);

        for (key, value) in rrs.iter() {
            println!("{} - {}", key.clone(), value.len());
            ns_zone.add_node(key.clone(), value.clone());
        }

        ns_zone
    }

    pub fn from_axfr_msg(msg: DnsMessage) -> Self {
        let answers = msg.get_answer();
        let mut new_zone = NSZone::new();

        let soa_rr = answers[0].clone();
        let zone_name = soa_rr.get_name().get_name();

        new_zone.set_name(zone_name.clone());

        let mut rr_iter = answers[1..].iter();
        let mut next_rr = rr_iter.next();
        let mut rrs_for_node = Vec::<ResourceRecord>::new();
        let mut actual_node_name = zone_name.clone();

        while next_rr.is_none() == false {
            let rr = next_rr.unwrap();
            let rr_name = rr.get_name().get_name();
            let rr_type = rr.get_type_code();

            // Check if the rr is a SOA for the top node
            if rr_type == 6 && rr_name == zone_name {
                break;
            } else {
                // If the rr name is not the same with last rr, add the node to the zone
                if rr_name != actual_node_name {
                    new_zone.add_node(actual_node_name, rrs_for_node);

                    rrs_for_node = Vec::<ResourceRecord>::new();
                    actual_node_name = rr_name;
                }

                // Add the rr to the vec
                rrs_for_node.push(rr.clone());
                next_rr = rr_iter.next();
            }
        }

        new_zone
    }

    pub fn exist_child(&self, name: String) -> bool {
        let children = self.get_children();

        for child in children {
            println!("Child name: {}", child.get_name());
            if child.get_name() == name {
                return true;
            }
        }

        return false;
    }

    pub fn get_child(&self, name: String) -> (NSZone, i32) {
        let children = self.get_children();

        let mut child_ns = NSZone::new();

        let mut index = 0;

        for child in children {
            if child.get_name() == name {
                return (child.clone(), index);
            }
            index = index + 1;
        }

        index = -1;

        (child_ns, index)
    }

    fn add_node(&mut self, host_name: String, rrs: Vec<ResourceRecord>) {
        let mut children = self.get_children();
        let mut labels: Vec<&str> = host_name.split(".").collect();
        // Check if the total number of octets is 255 or less
        if host_name.len()-labels.len()+1 <= 255 {

            labels.reverse();

            let label = labels.remove(0);

            let exist_child = self.exist_child(label.to_string());

            if exist_child == true {
                let (mut child, index) = self.get_child(label.to_string());

                if labels.len() == 0 {
                    child.set_value(rrs.clone());

                    if self.check_rrs_only_ns(rrs) == true {
                        child.set_subzone(true);
                    }
                } else {
                    let mut new_name = "".to_string();

                    labels.reverse();

                    for label in labels {
                        new_name.push_str(label);
                        new_name.push_str(".");
                    }

                    new_name.pop();

                    child.add_node(new_name, rrs);
                }

                children.remove(index as usize);
                children.push(child);
                self.set_children(children);
            } else if self.check_label_name(label.to_string()) {
                let mut new_ns_zone = NSZone::new();
                new_ns_zone.set_name(label.to_string());

                println!("RRs len: {}", rrs.len());

                if labels.len() == 0 {
                    new_ns_zone.set_value(rrs.clone());

                    if self.check_rrs_only_ns(rrs) == true {
                        new_ns_zone.set_subzone(true);
                    }
                } else {
                    let mut new_name = "".to_string();

                    labels.reverse();

                    for label in labels {
                        new_name.push_str(label);
                        new_name.push_str(".");
                    }

                    new_name.pop();

                    new_ns_zone.add_node(new_name, rrs);
                }

                children.push(new_ns_zone);
                self.set_children(children);
            }
        }
    }

    fn check_rrs_only_ns(&self, rrs: Vec<ResourceRecord>) -> bool {
        for rr in rrs {
            let rr_type = rr.get_type_code();

            if rr_type != 2 {
                return false;
            }
        }

        return true;
    }

    pub fn print_zone(&self) {
        let name = self.get_name();
        let values = self.get_value();
        let children = self.get_children();

        println!("Name: {}", name);
        println!("Subzone: {}", self.get_subzone());

        for val in values {
            println!("  Type: {}", val.get_type_code());
        }

        for child in children {
            child.print_zone();
        }
    }

    pub fn get_rrs_by_type(&self, rr_type: u16) -> Vec<ResourceRecord> {
        let rrs = self.get_value();
        let mut rr_by_type = Vec::<ResourceRecord>::new();

        println!("RRs len zone: {}", rrs.len());

        for rr in rrs {
            if rr.get_type_code() == rr_type {
                rr_by_type.push(rr);
            }
        }

        return rr_by_type;
    }

    pub fn get_all_rrs(&self) -> Vec<ResourceRecord> {
        let mut rrs = self.get_value();
        let children = self.get_children();

        for child in children {
            rrs.append(&mut child.get_all_rrs());
        }

        rrs
    }

    fn check_label_name(&self, name: String) -> bool {
        if name.len() > 63 || name.len() == 0 {
            return false;
        }
        
        for (i, c) in name.chars().enumerate() {
            if i==0 && !c.is_ascii_alphabetic(){
                return false;
            } else if i==name.len()-1 && !c.is_ascii_alphanumeric(){
                return false;
            } else if !(c.is_ascii_alphanumeric() || c=='-') {
                return false;
            }
        }

        return true;
    }
}

// Setter
impl NSZone {
    // Sets the values with a new value
    pub fn set_value(&mut self, value: Vec<ResourceRecord>) {
        self.value = value;
    }

    pub fn set_ip_address_for_refresh_zone(&mut self, ip_address_for_refresh_zone: String) {
        self.ip_address_for_refresh_zone = ip_address_for_refresh_zone;
    }

    // Sets the children with a new value
    pub fn set_children(&mut self, children: Vec<NSZone>) {
        self.children = children;
    }

    // Sets the name with a new value
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // Sets the subzone with a new value
    pub fn set_subzone(&mut self, subzone: bool) {
        self.subzone = subzone;
    }

    // Sets the glue_rrs with a new value
    pub fn set_glue_rrs(&mut self, glue_rrs: Vec<ResourceRecord>) {
        self.glue_rrs = glue_rrs;
    }
}

// Getters
impl NSZone {
    // Gets the values from the node
    pub fn get_value(&self) -> Vec<ResourceRecord> {
        self.value.clone()
    }

    pub fn get_ip_address_for_refresh_zone(&self) -> String {
        self.ip_address_for_refresh_zone.clone()
    }

    // Gets the children from the node
    pub fn get_children(&self) -> Vec<NSZone> {
        self.children.clone()
    }

    // Gets the host name from the node
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // Gets the subzone from the node
    pub fn get_subzone(&self) -> bool {
        self.subzone.clone()
    }

    // Gets the glue rrs from the node
    pub fn get_glue_rrs(&self) -> Vec<ResourceRecord> {
        self.glue_rrs.clone()
    }
}

mod test {
    use crate::name_server::master_file::MasterFile;

    use super::NSZone;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn constructor_test() {
        let nszone = NSZone::new();

        assert_eq!(nszone.name, String::from(""));
        assert_eq!(nszone.value.len(), 0);
        assert_eq!(nszone.children.len(), 0);
        assert_eq!(nszone.subzone, false);
        assert_eq!(nszone.glue_rrs.len(), 0);
    }

    // Getters and Setters
    #[test]
    fn set_and_get_name_test() {
        let mut nszone = NSZone::new();

        assert_eq!(nszone.get_name(), String::from(""));
        nszone.set_name(String::from("test.com"));
        assert_eq!(nszone.get_name(), String::from("test.com"));
    }

    #[test]
    fn set_and_get_value_test() {
        let mut nszone = NSZone::new();

        let mut value: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        value.push(resource_record);

        assert_eq!(nszone.get_value().len(), 0);
        nszone.set_value(value);
        assert_eq!(nszone.get_value().len(), 1);
    }

    #[test]
    fn set_and_get_glue_rr_test() {
        let mut nszone = NSZone::new();

        let mut glue: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        glue.push(resource_record);

        assert_eq!(nszone.get_glue_rrs().len(), 0);
        nszone.set_glue_rrs(glue);
        assert_eq!(nszone.get_glue_rrs().len(), 1);
    }

    #[test]
    fn set_and_get_subzone_test() {
        let mut nszone = NSZone::new();

        assert_eq!(nszone.get_subzone(), false);
        nszone.set_subzone(true);
        assert_eq!(nszone.get_subzone(), true);
    }

    #[test]
    fn set_and_get_children_test() {
        let mut nszone = NSZone::new();

        let mut children: Vec<NSZone> = Vec::new();
        let some_nszone = NSZone::new();
        children.push(some_nszone);

        assert_eq!(nszone.get_children().len(), 0);
        nszone.set_children(children);
        assert_eq!(nszone.get_children().len(), 1);
    }

    // Other methods

    //pub fn from_file(file_name: String) -> Self
    /*

    #[test]
    fn from_file_test(){
    }*/

    #[test]
    fn exist_child_test() {
        let mut nszone = NSZone::new();
        let mut some_nszone = NSZone::new();
        some_nszone.set_name(String::from("test.com"));

        let mut children: Vec<NSZone> = Vec::new();
        children.push(some_nszone);
        nszone.set_children(children);
        assert_eq!(nszone.exist_child(String::from("test2.com")), false);
        assert_eq!(nszone.exist_child(String::from("test.com")), true)
    }

    #[test]
    fn get_child_test() {
        let mut nszone = NSZone::new();
        let mut some_nszone = NSZone::new();
        let mut some_other_nszone = NSZone::new();
        some_nszone.set_name(String::from("test.com"));
        some_other_nszone.set_name(String::from("other.test.com"));

        let mut children: Vec<NSZone> = Vec::new();
        children.push(some_nszone);
        children.push(some_other_nszone);
        nszone.set_children(children);
        assert_eq!(
            nszone
                .get_child(String::from("other.test.com"))
                .0
                .get_name(),
            String::from("other.test.com")
        );
        assert_eq!(nszone.get_child(String::from("other.test.com")).1, 1);

        assert_eq!(
            nszone.get_child(String::from("some.test.com")).0.get_name(),
            String::from("")
        );
        assert_eq!(nszone.get_child(String::from("some.test.com")).1, -1);
    }

    /*#[test]
    fn add_node_test(){
    }*/

    /*#[test]
    fn add_node_test(){ using a wrong domain
    }*/

    #[test]
    fn check_rrs_only_ns_test() {
        let mut nszone = NSZone::new();
        let mut rr: Vec<ResourceRecord> = Vec::new();

        let ns_rdata = Rdata::SomeNsRdata(NsRdata::new());
        let mut resource_record_1 = ResourceRecord::new(ns_rdata);
        resource_record_1.set_type_code(2);

        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let mut resource_record_2 = ResourceRecord::new(a_rdata);
        resource_record_2.set_type_code(1);

        rr.push(resource_record_1);

        assert_eq!(nszone.check_rrs_only_ns(rr.clone()), true);
        rr.push(resource_record_2);
        assert_eq!(nszone.check_rrs_only_ns(rr.clone()), false);
    }
    /*
    #[test]
    fn print_zone_test(){

    }

    */
    #[test]
    fn get_rrs_by_type_test() {
        let mut nszone = NSZone::new();

        let mut value: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let mut resource_record = ResourceRecord::new(a_rdata);
        resource_record.set_type_code(1);
        value.push(resource_record);

        assert_eq!(nszone.get_rrs_by_type(1).len(), 0);
        nszone.set_value(value);
        assert_eq!(nszone.get_rrs_by_type(1).len(), 1);
    }
}
