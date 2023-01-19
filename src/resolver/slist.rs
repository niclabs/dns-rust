use std::collections::HashMap;
use std::vec::Vec;

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
    ns_list: Vec<HashMap<String, String>>,
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
            ns_list: Vec::new(),
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
    pub fn insert(&mut self, name: String, ip_address: String, response_time: u32) {
        let mut new_element = HashMap::new();
        new_element.insert("name".to_string(), name);
        new_element.insert("ip_address".to_string(), ip_address);
        new_element.insert("response_time".to_string(), response_time.to_string());

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
    pub fn delete(&mut self, name: String) {
        let mut ns_list = self.get_ns_list();
        let mut index = 0;

        for ns in ns_list.iter() {
            if *ns.get("name").unwrap() == name {
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
    pub fn update_response_time(&mut self, name: String, response_time: u32) {
        let ns_list = self.get_ns_list();
        let mut index = 0;
        let mut new_ns_list = Vec::new();

        for mut ns in ns_list.into_iter() {
            if *(ns.get(&"name".to_string()).unwrap()) == name {
                ns.remove(&"response_time".to_string());
                ns.insert("response_time".to_string(), response_time.to_string());
            };
            new_ns_list.push(ns.clone());
            index = index + 1;
        }

        self.set_ns_list(new_ns_list);
    }

    // Gets the first ns from the list
    pub fn get_first(&self) -> HashMap<String, String> {
        let ns_list = self.get_ns_list();
        ns_list[0].clone()
    }

    pub fn get(&self, index: u16) -> HashMap<String, String> {
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
        let sort_by_response_time = |k: &HashMap<String, String>, j: &HashMap<String, String>| {
            let a = k
                .get(&"response_time".to_string())
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let b = j
                .get(&"response_time".to_string())
                .unwrap()
                .parse::<u32>()
                .unwrap();

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
    pub fn get_ns_list(&self) -> Vec<HashMap<String, String>> {
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
    pub fn set_ns_list(&mut self, ns_list: Vec<HashMap<String, String>>) {
        self.ns_list = ns_list;
    }
}

#[cfg(test)]
mod slist_test {
    use crate::resolver::slist::Slist;
    use std::collections::HashMap;
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
        let ns = HashMap::new();

        ns_list.push(ns);

        slist.set_ns_list(ns_list);

        assert_eq!(slist.get_ns_list().len(), 1);
    }

    #[test]
    fn insert_and_delete_test() {
        let mut slist = Slist::new();

        assert_eq!(slist.get_ns_list().len(), 0);

        slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5000);

        assert_eq!(slist.get_ns_list().len(), 1);

        slist.delete("test.com".to_string());

        assert_eq!(slist.get_ns_list().len(), 0);
    }

    #[test]
    fn get_first_test() {
        let mut slist = Slist::new();

        let mut first_element = HashMap::new();

        let name = "VENERA.ISI.EDU".to_string();
        let ip_address = "128.9.0.32".to_string();
        let response_time = 5000;

        first_element.insert("name".to_string(), name);
        first_element.insert("ip_address".to_string(), ip_address);
        first_element.insert("response_time".to_string(), response_time.to_string());

        slist.insert("VENERA.ISI.EDU".to_string(), "128.9.0.32".to_string(), 5000);
        slist.insert("XX.LCS.MIT.EDU".to_string(), "10.0.0.44".to_string(), 5001);

        assert_eq!(slist.get_first(), first_element);
    }

    #[test]
    fn get_test() {
        let mut slist = Slist::new();
        let mut some_element = HashMap::new();

        let name = "VENERA.ISI.EDU".to_string();
        let ip_address = "128.9.0.32".to_string();
        let response_time = 5000;

        some_element.insert("name".to_string(), name);
        some_element.insert("ip_address".to_string(), ip_address);
        some_element.insert("response_time".to_string(), response_time.to_string());

        slist.insert("VAXA.ISI.EDU".to_string(), "128.9.0.33".to_string(), 5000);
        slist.insert("XX.LCS.MIT.EDU".to_string(), "10.0.0.44".to_string(), 5001);
        slist.insert("VENERA.ISI.EDU".to_string(), "128.9.0.32".to_string(), 5000);

        assert_eq!(slist.get(2 as u16), some_element);
    }

    #[test]
    fn update_response_time_test() {
        let mut slist = Slist::new();
        slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5000);

        assert_eq!(
            *slist.get_first().get(&"response_time".to_string()).unwrap(),
            5000.to_string()
        );

        slist.update_response_time("test.com".to_string(), 2000);

        assert_eq!(
            *slist.get_first().get(&"response_time".to_string()).unwrap(),
            2000.to_string()
        );
    }

    #[test]
    fn sort_test() {
        let mut slist = Slist::new();
        slist.insert("test.com".to_string(), "127.0.0.1".to_string(), 5000);
        slist.insert("test2.com".to_string(), "127.0.0.1".to_string(), 2000);

        assert_eq!(
            *slist.get_first().get(&"response_time".to_string()).unwrap(),
            5000.to_string()
        );
        assert_eq!(slist.get_ns_list().len(), 2);

        slist.sort();

        assert_eq!(
            *slist.get_first().get(&"response_time".to_string()).unwrap(),
            2000.to_string()
        );
        assert_eq!(slist.get_ns_list().len(), 2);
    }
}
