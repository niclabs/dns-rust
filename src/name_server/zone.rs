

use super::zone_node::NSNode;

/// Struct that represents a zone.
pub struct NSZone {
    // Zone name
    name: String,
    // Ip to ask the SOA RR data for refreshing
    ip_address_for_refresh_zone: String,
    // Nodes of the zone
    nodes: NSNode,
    // Zone class
    class: u16,
    // Zone is active
    active: bool,
}

impl NSZone {
    pub fn new() -> Self {
        let ns_zone = NSZone {
            name: "".to_string(),
            ip_address_for_refresh_zone: "".to_string(),
            nodes: NSNode::new(),
            class: 1,
            active: true,
        };

        return ns_zone
    }

    pub fn from_file(file_name: String, ip_address_for_refresh_zone: String, validity_check: bool) -> Self {
        let master_file_parsed;
        master_file_parsed = MasterFile::from_file(file_name, validity_check);
        let origin = master_file_parsed.get_origin();
        let mut rrs = master_file_parsed.get_rrs();

        let origin_rrs = rrs.remove(&origin).unwrap();

        let mut ns_zone = NSNode::new();
        ns_zone.set_name(origin);
        ns_zone.set_ip_address_for_refresh_zone(ip_address_for_refresh_zone);
        ns_zone.set_value(origin_rrs);
        ns_zone.set_class_str(master_file_parsed.get_class_default());

        for (key, value) in rrs.iter() {
            println!("{} - {}", key.clone(), value.len());
            ns_zone.add_node(key.clone(), value.clone());
        }

        return ns_zone
    }

    pub fn from_axfr_msg(msg: DnsMessage) -> Self {
        let answers = msg.get_answer();
        let mut new_zone = NSNode::new();

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

        return new_zone
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

    /// Sets the nodes for the zone
    pub fn set_nodes(&mut self, nodes: NSNode) {
        self.nodes = nodes;
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
            _ => unreachable!(),
        };

        self.set_class(class);
    }

    /// Sets if the zone is active or not
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
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
    pub fn get_nodes(&self) -> NSNode {
        self.nodes
    }

    /// Gets the zone class
    pub fn get_class(&self) -> u16 {
        self.class
    }

    /// Gets if the zone is active
    pub fn get_active(&self) -> bool {
        self.active
    }
}