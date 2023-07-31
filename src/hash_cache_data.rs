use crate::message::type_rtype::Rtype;
use crate::rr_cache::RRCache;
use crate::hash_host_data::HostData;
use std::collections::HashMap;
use crate::domain_name::DomainName;


///struct to define the cache data
#[derive(Clone)]
pub struct CacheData {
    pub cache_data: HashMap<Rtype, HostData>,
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
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRCache::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_domain_name(String::from("uchile.cl"));
    /// cache_data.add_to_cache_data(Rtype::A, domain_name, rr_cache);
    /// ```
    /// # Arguments
    /// * `rtype` - A Rtype that represents the rtype of the cache data
    /// * `domain_name` - A DomainName that represents the domain name of the cache data
    /// * `rr_cache` - A RRCache that represents the rr_cache of the cache data

    pub fn add_to_cache_data(&mut self, rtype: Rtype, domain_name: DomainName, rr_cache:RRCache){
        let mut cache_data = self.get_cache_data();
        if let Some(x) = cache_data.get_mut(&rtype) { 
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

    ///function to remove an element from the cache data
    /// # Example
    /// ```
    /// let mut cache_data = CacheData::new();
    /// let a_rdata = Rdata::SomeARdata(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRCache::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_domain_name(String::from("uchile.cl"));
    /// cache_data.add_to_cache_data(Rtype::A, domain_name, rr_cache);
    /// cache_data.remove_from_cache_data(domain_name, Rtype::A);
    /// ```
    /// # Arguments
    /// * `domain_name` - A DomainName that represents the domain name of the cache data
    /// * `rtype` - A Rtype that represents the rtype of the cache data
    pub fn remove_from_cache_data(&mut self, domain_name: DomainName, rtype: Rtype){
        let mut cache_data = self.get_cache_data();
        if let Some(x) = cache_data.get_mut(&rtype) {
            let mut type_hash: HostData = x.clone();
            type_hash.remove_from_host_data(domain_name);
            cache_data.insert(rtype, type_hash);
            self.set_cache_data(cache_data);
        } 
    }
}

///setter and getter for the host data
impl CacheData{

    pub fn get_cache_data(&self) -> HashMap<Rtype, HostData> {
        return self.cache_data.clone();
    }

    pub fn set_cache_data(&mut self, cache_data: HashMap<Rtype, HostData>) {
        self.cache_data = cache_data;
    }
}

#[cfg(test)]
mod cache_data_test{
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::rr_cache::RRCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::hash_host_data::HostData;
    use std::collections::HashMap;

    use super::CacheData;

    //Constructor test
    #[test]
    fn constructor_test(){
        let cache_data = CacheData::new();

        assert!(cache_data.cache_data.is_empty());
    }

    //Getter and setter test
    #[test]
    fn get_cache_data(){
        let cache_data = CacheData::new();

        let cache_data_hash = cache_data.get_cache_data();

        assert!(cache_data_hash.is_empty());
    }
}