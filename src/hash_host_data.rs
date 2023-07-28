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
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::rr_cache::RRCache;
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use std::arch::x86_64::_CMP_NEQ_OS;
    use std::collections::HashMap;

    use super::HostData;

    //Contructor test
    #[test]
    fn constructor_test(){
        let host_data = HostData::new();
        assert!(host_data.host_hash.is_empty());
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

        assert_eq!(host_hash.len(), 1);

        let host_hash_vec = host_hash.get(&domain_name).unwrap();

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

        assert_eq!(host_hash.len(), 0);
    }
}
