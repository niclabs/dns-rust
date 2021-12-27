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
            ns_zone.add_node(key.clone(), value.clone());
        }

        ns_zone
    }

    pub fn exist_child(&self, name: String) -> bool {
        let childs = self.get_childs();

        for child in childs {
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
