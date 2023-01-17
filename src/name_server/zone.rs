

use crate::message::DnsMessage;
use crate::message::resource_record::ResourceRecord;

use super::master_file::MasterFile;
use super::zone_node::NSNode;


#[derive(Clone)]
/// Struct that represents a zone.
pub struct NSZone {
    // Zone name
    name: String,
    // Ip to ask the SOA RR data for refreshing
    ip_address_for_refresh_zone: String,
    // Top node of the zone
    zone_nodes: NSNode,
    // Zone class
    class: u16,
    // Zone is active
    active: bool,
    // Glue records of the zone
    glue_rrs: Vec<ResourceRecord>,
}

impl NSZone {
    pub fn new() -> Self {
        let ns_zone = NSZone {
            name: "".to_string(),
            ip_address_for_refresh_zone: "".to_string(),
            zone_nodes: NSNode::new(),
            class: 1,
            active: true,
            glue_rrs: Vec::<ResourceRecord>::new(),
        };

        return ns_zone
    }

    ///Creates a zone base on the masterfile given 
    pub fn from_file(file_name: String, origin:String, ip_address_for_refresh_zone: String, validity_check: bool) -> Self {
        let master_file_parsed;
        print!("checkpint1");
        master_file_parsed = MasterFile::from_file(file_name, origin,validity_check);
        print!("checkpint1");
        let origin = master_file_parsed.get_origin();
        let mut rrs = master_file_parsed.get_rrs();
        
        // Sets Zone info
        let mut ns_zone = NSZone::new();
        let top_node_name = master_file_parsed.get_top_host();
        ns_zone.set_name(top_node_name.clone());
        ns_zone.set_ip_address_for_refresh_zone(ip_address_for_refresh_zone);
        ns_zone.set_class_str(master_file_parsed.get_class_default());
        
        // Sets top node info
        let mut top_node = NSNode::new();
        top_node.set_name(top_node_name.clone());
        top_node.set_value(rrs.get(top_node_name.clone().as_str()).unwrap().clone());

        rrs.remove(&origin);

        for (key, value) in rrs.iter() {
            println!("{} - {}", key.clone(), value.len());
            top_node.add_node(key.clone(), value.clone()).unwrap();
        }

        ns_zone.set_zone_nodes(top_node);

        return ns_zone
    }

    ///
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
                    // FIXME:
                    //new_zone.add_node(actual_node_name, rrs_for_node);

                    rrs_for_node = Vec::<ResourceRecord>::new();
                    actual_node_name = rr_name;
                }

                // Add the rr to the vec
                rrs_for_node.push(rr.clone());
                next_rr = rr_iter.next();
            }
        }

        return new_zone
    }
}

// SETTERS
impl NSZone {
    /// Sets the name of the zone with a new value
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_ip_address_for_refresh_zone(&mut self, ip_address_for_refresh_zone: String) {
        self.ip_address_for_refresh_zone = ip_address_for_refresh_zone;
    }

    /// Sets the nodes of the zone
    pub fn set_zone_nodes(&mut self, zone_nodes: NSNode) {
        self.zone_nodes = zone_nodes;
    }

    /// Sets the class for the zone
    pub fn set_class(&mut self, class: u16) {
        self.class = class;
    }

    /// Sets the class from a string
    pub fn set_class_str(&mut self, class: String) {
        let class = match class.as_str() {
            "IN" => 1,
            "CH" => 3,
            "HS" => 4,
            _ => unreachable!("invalid string"),
        };

        self.set_class(class);
    }

    /// Sets if the zone is active or not
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Sets the glue_rrs with a new value
    pub fn set_glue_rrs(&mut self, glue_rrs: Vec<ResourceRecord>) {
        self.glue_rrs = glue_rrs;
    }
}

// GETTERS
impl NSZone {
    /// Gets the name of the zone
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_ip_address_for_refresh_zone(&self) -> String {
        self.ip_address_for_refresh_zone.clone()
    }

    /// Gets the nodes of the zone
    pub fn get_zone_nodes(&self) -> NSNode {
        self.zone_nodes.clone()
    }

    /// Gets the zone class
    pub fn get_class(&self) -> u16 {
        self.class
    }

    /// Gets if the zone is active
    pub fn get_active(&self) -> bool {
        self.active
    }

    /// Gets the glue rrs from the node
    pub fn get_glue_rrs(&self) -> Vec<ResourceRecord> {
        self.glue_rrs.clone()
    }
}


#[cfg(test)]
mod zone_test {

    use crate::message::rdata::a_rdata::ARdata;
    use crate::name_server::zone::NSZone;    
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::name_server::zone_node::NSNode;

    #[test]
    fn constructor_test() { //TODO revisar práctica 1
        let mut nszone = NSZone::new();
        let mut nsnode =NSNode::new();
        nsnode.set_name("example.com".to_string());
        nszone.set_zone_nodes(nsnode.clone());
        assert_eq!(nszone.name, String::from(""));
        assert_eq!(nszone.zone_nodes.get_name(), nsnode.clone().get_name());
        assert_eq!(nszone.class, 1);
        assert_eq!(nszone.active, true);
        assert_eq!(nszone.glue_rrs.len(), 0);
    }

    #[test]
    fn get_and_set_name_test(){//TODO revisar práctica 1
        let mut nszone = NSZone::new();

        assert_eq!(nszone.get_name(), String::from(""));
        nszone.set_name(String::from("test.com"));
        assert_eq!(nszone.get_name(), String::from("test.com"));
    }

    #[test]
    fn get_and_set_ip_address_for_refresh_zone_test(){//TODO revisar práctica 1
        let mut nszone = NSZone::new();

        nszone.set_class(1);
        assert_eq!(nszone.get_class(), 1);
        nszone.set_class(3);
        assert_eq!(nszone.get_class(), 3);


    }

    #[test]
    fn set_and_get_zone_nodes_test(){//TODO revisar práctica 1
        let mut nszone = NSZone::new();
        let mut nsnode =NSNode::new();
        nsnode.set_name("example.com".to_string());
        nszone.set_zone_nodes(nsnode.clone());
        
        assert_eq!(nszone.zone_nodes.get_name(), nsnode.clone().get_name());

    }

    #[test]
    fn set_and_get_class_test(){//TODO revisar práctica 1
        let mut nszone = NSZone::new();
        let mut nsnode =NSNode::new();
        nsnode.set_name("example.com".to_string());
        nszone.set_zone_nodes(nsnode.clone());
        assert_eq!(nszone.name, String::from(""));
        assert_eq!(nszone.zone_nodes.get_name(), nsnode.clone().get_name());

    }

    #[test]
    fn set_and_get_active_test(){//TODO revisar práctica 1
        let mut nszone = NSZone::new();

        nszone.set_active(false);
        assert_eq!(nszone.get_active(), false);
        nszone.set_active(true);
        assert_eq!(nszone.get_active(), true);
        

    }


  
    #[test]
    fn from_file_test(){//TODO revisar práctica 1
        let nszone_mut = NSZone::from_file("test.txt".to_string(),"example".to_string(),"192.80.24.11".to_string(),true );
        let name= nszone_mut.get_name();
        let class= nszone_mut.get_class();
        let ip= nszone_mut.get_ip_address_for_refresh_zone();
        assert_eq!("uchile.cl.".to_string(), name);
        assert_eq!(1,class);
        assert_eq!("192.80.24.11".to_string(),ip);

    }
 

    
    #[test]
    fn from_axfr_msg_test(){//TODO revisar práctica 1

    }
    

    #[test]
    fn set_and_get_glue_rr_test() {//TODO revisar práctica 1
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
    #[should_panic]
    fn set_class_str_fail_test(){//TODO revisar práctica 1
        let mut nszone = NSZone::new();
        nszone.set_class_str("asjkh".to_string());   
    }

    
}