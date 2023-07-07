use crate::rr_cache::RRCache;
use std::collections::HashMap;

///type to define the name of the host
type host_name = String;

///struct to define the host data
#[derive(Clone)]
pub struct HostData {
    pub host_hash: HashMap<host_name, Vec<RRcache>>,
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
    /// host_data.add_to_host_data(String::from("uchile.cl"), rr_cache);
    /// ```
    /// # Arguments
    /// * `host_name` - A String that represents the name of the host
    /// * `rr_cache` - A RRCache that represents the rr_cache of the host
    pub fn add_to_host_data(&mut self, host_name: String, rr_cache: RRCache) {
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
    /// host_data.add_to_host_data(String::from("uchile.cl"), rr_cache);
    /// host_data.remove_from_host_data(String::from("uchile.cl"));
    /// ```
    pub fn remove_from_host_data(&mut self, host_name: String){
        let mut host_hash = self.get_host_hash();
        if let Some(x) = host_hash.remove(&host_name){
            self.set_host_hash(host_hash);    
        }
    }
}

///setter and getter for the host data
impl HostData{

    pub fn get_host_hash(&self) -> HashMap<host_name, Vec<RRCache>> {
        return self.host_hash.clone();
    }

    pub fn set_host_hash(&mut self, host_hash: HashMap<host_name, Vec<RRCache>>) {
        self.host_hash = host_hash;
    }
}