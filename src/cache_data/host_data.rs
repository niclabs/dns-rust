use chrono::Utc;

use crate::{rr_cache::RRCache, domain_name::DomainName, message::type_rtype::Rtype};
use std::collections::HashMap;

///type to define the name of the host

///struct to define the host data
#[derive(Clone)]
pub struct HostData {
    pub host_hash: HashMap<DomainName, Vec<RRCache>>,
}

///functions for the host data
impl HostData{

    ///function to create a new host data
    /// # Example
    /// ```
    /// let host_data = HostData::new();
    /// ```
    pub fn new() -> HostData {
        HostData {
            host_hash: HashMap::new(),
        }
    }

    ///function to add a rr_cache to the host data
    /// # Example
    /// ```
    /// let mut host_data = HostData::new();
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRCache::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// ```
    /// # Arguments
    /// * `host_name` - A Domain Name that represents the name of the host
    /// * `rr_cache` - A RRCache that represents the rr_cache of the host
    pub fn add_to_host_data(&mut self, host_name: DomainName, rr_cache: RRCache) {
        let mut host_hash = self.get_host_hash();
        if let Some(y) = host_hash.get_mut(&host_name){
            let mut rr_cache_vec = y.clone();
            rr_cache_vec.push(rr_cache);
            host_hash.insert(host_name, rr_cache_vec);
        }
        else{
            let mut rr_cache_vec = Vec::new();
            rr_cache_vec.push(rr_cache);
            host_hash.insert(host_name, rr_cache_vec);
        }
        self.set_host_hash(host_hash);
    }

    ///function to remove an element from the host data
    /// # Example
    /// ```
    /// let mut host_data = HostData::new();
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRCache::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// host_data.remove_from_host_data(domain_name);
    /// ```
    pub fn remove_from_host_data(&mut self, host_name: DomainName){
        let mut host_hash = self.get_host_hash();
        if let Some(_x) = host_hash.remove(&host_name){
            self.set_host_hash(host_hash);    
        }
    }

    ///function to get an element from the host data
    /// # Example
    /// ```
    /// let mut host_data = HostData::new();
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRCache::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// let host_data_2_vec = host_data.get_from_host_data(domain_name);
    /// ```
    pub fn get_from_host_data(&self, host_name: DomainName) -> Option<Vec<RRCache>>{
        let host_hash = self.get_host_hash();
        if let Some(x) = host_hash.get(&host_name){
            return Some(x.clone());
        }
        else{
            return None;
        }
    }

    ///function to get the oldest used 
    /// # Example
    /// ```
    /// let mut host_data = HostData::new();
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRCache::new(resource_record);
    /// rr_cache.set_last_use(Utc::now());
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// let host_data_2 = get_oldest_used.get_from_host_data(domain_name);
    /// ```
    pub fn get_oldest_used(&mut self) -> DomainName{
        let host = self.get_host_hash();
        let mut used_in = Utc::now();

        let mut oldest_used_domain_name = DomainName::new();

        for (host_key, host_value) in host {
            let rr_last_use = host_value[0].get_last_use();

            if rr_last_use <= used_in {
                used_in = rr_last_use;
                oldest_used_domain_name = host_key.clone();
                
            }
        }

        return oldest_used_domain_name

    }
    
    ///function to insert a domain name and a new value to be associated
    /// and return the value that was associated before, or none if 
    /// the domain name didn't exist before
    /// # Example
    /// ```
    /// let mut host_data = HostData::new();
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let mut rr_cache = RRCache::new(resource_record);
    /// rr_cache.set_last_use(Utc::now());
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_name(String::from("uchile.cl"));
    /// host_data.add_to_host_data(domain_name, rr_cache);
    /// let mut domain_name_new = DomainName::new();
    /// domain_name_new.set_name(String::from("inserted"));
    /// let a_rdata_2 = Rdata::SomeARdata(ARdata::new());
    /// let resource_record_2 = ResourceRecord::new(a_rdata);
    /// let mut rr_cache_2 = RRCache::new(resource_record);
    /// host_data.insert(domain_name_new, rr_cache_2);
    /// ```
    pub fn insert(&mut self,domain_name:DomainName, rr_cache_vec : Vec<RRCache>) -> Option<Vec<RRCache>>{
        return self.host_hash.insert(domain_name, rr_cache_vec)
    }
}

///setter and getter for the host data
impl HostData{

    pub fn get_host_hash(&self) -> HashMap<DomainName, Vec<RRCache>> {
        return self.host_hash.clone();
    }

    pub fn set_host_hash(&mut self, host_hash: HashMap<DomainName, Vec<RRCache>>) {
        self.host_hash = host_hash;
    }

    pub fn get(&self,domain_name:&DomainName) -> Option<&Vec<RRCache>>{
        return self.host_hash.get(domain_name)

    }
}

#[cfg(test)]
mod host_data_test{
    use chrono::Utc;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::rr_cache::RRCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use std::collections::HashMap;

    use super::HostData;

    //Contructor test
    #[test]
    fn constructor_test(){
        let host_data = HostData::new();
        assert!(host_data.host_hash.is_empty());
    }

    //Getters and setters test
    #[test]
    fn get_host_hash(){
        let host_data = HostData::new();

        let host_hash = host_data.get_host_hash();

        assert!(host_hash.is_empty());
    }

    #[test]
    fn set_host_hash(){
        let mut host_data = HostData::new();

        let mut host_hash = HashMap::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_hash.insert(domain_name.clone(), Vec::new());

        assert!(host_data.host_hash.is_empty());

        host_data.set_host_hash(host_hash.clone());

        let host_hash_2 = host_data.get_host_hash();

        assert!(!host_hash_2.is_empty());
    }

    //add_to_host_data test
    #[test]
    fn add_to_host_data(){
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRCache::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let host_hash = host_data.get_host_hash();

        assert_eq!(host_hash.len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::SomeTxtRdata(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let rr_cache_2 = RRCache::new(resource_record_2);
        host_data.add_to_host_data(domain_name.clone(), rr_cache_2);

        let host_hash_2 = host_data.get_host_hash();

        assert_eq!(host_hash_2.len(), 1);

        let host_hash_vec = host_hash_2.get(&domain_name).unwrap();

        assert_eq!(host_hash_vec.len(), 2);
    }

    //remove_from_host_data test
    #[test]
    fn remove_from_host_data(){
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRCache::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let host_hash = host_data.get_host_hash();

        assert_eq!(host_hash.len(), 1);

        host_data.remove_from_host_data(domain_name.clone());

        let host_hash_2 = host_data.get_host_hash();

        assert_eq!(host_hash_2.len(), 0);
    }

    //get_from_host_data test
    #[test]
    fn get_from_host_data(){
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRCache::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let host_hash = host_data.get_host_hash();

        assert_eq!(host_hash.len(), 1);

        let host_hash_vec = host_data.get_from_host_data(domain_name.clone()).unwrap();

        assert_eq!(host_hash_vec.len(), 1);
    }

    //get_from_host_data test with no domain name
    #[test]
    fn get_from_host_data_no_domain_name(){
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRCache::new(resource_record);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name.clone(), rr_cache);

        let host_hash = host_data.get_host_hash();

        assert_eq!(host_hash.len(), 1);

        let domain_name_2 = DomainName::new();
        let host_hash_vec = host_data.get_from_host_data(domain_name_2.clone());

        assert!(host_hash_vec.is_none());
    }

    //get test
    #[test]
    fn get(){
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRCache::new(resource_record);
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
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRCache::new(resource_record);
        rr_cache.set_last_use(Utc::now());
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("expected"));
    
        let mut new_vec = Vec::new();
        new_vec.push(String::from("uchile.cl"));
        let text_rdata = Rdata::SomeTxtRdata(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let mut rr_cache_2 = RRCache::new(resource_record_2);
        rr_cache_2.set_last_use(Utc::now());
        host_data.add_to_host_data(domain_name.clone(), rr_cache_2);

        let oldest_used = host_data.get_oldest_used();
        let oldest_name = oldest_used.get_name();

        assert_eq!("expected".to_string(), oldest_name)        
    }

     //get insert  used test
     #[test]
     fn insert(){
        let mut host_data = HostData::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRCache::new(resource_record);
        rr_cache.set_response_time(12);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        host_data.add_to_host_data(domain_name, rr_cache);
        let mut domain_name_new = DomainName::new();
        domain_name_new.set_name(String::from("inserted"));
        let a_rdata_2 = Rdata::SomeARdata(ARdata::new());
        let resource_record_2 = ResourceRecord::new(a_rdata_2);
        let rr_cache_2 = RRCache::new(resource_record_2);

        let mut rr_vec = Vec::new();
        rr_vec.push(rr_cache_2);
        let expected = host_data.insert(domain_name_new, rr_vec);
        assert_eq!(expected, None)
        
     }


}