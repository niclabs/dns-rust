pub mod slist_element;

use std::net::IpAddr;
use std::vec::Vec;
use crate::domain_name::DomainName;
use crate::resolver::slist::slist_element::SlistElement;

#[derive(Clone)]
// Struct that represents the Slist estructure from RFC 1034 page 33.
// "A structure which describes the name servers and the
// zone which the resolver is currently trying to query.
// This structure keeps track of the resolver's current
// best guess about which name servers hold the desired
// information; it is updated when arriving information
// changes the guess.  This structure includes the
// equivalent of a zone name, the known name servers for
// the zone, the known addresses for the name servers, and
// history information which can be used to suggest which
// server is likely to be the best one to try next.  The
// zone name equivalent is a match count of the number of
// labels from the root down which SNAME has in common with
// the zone being queried; this is used as a measure of how
// "close" the resolver is to SNAME."
pub struct Slist {
    zone_name_equivalent: i32,
    ns_list: Vec<SlistElement>,
}

impl Slist {
    // Creates a new Slist.
    //
    // # Examples
    // '''
    // let slist = Slist::new();
    //
    // assert_eq!(slist.zone_name_equivalent, -1);
    // assert_eq!(slist.ns_list.len(), 0);
    // '''
    //
    pub fn new() -> Self {
        let slist = Slist {
            zone_name_equivalent: -1,
            ns_list: Vec::<SlistElement>::new(),
        };

        slist
    }

    // Inserts a new ns in the slist.
    //
    // # Examples
    // '''
    // let mut slist = Slist::new();
    //
    // assert_eq!(slist.get_ns_list().len(), 0);
    //
    // slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5 as f32);
    //
    // assert_eq!(slist.get_ns_list().len(), 1);
    // '''
    //
    pub fn insert(&mut self, name: DomainName, ip_address: IpAddr, response_time: u32) {
        let mut new_element = SlistElement::new(name, ip_address, response_time);
        /* new_element.insert("name".to_string(), name);
        new_element.insert("ip_address".to_string(), ip_address);
        new_element.insert("response_time".to_string(), response_time.to_string()); */

        let mut ns_list = self.get_ns_list();
        ns_list.push(new_element);

        self.set_ns_list(ns_list);
    }

    // Deletes a ns from the slist.
    //
    // # Examples
    // '''
    // let mut slist = Slist::new();
    //
    // assert_eq!(slist.get_ns_list().len(), 0);
    //
    // slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5 as f32);
    //
    // assert_eq!(slist.get_ns_list().len(), 1);
    //
    // slist.delete("test.com".to_string());
    //
    // assert_eq!(slist.get_ns_list().len(), 0);
    // '''
    //
    pub fn delete(&mut self, name: DomainName) {
        let mut ns_list = self.get_ns_list();
        let mut index = 0;

        for ns in ns_list.iter() {
            if *ns.get_domain_name().get_name() == name.get_name() {
                ns_list.remove(index);
                break;
            };
            index = index + 1;
        }

        self.set_ns_list(ns_list);
    }

    // Updates the response time from a ns.
    //
    // # Examples
    // '''
    // let mut slist = Slist::new();
    // slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5 as f32);
    //
    // assert_eq!(
    //     *slist.get_first().get(&"response_time".to_string()).unwrap(),
    //     5.to_string()
    // );
    //
    // slist.update_response_time("test.com".to_string(), 2 as f32);
    //
    // assert_eq!(
    //     *slist.get_first().get(&"response_time".to_string()).unwrap(),
    //     2.to_string()
    // );
    // '''
    //
    pub fn update_response_time(&mut self, name: DomainName, response_time: u32) {
        let ns_list = self.get_ns_list();
        let mut index = 0;
        let mut new_ns_list = Vec::new();

        for mut ns in ns_list.into_iter() {
            if *(ns.get_domain_name().get_name()) == name.get_name() {
                ns.set_response_time(response_time);
            };
            new_ns_list.push(ns.clone());
            index = index + 1;
        }

        self.set_ns_list(new_ns_list);
    }

    // Gets the first ns from the list
    pub fn get_first(&self) -> SlistElement {
        let ns_list = self.get_ns_list();
        ns_list[0].clone()
    }

    pub fn get(&self, index: u16) -> SlistElement {
        let ns_list = self.get_ns_list();

        ns_list[index as usize].clone()
    }

    // Sorts the ns list by response time
    //
    // # Examples
    // '''
    // let mut slist = Slist::new();
    // slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5 as f32);
    // slist.insert("test2.com".to_string(), "127.0.0.1".to_string(), 2 as f32);
    //
    // assert_eq!(
    //     *slist.get_first().get(&"response_time".to_string()).unwrap(),
    //     5.to_string()
    // );
    // assert_eq!(slist.get_ns_list().len(), 2);
    //
    // slist.sort();
    //
    // assert_eq!(
    //     *slist.get_first().get(&"response_time".to_string()).unwrap(),
    //     2.to_string()
    // );
    //
    // assert_eq!(slist.get_ns_list().len(), 2);
    // '''
    //
    pub fn sort(&mut self) {
        let sort_by_response_time = |k: &SlistElement, j: &SlistElement| {
            let a = k
                .get_response_time();
            let b = j
                .get_response_time();

            a.partial_cmp(&b).unwrap()
        };

        let mut ns_list = self.get_ns_list();

        ns_list.sort_by(sort_by_response_time);

        self.set_ns_list(ns_list);
    }

    pub fn len(&self) -> usize {
        let ns_list = self.get_ns_list();

        ns_list.len()
    }
}

// Getters
impl Slist {
    // Gets the zone name equivalent from the slist
    pub fn get_zone_name_equivalent(&self) -> i32 {
        self.zone_name_equivalent
    }

    // Gets the ns list from the slist
    pub fn get_ns_list(&self) -> Vec<SlistElement> {
        self.ns_list.clone()
    }
}

// Setters
impl Slist {
    // Sets the zone name equivalent attribute with a new value
    pub fn set_zone_name_equivalent(&mut self, zone_name_equivalent: i32) {
        self.zone_name_equivalent = zone_name_equivalent;
    }

    // Sets the ns list attribute with a new value
    pub fn set_ns_list(&mut self, ns_list: Vec<SlistElement>) {
        self.ns_list = ns_list;
    }
}

#[cfg(test)]
mod slist_test {
    use crate::domain_name::DomainName;
    use crate::resolver::slist::Slist;
    use crate::resolver::slist::slist_element::SlistElement;
    use std::net::{IpAddr,Ipv4Addr};
    use std::vec::Vec;

    #[test]
    fn constructor_test() {
        let slist = Slist::new();

        assert_eq!(slist.zone_name_equivalent, -1);
        assert_eq!(slist.ns_list.len(), 0);
    }

    #[test]
    fn set_and_get_zone_name_equivalent() {
        let mut slist = Slist::new();

        assert_eq!(slist.get_zone_name_equivalent(), -1);

        slist.set_zone_name_equivalent(2);

        assert_eq!(slist.get_zone_name_equivalent(), 2);
    }

    #[test]
    fn set_and_get_ns_list() {
        let mut slist = Slist::new();

        assert_eq!(slist.get_ns_list().len(), 0);

        let mut ns_list = Vec::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ns = SlistElement::new(domain_name, ip_address, 4);

        ns_list.push(ns);

        slist.set_ns_list(ns_list);

        assert_eq!(slist.get_ns_list().len(), 1);
    }

    #[test]
    fn insert_and_delete_test() {
        let mut slist = Slist::new();

        assert_eq!(slist.get_ns_list().len(), 0);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        slist.insert(domain_name.clone(), ip_address.clone(), 5000);

        assert_eq!(slist.get_ns_list().len(), 1);

        slist.delete(domain_name.clone());

        assert_eq!(slist.get_ns_list().len(), 0);
    }

    #[test]
    fn get_first_test() {
        let mut slist = Slist::new();

        

        let mut name = DomainName::new();
        name.set_name(String::from("VENERA.ISI.EDU"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(128, 9, 0, 32));
        let response_time = 5000;

        let first_element = SlistElement::new(name.clone(), ip_address.clone(), response_time.clone());

        slist.insert(name.clone(), ip_address.clone(), 5000);

        let mut name_2 = DomainName::new();
        name_2.set_name(String::from("XX.LCS.MIT.EDU"));
        let ip_address_2 = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 44));
        slist.insert(name_2, ip_address_2, 5001);

        let first = slist.get_first();
        assert_eq!(first.get_domain_name(), first_element.get_domain_name());
        assert_eq!(first.get_ip_address(), first_element.get_ip_address());
        assert_eq!(first.get_response_time(), first_element.get_response_time());
    }

    #[test]
    fn get_test() {
        let mut slist = Slist::new();

        let mut name = DomainName::new();
        name.set_name(String::from("VENERA.ISI.EDU"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(128, 9, 0, 32));
        let response_time = 5000;

        let some_element = SlistElement::new(name.clone(), ip_address.clone(), response_time.clone());

        let mut name_2 = DomainName::new();
        name_2.set_name(String::from("VAXA.ISI.EDU"));
        let ip_address_2 = IpAddr::V4(Ipv4Addr::new(128, 9, 0, 33));
        slist.insert(name_2.clone(), ip_address_2.clone(), 5000);

        let mut name_3 = DomainName::new();
        name_3.set_name(String::from("XX.LCS.MIT.EDU"));
        let ip_address_3 = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 44));

        slist.insert(name_3.clone(), ip_address_3.clone(), 5001);

        slist.insert(name.clone(), ip_address.clone(), 5000);

        let third_element = slist.get(2 as u16);
        assert_eq!(third_element.get_domain_name(), some_element.get_domain_name());
        assert_eq!(third_element.get_ip_address(), some_element.get_ip_address());
        assert_eq!(third_element.get_response_time(), some_element.get_response_time());
    }

    #[test]
    fn update_response_time_test() {
        let mut slist = Slist::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        slist.insert(domain_name.clone(), ip_address.clone(), 5000);

        assert_eq!(
            slist.get_first().get_response_time(),
            5000
        );

        slist.update_response_time(domain_name.clone(), 2000);

        assert_eq!(
            slist.get_first().get_response_time(),
            2000
        );
    }

    #[test]
    fn sort_test() {
        let mut slist = Slist::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        slist.insert(domain_name.clone(), ip_address.clone(), 5000);
        
        let mut domain_name_2 = DomainName::new();
        domain_name_2.set_name(String::from("test2.com"));
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        slist.insert(domain_name.clone(), ip_address.clone(), 2000);

        assert_eq!(
            slist.get_first().get_response_time(),
            5000
        );
        assert_eq!(slist.get_ns_list().len(), 2);

        slist.sort();
        assert_eq!(
            slist.get_first().get_response_time(),
            2000
        );
        assert_eq!(slist.get_ns_list().len(), 2);
    }
}
