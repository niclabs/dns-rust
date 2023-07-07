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
    /// let mut host_data = HostData::new();
    /// let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["dcc".to_string()]));
    /// let mut resource_record = ResourceRecord::new(txt_rdata);
    /// let rtype = resource_record.get_rtype();
    /// cache_data.add_to_cache_data(rtype,host_data);
    /// ```
    /// 
    /// # Arguments
    /// * `rtype`  -An enum that indicates the type of the data
    /// * `host_data` -The structure HostData that contains host name and host cache

    pub fn add_to_cache_data(&mut self, rtype: Rtype, host_data: HostData){


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