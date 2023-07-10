pub mod type_rtype;

use crate::message::type_rtype::Rtype;
use crate::rr_cache::RRCache;
use crate::host_data;
use std::collections::HashMap;

///type to define the rtype of the cache data
type rtype = Rtype;

///type to denine the host data
type host_data = HostData;

///struct to define the cache data
#[derive(Clone)]
pub struct CacheData {
    pub cache_hash: HashMap<rtype, host_data>,
}

/// functions for the cache data
impl CacheData{
    /// function to create a new CacheData
    /// Example
    /// ```
    /// let cache_data = CacheData::new();
    /// ```
    pub fn new() -> CacheData {
        CacheData {
            cache_data: HashMap::new(),
        }
    }

    ///function to add a new element into the cache_data
    /// # Example
    /// ```
    /// let mut cache_data = CacheData::new();
    /// let rr_cache = RRCache::new();
    /// cache_data.add_to_cache_data(Rtype::A, String::from("uchile.cl"), rr_cache);
    /// ```
    /// # Arguments
    /// * `rtype` - A Rtype that represents the rtype of the cache data
    /// * `domain_name` - A String that represents the domain name of the cache data
    /// * `rr_cache` - A RRCache that represents the rr_cache of the cache data

    pub fn add_to_cache_data(&mut self, rtype: Rtype, domain_name: String, rr_cache:RRCache){
        let mut cache_data = self.get_cache_data();
        rr_type_str = Rtype::from_rtype_to_str(rtype);
        if let Some(x) = cache_data.get_mut(&rr_type_str) { 
            let mut type_hash: HostData = x.clone();
            type_hash.add_to_host_data(domain_name, rr_cache);
            cache_data.insert(rtype, type_hash);
        }
        else {
            let mut type_hash: HostData = HostData::new();
            type_hash.add_to_host_data(domain_name, rr_cache);
            cache_data.insert(rtype, type_hash);
        }
        self.set_cache_data(cache_data);
    }

    pub fn remove_from_cache_data(&mut self, domain_name: String, rtype: Rtype){
        
    }
}

///setter and getter for the host data
impl CacheData{

    pub fn get_cache_data(&self) -> HashMap<rtype, host_data> {
        return self.cache_data.clone();
    }

    pub fn set_cache_data(&mut self, cache_data: HashMap<rtype, host_data>) {
        self.cache_data = cache_data;
    }
}