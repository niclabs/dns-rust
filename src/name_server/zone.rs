use crate::message::resource_record::ResourceRecord;
use crate::name_server::master_file::MasterFile;

#[derive(Clone)]
/// Structs that represents data from a zone
pub struct NSZone {
    name: String,
    value: Vec<ResourceRecord>,
    childs: Vec<NSZone>,
    subzone: bool,
    glue_rrs: Vec<ResourceRecord>,
}

impl NSZone {
    pub fn new() -> Self {
        let ns_zone = NSZone {
            name: "".to_string(),
            value: Vec::<ResourceRecord>::new(),
            childs: Vec::<NSZone>::new(),
            subzone: false,
            glue_rrs: Vec::<ResourceRecord>::new(),
        };

        ns_zone
    }

    pub fn from_file(file_name: String) -> Self {
        let master_file_parsed = MasterFile::from_file(file_name);
        let origin = master_file_parsed.get_origin();
        let mut rrs = master_file_parsed.get_rrs();

        let origin_rrs = rrs.remove(&origin).unwrap();

        let mut ns_zone = NSZone::new();
        ns_zone.set_name(origin);
        ns_zone.set_value(origin_rrs);

        for (key, value) in rrs.iter() {
            println!("{}", key);
            ns_zone.add_node(key.clone(), value.clone());
        }

        ns_zone
    }

    pub fn exist_child(&self, name: String) -> bool {
        let childs = self.get_childs();

        for child in childs {
            println!("Child name: {}", child.get_name());
            if child.get_name() == name {
                return true;
            }
        }

        return false;
    }

    pub fn get_child(&self, name: String) -> (NSZone, i32) {
        let childs = self.get_childs();

        let mut child_ns = NSZone::new();

        let mut index = 0;

        for child in childs {
            if child.get_name() == name {
                return (child.clone(), index);
            }
            index = index + 1;
        }

        index = -1;

        (child_ns, index)
    }

    fn add_node(&mut self, host_name: String, rrs: Vec<ResourceRecord>) {
        let mut childs = self.get_childs();
        let mut labels: Vec<&str> = host_name.split(".").collect();

        labels.reverse();

        let mut index = 0;

        let label = labels.remove(0);

        let exist_child = self.exist_child(label.to_string());

        if exist_child == true {
            let (mut child, index) = self.get_child(label.to_string());

            if child.get_subzone() == true {
                child.set_glue_rrs(rrs.clone());
            } else {
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
            }

            childs.remove(index as usize);
            childs.push(child);
            self.set_childs(childs);
        } else {
            let mut new_ns_zone = NSZone::new();
            new_ns_zone.set_name(label.to_string());

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

            childs.push(new_ns_zone);
            self.set_childs(childs);
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
        let childs = self.get_childs();

        println!("Name: {}", name);
        println!("Subzone: {}", self.get_subzone());

        for val in values {
            println!("  Type: {}", val.get_type_code());
        }

        for child in childs {
            child.print_zone();
        }
    }

    pub fn get_rrs_by_type(&self, rr_type: u16) -> Vec<ResourceRecord> {
        let rrs = self.get_value();
        let mut rr_by_type = Vec::<ResourceRecord>::new();

        for rr in rrs {
            if rr.get_type_code() == rr_type {
                rr_by_type.push(rr);
            }
        }

        return rr_by_type;
    }
}

// Setter
impl NSZone {
    // Sets the values with a new value
    pub fn set_value(&mut self, value: Vec<ResourceRecord>) {
        self.value = value;
    }

    // Sets the childs with a new value
    pub fn set_childs(&mut self, childs: Vec<NSZone>) {
        self.childs = childs;
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

    // Gets the childs from the node
    pub fn get_childs(&self) -> Vec<NSZone> {
        self.childs.clone()
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

    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use super::NSZone;

    #[test]
    fn constructor_test() {
        let nszone = NSZone::new();

        assert_eq!(nszone.name, String::from(""));
        assert_eq!(nszone.value.len(), 0);
        assert_eq!(nszone.childs.len(), 0);
        assert_eq!(nszone.subzone, false);
        assert_eq!(nszone.glue_rrs.len(), 0);
    }

    // Getters and Setters 
    #[test]
    fn set_and_get_name_test(){
        let mut nszone = NSZone::new();

        assert_eq!(nszone.get_name(), String::from(""));
        nszone.set_name(String::from("test.com"));
        assert_eq!(nszone.get_name(), String::from("test.com"));
    }

    #[test]
    fn set_and_get_value_test(){
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
    fn set_and_get_glue_rr_test(){
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
    fn set_and_get_subzone_test(){
        let mut nszone = NSZone::new();

        assert_eq!(nszone.get_subzone(), false);
        nszone.set_subzone(true);
        assert_eq!(nszone.get_subzone(), true);
    }

    #[test]
    fn set_and_get_childs_test(){
        let mut nszone = NSZone::new();

        let mut childs: Vec<NSZone> = Vec::new();
        let some_nszone = NSZone::new(); 
        childs.push(some_nszone);

        assert_eq!(nszone.get_childs().len(), 0);
        nszone.set_childs(childs);
        assert_eq!(nszone.get_childs().len(), 1);
    }

    // Other methods

    //pub fn from_file(file_name: String) -> Self
   /* 

    #[test]
    fn from_file_test(){
    }*/

    #[test]
    fn exist_child_test(){
        let mut nszone = NSZone::new();
        let mut some_nszone = NSZone::new();
        some_nszone.set_name(String::from("test.com"));

        let mut childs: Vec<NSZone> = Vec::new();
        childs.push(some_nszone);
        nszone.set_childs(childs);
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

        let mut childs: Vec<NSZone> = Vec::new();
        childs.push(some_nszone);
        childs.push(some_other_nszone);
        nszone.set_childs(childs);
        assert_eq!(nszone.get_child(String::from("other.test.com")).0.get_name(), String::from("other.test.com"));
        assert_eq!(nszone.get_child(String::from("other.test.com")).1, 1);

        assert_eq!(nszone.get_child(String::from("some.test.com")).0.get_name(), String::from(""));
        assert_eq!(nszone.get_child(String::from("some.test.com")).1, -1);
    }

    /*#[test]
    fn add_node_test(){
    }*/

    #[test]
    fn check_rrs_only_ns_test(){

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
    fn get_rrs_by_type_test(){
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

