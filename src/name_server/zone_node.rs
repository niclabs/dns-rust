use crate::message::resource_record::ResourceRecord;

//utils
use crate::utils::check_label_name;

#[derive(Clone)]
/// Recursive Struct that represents data from a zone
pub struct NSNode {
    name: String,
    value: Vec<ResourceRecord>,
    children: Vec<NSNode>,
    subzone: bool,
}

impl NSNode {
    pub fn new() -> Self {
        let ns_node = NSNode {
            name: "".to_string(),
            value: Vec::<ResourceRecord>::new(),
            children: Vec::<NSNode>::new(),
            subzone: false,
        };

        return ns_node;
    }

    pub fn exist_child(&self, name: String) -> bool {
        // case insensitive
        let children = self.get_children();
        let lower_case_name = name.to_ascii_lowercase();

        for child in children {
            println!("Child name: {}", child.get_name());
            let lower_case_child = child.get_name().to_ascii_lowercase();
            if lower_case_child == lower_case_name{
                return true;
            }
        }

        return false;
    }

    pub fn get_child(&self, name: String) -> (NSNode, i32) {
        // case insensitive
        let children = self.get_children();
        let child_ns = NSNode::new();
        let lower_case_name = name.to_ascii_lowercase();
        let mut index = 0;

        for child in children {
            let lower_case_child = child.get_name().to_ascii_lowercase();
            if lower_case_child == lower_case_name {
                return (child.clone(), index);
            }
            index = index + 1;
        }

        index = -1;

        (child_ns, index)
    }

    pub fn add_node(&mut self, host_name: String, rrs: Vec<ResourceRecord>) -> Result<(), &'static str> {
        let mut children = self.get_children();

        let mut host_name = host_name;

        if host_name.ends_with("."){
            host_name.pop();
        }

        // null label is reserved for the root. Children cannot have it. 
        if host_name.len() == 0 {
           return Err("Error: Child cannot have null label, reserved for root only.");
        }
        else {
            let mut labels: Vec<&str> = host_name.split(".").collect();
            // Check if the total number of octets is 255 or less
            if host_name.len() - labels.len() + 1 <= 255 {
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
                } else if check_label_name(label.to_string()) {
                    let mut new_ns_zone = NSNode::new();
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
            return Ok(()); 
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


    pub fn get_rrs_by_type(&self, rr_type: u16) -> Vec<ResourceRecord> {
        let rrs = self.get_value();

        if rr_type == 255 {
            return rrs;
        }

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
}

// Setter
impl NSNode {
    // Sets the values with a new value
    pub fn set_value(&mut self, value: Vec<ResourceRecord>) {
        self.value = value;
    }

    // Sets the children with a new value
    pub fn set_children(&mut self, mut children: Vec<NSNode>) {
        // check if there is duplicates labels, case insensitive
        // checks from the vector's tail to keep the last added child
        let mut labels: Vec<String> = ([]).to_vec();
        let mut lower_case_labels: Vec<String> = ([]).to_vec();
        let mut n = children.len();
        while n > 0 {
            let label = children[n - 1].get_name();
            let lower_case_label = label.to_ascii_lowercase();
            if lower_case_labels.contains(&lower_case_label.clone()) {
                children.remove(n - 1);
                //TODO: add warning
            } else {
                labels.push(label.clone());
                lower_case_labels.push(label.clone().to_ascii_lowercase());
            }
            n -= 1;
        }

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
}

// Getters
impl NSNode {
    // Gets the host name from the node
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // Gets the values from the node
    pub fn get_value(&self) -> Vec<ResourceRecord> {
        self.value.clone()
    }

    // Gets the children from the node
    pub fn get_children(&self) -> Vec<NSNode> {
        self.children.clone()
    }

    // Gets the subzone from the node
    pub fn get_subzone(&self) -> bool {
        self.subzone.clone()
    }
}

#[cfg(test)]
mod zone_node_test {
    use super::NSNode;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn constructor_test() {
        let nsnode = NSNode::new();

        assert_eq!(nsnode.name, String::from(""));
        assert_eq!(nsnode.value.len(), 0);
        assert_eq!(nsnode.children.len(), 0);
        assert_eq!(nsnode.subzone, false);
    }

    // Getters and Setters
    #[test]
    fn set_and_get_name_test() {
        let mut nsnode = NSNode::new();

        assert_eq!(nsnode.get_name(), String::from(""));
        nsnode.set_name(String::from("test.com"));
        assert_eq!(nsnode.get_name(), String::from("test.com"));
    }

    #[test]
    fn set_and_get_value_test() {
        let mut nsnode = NSNode::new();

        let mut value: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        value.push(resource_record);

        assert_eq!(nsnode.get_value().len(), 0);
        nsnode.set_value(value);
        assert_eq!(nsnode.get_value().len(), 1);
    }

    #[test]
    fn set_and_get_subzone_test() {
        let mut nsnode = NSNode::new();

        assert_eq!(nsnode.get_subzone(), false);
        nsnode.set_subzone(true);
        assert_eq!(nsnode.get_subzone(), true);
    }

    #[test]
    fn set_and_get_children_test() {
        let mut nsnode = NSNode::new();

        let mut children: Vec<NSNode> = Vec::new();
        let some_nsnode = NSNode::new();
        children.push(some_nsnode);

        assert_eq!(nsnode.get_children().len(), 0);
        nsnode.set_children(children);
        assert_eq!(nsnode.get_children().len(), 1);
    }

    /* #[test]
    fn set_duplicate_children_test() {
        //TODO:
    } */

    // Other methods

    //pub fn from_file(file_name: String) -> Self
    /*

    #[test]
    fn from_file_test(){
    }*/

    #[test]
    fn exist_child_test() {
        let mut nsnode = NSNode::new();
        let mut some_nsnode = NSNode::new();
        some_nsnode.set_name(String::from("test.com"));

        let mut children: Vec<NSNode> = Vec::new();
        children.push(some_nsnode);
        nsnode.set_children(children);
        assert_eq!(nsnode.exist_child(String::from("test2.com")), false);
        assert_eq!(nsnode.exist_child(String::from("tEsT.com")), true)
    }

    #[test]
    fn get_child_test() {
        let mut nsnode = NSNode::new();
        let mut some_nsnode = NSNode::new();
        let mut some_other_nsnode = NSNode::new();
        some_nsnode.set_name(String::from("test.com"));
        some_other_nsnode.set_name(String::from("other.test.com"));

        let mut children: Vec<NSNode> = Vec::new();
        children.push(some_nsnode);
        children.push(some_other_nsnode);
        nsnode.set_children(children);
        assert_eq!(
            nsnode
                .get_child(String::from("OTher.test.com"))
                .0
                .get_name(),
            String::from("other.test.com")
        );
        assert_eq!(nsnode.get_child(String::from("other.test.com")).1, 1);

        assert_eq!(
            nsnode.get_child(String::from("some.test.com")).0.get_name(),
            String::from("")
        );
        assert_eq!(nsnode.get_child(String::from("some.test.com")).1, -1);
    }

    #[test]
    fn add_node_test(){       


        let value: Vec<ResourceRecord> = Vec::new();
        
        
        let mut nsnode = NSNode::new();
        nsnode.set_name(String::from(""));
        let children: Vec<NSNode> = Vec::new();
        nsnode.set_children(children);

        assert_eq!(nsnode.add_node(String::from("mil"), value.clone()), Ok(()));
        assert_eq!(nsnode.add_node(String::from("edu"), value.clone()), Ok(()));

        assert_eq!(nsnode.add_node(String::from(""), value.clone()), Err("Error: Child cannot have null label, reserved for root only."));

        assert_eq!(nsnode.get_children().len(), 2)
    }

    /*#[test]
    fn add_node_test(){ using a wrong domain
    }*/

    #[test]
    fn check_rrs_only_ns_test() {
        let nsnode = NSNode::new();
        let mut rr: Vec<ResourceRecord> = Vec::new();

        let ns_rdata = Rdata::SomeNsRdata(NsRdata::new());
        let mut resource_record_1 = ResourceRecord::new(ns_rdata);
        resource_record_1.set_type_code(2);

        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let mut resource_record_2 = ResourceRecord::new(a_rdata);
        resource_record_2.set_type_code(1);

        rr.push(resource_record_1);

        assert_eq!(nsnode.check_rrs_only_ns(rr.clone()), true);
        rr.push(resource_record_2);
        assert_eq!(nsnode.check_rrs_only_ns(rr.clone()), false);
    }
    /*
    #[test]
    fn print_zone_test(){

    }

    */
    #[test]
    fn get_rrs_by_type_test() {
        let mut nsnode = NSNode::new();

        let mut value: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let mut resource_record = ResourceRecord::new(a_rdata);
        resource_record.set_type_code(1);
        value.push(resource_record);

        assert_eq!(nsnode.get_rrs_by_type(1).len(), 0);
        nsnode.set_value(value);
        assert_eq!(nsnode.get_rrs_by_type(1).len(), 1);
    }
}
