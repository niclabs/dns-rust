use crate::message::resource_record::ResourceRecord;

//utils
use crate::utils::check_label_name;

#[derive(Clone, PartialEq, Debug)]
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

    ///Returns bolean depending if the node has a children with the same name given
    pub fn exist_child(&self, name: String) -> bool {
        // case insensitive
        let childrens= self.get_children();
        let lower_case_name = name.to_ascii_lowercase();

        for child in childrens {
            println!("Child name: {}", child.get_name());
            let lower_case_child = child.get_name().to_ascii_lowercase();
            if lower_case_child == lower_case_name{
                return true;
            }
        }

        return false;
    }

    ///looks if exist a children with the name given.
    /// If exist returns a tuple with the node and the index where is found in the vec 
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

    ///Creates all the nodes - if they are not creted yet - within a host name and add as values
    /// the vec of RR of the final node of the label
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

                        child.add_node(new_name, rrs).unwrap();
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

                        new_ns_zone.add_node(new_name, rrs).unwrap();
                    }

                    children.push(new_ns_zone);
                    self.set_children(children);
                }
            }
            return Ok(()); 
        }
    }

    ///Checks if all the RR given in the vec are type NS
    fn check_rrs_only_ns(&self, rrs: Vec<ResourceRecord>) -> bool {
        for rr in rrs {
            let rr_type = rr.get_type_code();

            if rr_type != 2 {
                return false;
            }
        }

        return true;
    }

    ///Given a type of RR as int returns a vec with all the RR that match tis type (exclude type NS)
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


    ///Returns all the RR within a node  
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


    //TODO: Revisar Práctica 1
    #[test]
    fn set_duplicate_children_test() {
        let mut nsnode = NSNode::new();

        let mut children = Vec::<NSNode>::new();
        let mut nsnode_1 = NSNode::new();
        nsnode_1.set_name(String::from("test1"));
        children.push(nsnode_1);
        let mut nsnode_3 = NSNode::new();
        nsnode_3.set_name(String::from("test2"));
        children.push(nsnode_3);
        let mut nsnode_3 = NSNode::new();
        nsnode_3.set_name(String::from("TEST1"));
        children.push(nsnode_3);

        assert_eq!(nsnode.get_children().len(), 0);
        nsnode.set_children(children);
        assert_eq!(nsnode.get_children().len(), 2);
        let child1_name = nsnode.get_children()[1].get_name();
        assert_eq!(
            child1_name,
            String::from("TEST1") 
        );
        assert_eq!(nsnode.get_children()[0].get_name(), String::from("test2"));
        assert_eq!(nsnode.exist_child(String::from("test1")), true); //child still can be searched in lowercase      
    }

    #[test]
    fn exist_child_test() {
        let mut nsnode = NSNode::new();

        let mut some_nsnode = NSNode::new();
        some_nsnode.set_name(String::from("test.com"));

        let mut children: Vec<NSNode> = Vec::new();
        children.push(some_nsnode);
        nsnode.set_children(children);

        assert_eq!(nsnode.exist_child(String::from("test2.com")), false);
        assert_eq!(nsnode.exist_child(String::from("tEsT.com")), true);
        assert_eq!(nsnode.exist_child(String::from("test.com")), true);
    }

    #[test]
    fn get_child_test() {
        //creates nodes
        let mut node = NSNode::new();
        let mut ns_node1= NSNode::new();
        let mut ns_node2 = NSNode::new();
        ns_node1.set_name(String::from("test.com"));
        ns_node2.set_name(String::from("other.test.com"));

        //create vec children  
        let mut children: Vec<NSNode> = Vec::new();
        children.push(ns_node1);
        children.push(ns_node2);
        node.set_children(children);

        assert_eq!(
            node
                .get_child(String::from("OTher.test.com"))
                .0
                .get_name(),
            String::from("other.test.com")
        );
        assert_eq!(node.get_child(String::from("other.test.com")).1, 1);

        assert_eq!(
            node.get_child(String::from("some.test.com")).0.get_name(),
            String::from("")
        );
        assert_eq!(node.get_child(String::from("some.test.com")).1, -1);
    }

    #[test]
    fn add_node_test(){       
        //creeate node
        let mut nsnode = NSNode::new();
        nsnode.set_name(String::from(""));

        let value: Vec<ResourceRecord> = Vec::new();
        let children: Vec<NSNode> = Vec::new();
        nsnode.set_children(children);


        assert_eq!(nsnode.add_node(String::from("mil"), value.clone()), Ok(()));
        assert_eq!(nsnode.add_node(String::from("edu"), value.clone()), Ok(()));

        assert_eq!(nsnode.add_node(String::from(""), value.clone()), Err("Error: Child cannot have null label, reserved for root only."));

        assert_eq!(nsnode.get_children().len(), 2);
        //ToDo: Revisar Práctica 1
        assert_eq!(nsnode.get_children()[0].get_name(), String::from("mil"));
        assert_eq!(nsnode.get_children()[1].get_name(), String::from("edu"));
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn add_node_test_wrong_domain(){
        let mut nsnode = NSNode::new();
        let mut value = Vec::<ResourceRecord>::new();
        let children = Vec::<NSNode>::new();
        let ardata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(ardata);
        value.push(rr);
        nsnode.set_children(children);

        let too_large = String::from("kuvfirdigdkfhksjfoi.shfuiedehiauhdeaanda.ehbndpuoeyuwuwmejulfg.hvdjxkkxucicxvadofeiaee.txyvqlclymdgccungwiwrwonl.ehjedknwbupefsysvxmkcski.pmkxifjrqxsth.pljtklaaubdzskc.almwbcllhifmknrxamlhdfywmvtwiblee.zrfttfcloqqc.xsgqtpanxkfih.uwfyxkfeemnl.vdqxkdx.ndtkxdauhuae.jlpllhvhkem.gudururqu.ftqoumt.fywup.edu");

        let res1 = nsnode.add_node(too_large, value.clone());
        let res2 = nsnode.get_children().len();
        assert_eq!(res1, Ok(()));
        assert_eq!(res2, 0); //There must be no children

        let large_but_not_too_much = String::from("this-is-a-extremely-large-label-that-have-exactly--64-characters");

        let res3 = nsnode.add_node(large_but_not_too_much, value.clone());
        let res4 = nsnode.get_children().len();
        assert_eq!(res3, Ok(()));
        assert_eq!(res4, 0); //There must be no children
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn add_node_test_recursive(){
        let mut nsnode = NSNode::new();
        let mut value = Vec::<ResourceRecord>::new();
        let children = Vec::<NSNode>::new();
        let ardata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(ardata);
        value.push(rr);
        nsnode.set_children(children);

        let name = String::from("mail.example.edu");

        let res1 = nsnode.add_node(name, value.clone());
        let res2 = nsnode.get_children().len();
        assert_eq!(res1, Ok(()));
        assert_eq!(res2, 1);

        let child = nsnode.get_children()[0].clone(); //child with the name of edu
        assert_eq!(child.get_name(), String::from("edu"));
        
        let child2 = child.get_children()[0].clone(); //child with the name of example
        assert_eq!(child2.get_name(), String::from("example"));

        let child3 = child2.get_children()[0].clone(); //child with the name of mail
        assert_eq!(child3.get_name(), String::from("mail"));
    }

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

    #[test]
    fn get_rrs_by_type_test() {

        let mut nsnode = NSNode::new();
        let mut value: Vec<ResourceRecord> = Vec::new();

        //ToDo: Revisar Práctica 1
        let ns_rdata1 = Rdata::SomeNsRdata(NsRdata::new());
        let mut rr1 = ResourceRecord::new(ns_rdata1);
        rr1.set_type_code(2);

        let ns_rdata2 = Rdata::SomeNsRdata(NsRdata::new());
        let mut rr2 = ResourceRecord::new(ns_rdata2);
        rr2.set_type_code(2);

        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let mut rr3 = ResourceRecord::new(a_rdata);
        rr3.set_type_code(1);

        value.push(rr1);
        value.push(rr2);
        value.push(rr3);
        
        let res1 = nsnode.get_rrs_by_type(1);
        let res2 = nsnode.get_rrs_by_type(2);
        assert_eq!(res1.len(), 0);
        assert_eq!(res2.len(), 0);

        nsnode.set_value(value);

        let res3 = nsnode.get_rrs_by_type(1);
        let res4 = nsnode.get_rrs_by_type(2);
        let res5 = nsnode.get_rrs_by_type(255);
        assert_eq!(res3.len(), 1);
        assert_eq!(res4.len(), 2);
        assert_eq!(res5.len(), 3);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_all_rrs_test() {
        let mut nsnode = NSNode::new();
        let mut value: Vec<ResourceRecord> = Vec::new();

        
        let ns_rdata1 = Rdata::SomeNsRdata(NsRdata::new());
        let mut rr1 = ResourceRecord::new(ns_rdata1);
        rr1.set_type_code(2);

        let ns_rdata2 = Rdata::SomeNsRdata(NsRdata::new());
        let mut rr2 = ResourceRecord::new(ns_rdata2);
        rr2.set_type_code(2);

        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let mut rr3 = ResourceRecord::new(a_rdata);
        rr3.set_type_code(1);

        value.push(rr1);
        value.push(rr2);
        value.push(rr3);

        assert_eq!(nsnode.get_all_rrs().len(), 0);

        nsnode.set_value(value);

        assert_eq!(nsnode.get_all_rrs().len(), 3);

        let res1 = nsnode.get_all_rrs()[0].get_type_code();
        assert_eq!(res1, 2);

        let res2 = nsnode.get_all_rrs()[1].get_type_code();
        assert_eq!(res2, 2);
        
        let res3 = nsnode.get_all_rrs()[2].get_type_code();
        assert_eq!(res3, 1);

    }
    
}
