use chrono::{Utc, DateTime};
use crate::{domain_name::DomainName, message::rdata::Rdata};
use crate::dns_cache::cache_by_record_type::rr_stored_data::RRStoredData;
use std::{collections::HashMap, net::IpAddr};

/// This struct saves the data associated with a host in the cache.
/// 
/// Given a single `DomainName`, it groups all data associated with it 
/// a `Vec<RRStoredData>` inside a `HashMap<DomainName, Vec<RRStoredData>>`. 
/// This means, all the cache data associated with a single host
/// of an specific `Rtype`.
#[derive(Clone, Debug)]
pub struct CacheByDomainName {
    /// Contains the Resource Records associated to each host domain name.
    /// 
    /// The key is the `DomainName` of the host, and the value is a 
    /// `Vec<RRStoredData>`, which contains all the Resource Records 
    /// data associated to the host for a single `Rtype`.
    domain_names_data: HashMap<DomainName, Vec<RRStoredData>>,
}

///functions for the host data
impl CacheByDomainName {

    ///function to create a new host data
    /// # Example
    /// ```
    /// let host_data = CacheByDomainName::new();
    /// ```
    pub fn new() -> CacheByDomainName {
        CacheByDomainName {
            domain_names_data: HashMap::new(),
        }
    }

    ///function to add a rr_cache to the host data
    /// # Example
    /// ```
    /// let mut host_data = CacheByDomainName::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// ```
    /// # Arguments
    /// * `host_name` - A Domain Name that represents the name of the host
    /// * `rr_cache` - A RRStoredData that represents the rr_cache of the host
    pub fn add_to_host_data(&mut self, host_name: DomainName, rr_cache: RRStoredData) {
        let mut domain_names_data = self.get_domain_names_data();
        if let Some(y) = domain_names_data.get_mut(&host_name){
            let mut rr_cache_vec = y.clone();
            rr_cache_vec.push(rr_cache);
            domain_names_data.insert(host_name, rr_cache_vec);
        }
        else{
            let mut rr_cache_vec = Vec::new();
            rr_cache_vec.push(rr_cache);
            domain_names_data.insert(host_name, rr_cache_vec);
        }
        self.set_domain_names_data(domain_names_data);
    }

    ///function to remove an element from the host data
    /// # Example
    /// ```
    /// let mut host_data = CacheByDomainName::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// host_data.remove_from_host_data(domain_name);
    /// ```
    pub fn remove_from_host_data(&mut self, host_name: DomainName) -> u32{
        let mut domain_names_data = self.get_domain_names_data();
        if let Some(_x) = domain_names_data.remove(&host_name){
            self.set_domain_names_data(domain_names_data);
            return _x.len() as u32;    
        }
        return 0;
    }

    /// Returns an element from the host data. 
    /// 
    /// This element corresponds to a vector of `RRStoredData` that contains 
    /// the `RRStoredData` of the host.
    /// 
    /// # Example
    /// ```
    /// let mut host_data = CacheByDomainName::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// let host_data_2_vec = host_data.get_from_host_data(domain_name);
    /// ```
    pub fn get_from_host_data(&mut self, host_name: DomainName) -> Option<Vec<RRStoredData>>{
        let mut domain_names_data = self.get_domain_names_data();
        if let Some(x) = domain_names_data.get(&host_name){
            let new_x = x.clone();
            let mut rr_cache_vec = Vec::<RRStoredData>::new();
            for mut rr_cache in new_x{
                rr_cache.set_last_use(Utc::now());
                rr_cache_vec.push(rr_cache.clone());
            }
            domain_names_data.insert(host_name, rr_cache_vec.clone());
            self.set_domain_names_data(domain_names_data);

            return Some(rr_cache_vec);
        }
        else{
            return None;
        }
    }

    ///function to get the oldest used 
    /// # Example
    /// ```
    /// let mut host_data = CacheByDomainName::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// rr_cache.set_last_use(Utc::now());
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// let host_data_2 = get_oldest_used.get_from_host_data(domain_name);
    /// ```
    pub fn get_oldest_used(&mut self)-> (DomainName,DateTime<Utc>){
        let host = self.get_domain_names_data();
        let mut used_in = Utc::now();

        let mut oldest_used_domain_name = DomainName::new();

        for (host_key, host_value) in host {
            let rr_last_use = host_value[0].get_last_use();

            if rr_last_use <= used_in {
                used_in = rr_last_use;
                oldest_used_domain_name = host_key.clone();
                
            }
        }

        return (oldest_used_domain_name,used_in);

    }
    
    ///function to insert a domain name and a new value to be associated
    /// and return the value that was associated before, or none if 
    /// the domain name didn't exist before
    /// # Example
    /// ```
    /// let mut host_data = CacheByDomainName::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let mut rr_cache = RRStoredData::new(resource_record);
    /// rr_cache.set_last_use(Utc::now());
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// let mut domain_name_new = DomainName::new();
    /// domain_name_new.set_name(String::from("inserted"));
    /// let a_rdata_2 = Rdata::A(ARdata::new());
    /// let resource_record_2 = ResourceRecord::new(a_rdata);
    /// let mut rr_cache_2 = RRStoredData::new(resource_record);
    /// host_data.insert(domain_name_new, rr_cache_2);
    /// ```
    pub fn insert(&mut self,domain_name:DomainName, rr_cache_vec : Vec<RRStoredData>) -> Option<Vec<RRStoredData>>{
        return self.domain_names_data.insert(domain_name, rr_cache_vec)
    }

    ///function to update the response time
    /// # Example
    /// ```
    /// let mut host_data = CacheByDomainName::new();
    /// let ip_address = IpAddr::from([127, 0, 0, 1]);
    /// let a_rdata = ARdata::new();
    /// a_rdata.set_address(ip_address);
    /// let rdata = Rdata::A(a_rdata);
    /// let resource_record = ResourceRecord::new(rdata);
    /// let mut rr_cache = RRStoredData::new(resource_record);
    /// rr_cache.set_response_time(1000);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// host_data.update_response_time(ip_address, 2000, domain_name);
    /// ```
    pub fn update_response_time(&mut self, ip_address: IpAddr, response_time: u32, domain_name: DomainName){
        let mut domain_names_data = self.get_domain_names_data();
        if let Some(x) = domain_names_data.get(&domain_name){
            let  rr_cache_vec = x.clone();

            let mut new_rr_cache_vec = Vec::<RRStoredData>::new();

            for mut rr_cache in rr_cache_vec{
                let rr_ip_address = match rr_cache.get_resource_record().get_rdata() {
                    Rdata::A(val) => val.get_address(),
                    _ => unreachable!(),
                };

                if rr_ip_address == ip_address{
                    rr_cache.set_response_time(response_time + rr_cache.get_response_time()/2);
                }

                new_rr_cache_vec.push(rr_cache.clone());
            }
            domain_names_data.insert(domain_name, new_rr_cache_vec);

            self.set_domain_names_data(domain_names_data);
        }
    }

    /// For each domain name, it removes the RRStoredData past its TTL.
    pub fn filter_timeout_host_data(&mut self) {
        let mut new_hash = HashMap::<DomainName, Vec<RRStoredData>>::new();
        let data = self.get_domain_names_data();
        let current_time = Utc::now();
        for (domain_name, rr_cache_vec) in data.into_iter() {
            let filtered_rr_cache_vec: Vec<RRStoredData> = rr_cache_vec
            .into_iter()
            .filter(|rr_cache| rr_cache.get_absolute_ttl() > current_time)
            .collect();

            new_hash.insert(domain_name, filtered_rr_cache_vec);
        }
        self.set_domain_names_data(new_hash);
    }

}

///setter and getter for the host data
impl CacheByDomainName{

    pub fn get_domain_names_data(&self) -> HashMap<DomainName, Vec<RRStoredData>> {
        return self.domain_names_data.clone();
    }

    pub fn set_domain_names_data(&mut self, domain_names_data: HashMap<DomainName, Vec<RRStoredData>>) {
        self.domain_names_data = domain_names_data;
    }

    pub fn get(&self,domain_name:&DomainName) -> Option<&Vec<RRStoredData>>{
        return self.domain_names_data.get(domain_name)

    }
}

#[cfg(test)]
mod host_data_test{
    use chrono::Utc;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::dns_cache::cache_by_record_type::rr_stored_data::RRStoredData;
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use std::{collections::HashMap, net::IpAddr};

    use super::CacheByDomainName;

    //Contructor test
    #[test]
    fn constructor_test(){
        let host_data = CacheByDomainName::new();
        assert!(host_data.domain_names_data.is_empty());
    }

    //Getters and setters test
    #[test]
    fn get_domain_names_data(){
        let host_data = CacheByDomainName::new();

        let domain_names_data = host_data.get_domain_names_data();

        assert!(domain_names_data.is_empty());
    }

    #[test]
    fn set_domain_names_data(){
        let mut host_data = CacheByDomainName::new();

        let mut domain_names_data = HashMap::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        domain_names_data.insert(domain_name.clone(), Vec::new());

        assert!(host_data.domain_names_data.is_empty());

        host_data.set_domain_names_data(domain_names_data.clone());

        let domain_names_data_2 = host_data.get_domain_names_data();

        assert!(!domain_names_data_2.is_empty());
    }

    //add_to_host_data test
    #[test]
    fn add_to_host_data(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let domain_names_data = host_data.get_domain_names_data();

        assert_eq!(domain_names_data.len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let rr_cache_2 = RRStoredData::new(resource_record_2);
        host_data.add_to_host_data(domain_name.clone(), rr_cache_2);

        let domain_names_data_2 = host_data.get_domain_names_data();

        assert_eq!(domain_names_data_2.len(), 1);

        let domain_names_data_vec = domain_names_data_2.get(&domain_name).unwrap();

        assert_eq!(domain_names_data_vec.len(), 2);
    }

    //remove_from_host_data test
    #[test]
    fn remove_from_host_data(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let domain_names_data = host_data.get_domain_names_data();

        assert_eq!(domain_names_data.len(), 1);

        host_data.remove_from_host_data(domain_name.clone());

        let domain_names_data_2 = host_data.get_domain_names_data();

        assert_eq!(domain_names_data_2.len(), 0);
    }

    //get_from_host_data test
    #[test]
    fn get_from_host_data(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let domain_names_data = host_data.get_domain_names_data();

        assert_eq!(domain_names_data.len(), 1);

        let domain_names_data_vec = host_data.get_from_host_data(domain_name.clone()).unwrap();

        assert_eq!(domain_names_data_vec.len(), 1);
    }

    //get_from_host_data test with no domain name
    #[test]
    fn get_from_host_data_no_domain_name(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let domain_names_data = host_data.get_domain_names_data();

        // One domain name
        assert_eq!(domain_names_data.len(), 1);

        // Assuming this test is for the case where the domain name is not in the host data
        let domain_name_2 = DomainName::new();
        let element = host_data.get_from_host_data(domain_name_2.clone());
        assert_eq!(element, None);
    }

    //get test
    #[test]
    fn get(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRStoredData::new(resource_record);
        rr_cache.set_response_time(1234433455);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let vec_rr_cache = host_data.get(&domain_name).unwrap();

        let rr_cache_o = vec_rr_cache.get(0).unwrap();

        assert_eq!(1234433455, rr_cache_o.get_response_time())
    }

    //get oldest used test
    #[test]
    fn get_oldest_used(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRStoredData::new(resource_record);
        rr_cache.set_last_use(Utc::now());
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("expected"));
    
        let mut new_vec = Vec::new();
        new_vec.push(String::from("uchile.cl"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let mut rr_cache_2 = RRStoredData::new(resource_record_2);
        rr_cache_2.set_last_use(Utc::now());
        host_data.add_to_host_data(domain_name.clone(), rr_cache_2);

        let oldest_used = host_data.get_oldest_used();
        let oldest_name = oldest_used.0;


        assert_eq!("expected".to_string(), oldest_name.get_name())        
    }

     //get insert  used test
     #[test]
     fn insert(){
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRStoredData::new(resource_record);
        rr_cache.set_response_time(12);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name, rr_cache);
        let mut domain_name_new = DomainName::new();
        domain_name_new.set_name(String::from("inserted"));
        let a_rdata_2 = Rdata::A(ARdata::new());
        let resource_record_2 = ResourceRecord::new(a_rdata_2);
        let rr_cache_2 = RRStoredData::new(resource_record_2);

        let mut rr_vec = Vec::new();
        rr_vec.push(rr_cache_2);
        let expected = host_data.insert(domain_name_new, rr_vec);
        assert_eq!(expected, None)
        
     }

    //update response time test
    #[test]
    fn update_response_time(){
        let mut host_data = CacheByDomainName::new();
        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        let mut rr_cache = RRStoredData::new(resource_record);
        rr_cache.set_response_time(1000);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);
        host_data.update_response_time(ip_address, 2000, domain_name.clone());

        let domain_names_data = host_data.get_domain_names_data();

        let rr_cache_vec = domain_names_data.get(&domain_name).unwrap();

        let rr_cache = rr_cache_vec.get(0).unwrap();

        assert_eq!(2500, rr_cache.get_response_time())
    }

    #[test]
    fn timeout_rr_cache() {
        use std::{thread, time};
        let mut host_data = CacheByDomainName::new();
        let a_rdata = Rdata::A(ARdata::new());

        let mut resource_record_valid = ResourceRecord::new(a_rdata.clone());
        resource_record_valid.set_ttl(1000);
        let rr_cache_valid = RRStoredData::new(resource_record_valid.clone());

        let mut resource_record_invalid = ResourceRecord::new(a_rdata);
        resource_record_invalid.set_ttl(4);
        let rr_cache_invalid = RRStoredData::new(resource_record_invalid);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));

        host_data.add_to_host_data(domain_name.clone(), rr_cache_valid);
        host_data.add_to_host_data(domain_name.clone(), rr_cache_invalid);

        assert_eq!(host_data.get_domain_names_data().len(), 1);
        if let Some(rr_cache_vec) = host_data.get_domain_names_data().get(&domain_name) {
            assert_eq!(rr_cache_vec.len(), 2);
        }

        println!("Before timeout: {:?}", Utc::now());
        thread::sleep(time::Duration::from_secs(5));
        println!("After timeout: {:?}", Utc::now());
        host_data.filter_timeout_host_data();

        assert_eq!(host_data.get_domain_names_data().len(), 1);
        if let Some(rr_cache_vec) = host_data.get_domain_names_data().get(&domain_name) {
            assert_eq!(rr_cache_vec.len(), 1);
            //check if the rescource record who survives is the correct
            if let Some(rrstore_data_valid) = rr_cache_vec.get(0){
                let resource_record_after_filter = rrstore_data_valid.get_resource_record();
                assert_eq!(resource_record_after_filter, resource_record_valid);
            }
        }
    }
}
