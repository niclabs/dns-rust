use crate::{rr_cache::RRCache, domain_name::DomainName};
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
    /// let rr_cache = RRCache::new();
    /// let domain_name = DomainName::new();
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
    /// let rr_cache = RRCache::new();
    /// let domain_name = DomainName::new();
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
}

///setter and getter for the host data
impl HostData{

    pub fn get_host_hash(&self) -> HashMap<DomainName, Vec<RRCache>> {
        return self.host_hash.clone();
    }

    pub fn set_host_hash(&mut self, host_hash: HashMap<DomainName, Vec<RRCache>>) {
        self.host_hash = host_hash;
    }
}

#[cfg(test)]
mod host_data_test{
    use crate::rr_cache::RRCache;
    use std::collections::HashMap;

    //Contructor test
    #[test]
    fn constructor_test(){
    //    let host_data = HostData::new();
        
    //    assert_eq!(host_data.host_hash.is_empty(), true);
    }
}
