pub mod cache_by_domain_name;
pub mod rr_stored_data;

use chrono::Utc;
use crate::message::type_rtype::Rtype;
use std::net::IpAddr;
use crate::dns_cache::cache_by_record_type::cache_by_domain_name::CacheByDomainName;
use std::collections::HashMap;
use crate::domain_name::DomainName;
use self::rr_stored_data::RRStoredData;


/// Struct that represents the cache data of the DNS cache by record type.
#[derive(Clone, Debug)]
pub struct CacheByRecordType {
    /// HashMap that represents the cache data of the DNS cache by record type.
    /// 
    /// The key is the record type and the value is the cache data of the DNS 
    /// cache by domain name.
    record_types_data: HashMap<Rtype, CacheByDomainName>,
}

/// functions for the cache data
impl CacheByRecordType{
    /// function to create a new CacheByRecordType
    /// Example
    /// ```
    /// let cache_data = CacheByRecordType::new();
    /// ```
    pub fn new() -> CacheByRecordType {
        CacheByRecordType {
            record_types_data: HashMap::new(),
        }
    }

    ///function to add a new element into the cache_data
    /// # Example
    /// ```
    /// let mut cache_data = CacheByRecordType::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_domain_name(String::from("uchile.cl"));
    /// cache_data.add_to_cache_data(Rtype::A, domain_name, rr_cache);
    /// ```
    /// # Arguments
    /// * `rtype` - A Rtype that represents the rtype of the cache data
    /// * `domain_name` - A DomainName that represents the domain name of the cache data
    /// * `rr_cache` - A RRStoredData that represents the rr_cache of the cache data

    pub fn add_to_cache_data(&mut self, rtype: Rtype, domain_name: DomainName, rr_cache:RRStoredData){
        let mut cache_data = self.get_cache_data();
        if let Some(x) = cache_data.get_mut(&rtype) { 
            let mut type_hash: CacheByDomainName = x.clone();
            type_hash.add_to_host_data(domain_name, rr_cache);
            cache_data.insert(rtype, type_hash);
        }
        else {
            let mut type_hash: CacheByDomainName = CacheByDomainName::new();
            type_hash.add_to_host_data(domain_name, rr_cache);
            cache_data.insert(rtype, type_hash);
        }
        self.set_cache_data(cache_data);
    }

    ///function to remove an element from the cache data
    /// # Example
    /// ```
    /// let mut cache_data = CacheByRecordType::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_domain_name(String::from("uchile.cl"));
    /// cache_data.add_to_cache_data(Rtype::A, domain_name, rr_cache);
    /// cache_data.remove_from_cache_data(domain_name, Rtype::A);
    /// ```
    /// # Arguments
    /// * `domain_name` - A DomainName that represents the domain name of the cache data
    /// * `rtype` - A Rtype that represents the rtype of the cache data
    pub fn remove_from_cache_data(&mut self, domain_name: DomainName, rtype: Rtype) -> u32{
        let mut cache_data = self.get_cache_data();
        if let Some(x) = cache_data.get_mut(&rtype) {
            let mut type_hash: CacheByDomainName = x.clone();
            let length = type_hash.remove_from_host_data(domain_name);
            cache_data.insert(rtype, type_hash);
            self.set_cache_data(cache_data);
            return length;
        }
        return 0; 
    }

    ///function to remove the oldest element from the cache data
    /// # Example
    /// ```
    /// let mut cache_data = CacheByRecordType::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_domain_name(String::from("uchile.cl"));
    /// cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache);
    /// cache_data.add_to_cache_data(Rtype::A, domain_name)
    /// cache_data.remove_oldest_used(domain_name, Rtype::A);
    /// ```
    /// # Arguments
    /// * `domain_name` - A DomainName that represents the domain name of the cache data
    /// * `rtype` - A Rtype that represents the rtype of the cache data

    pub fn remove_oldest_used(&mut self) -> u32{
        let cache = self.get_cache_data();
        let mut oldest_used_domain_name = DomainName::new();
        let mut oldest_used_type =Rtype::TXT;
        let mut oldest_time = Utc::now();

        for (rtype, mut host_data) in cache {
            let (domain_name,time)=host_data.get_oldest_used();
            if time <= oldest_time {
                oldest_used_type = rtype.clone();
                oldest_used_domain_name = domain_name;
                oldest_time = time;
            }    
        }
        
        let length = self.remove_from_cache_data(oldest_used_domain_name, oldest_used_type);
        length
    }

    ///function to get an element from the cache data
    /// # Example
    /// ```
    /// let mut cache_data = CacheByRecordType::new();
    /// let a_rdata = Rdata::A(ARdata::new());
    /// let resource_record = ResourceRecord::new(a_rdata);
    /// let rr_cache = RRStoredData::new(resource_record);
    /// let mut domain_name = DomainName::new();
    /// domain_name.set_domain_name(String::from("uchile.cl"));
    /// 
    /// cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache);
    /// 
    /// let rr_cache = cache_data.get_from_cache_data(domain_name.clone(), Rtype::A);
    /// ```
    /// # Arguments
    /// * `domain_name` - A DomainName that represents the domain name of the cache data
    /// * `rtype` - A Rtype that represents the rtype of the cache data
    pub fn get_from_cache_data(&mut self, domain_name: DomainName, rtype: Rtype) -> Option<Vec<RRStoredData>>{
        let mut cache_data = self.get_cache_data();
        if let Some(x) = cache_data.get(&rtype) {
            let mut type_hash: CacheByDomainName = x.clone();
            let rr_cache_vec = type_hash.get_from_host_data(domain_name); 
            cache_data.insert(rtype, type_hash);
            self.set_cache_data(cache_data);
            return rr_cache_vec;
        }
        else {
            return None;
        }
    }

    /// Removes the cache data that has expired.
    /// 
    /// For each type of cache data, it removes the cache data that has expired, using
    /// the `timeout_rr_cache` method of the `CacheByDomainName` struct. If the `CacheByDomainName` struct
    /// is empty after the removal, it is removed from the cache data.
    pub fn filter_timeout_cache_data(&mut self) {
        let cache_data = self.get_cache_data();
        let clean_cache_data: HashMap<Rtype, CacheByDomainName> = cache_data
        .into_iter()
        .filter_map(|(rtype, mut host_data)| {
            host_data.filter_timeout_host_data();
            if host_data.get_domain_names_data().is_empty() {
                None
            } else {
                Some((rtype, host_data))
            }
        })
        .collect();
        self.set_cache_data(clean_cache_data);

    }

    pub fn update_response_time(&mut self,
        domain_name: DomainName,
        rr_type: Rtype,
        response_time: u32,
        ip_address: IpAddr,
    ) {
        let mut cache = self.get_cache_data();
        if let Some(x) = cache.get(&rr_type) {
            let mut new_x = x.clone();
            new_x.update_response_time(ip_address, response_time, domain_name);
            cache.insert(rr_type, new_x);
            self.set_cache_data(cache);
        }

    }

    pub fn insert(&mut self,rtype:Rtype, host_data: CacheByDomainName) {
        self.record_types_data.insert(rtype, host_data);

    }

    pub fn iter(&mut self) -> std::collections::hash_map::Iter<'_, Rtype, CacheByDomainName>{
        return self.record_types_data.iter()

    }
}

///setter and getter for the host data
impl CacheByRecordType{

    pub fn get_cache_data(&self) -> HashMap<Rtype, CacheByDomainName> {
        return self.record_types_data.clone();
    }

    pub fn set_cache_data(&mut self, cache_data: HashMap<Rtype, CacheByDomainName>) {
        self.record_types_data = cache_data;
    }

    pub fn get(&self, rtype : Rtype) -> Option<&CacheByDomainName>{
         return self.record_types_data.get(&rtype);
    }
}

#[cfg(test)]
mod cache_data_test{
    use chrono::{Utc, Duration};
    //use std::thread::sleep;
    //use std::time::Duration as StdDuration;

    use crate::message::rdata::{txt_rdata::TxtRdata, ns_rdata::NsRdata};
    use crate::message::type_rtype::Rtype;
    use crate::dns_cache::cache_by_record_type::rr_stored_data::RRStoredData;    
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::dns_cache::cache_by_record_type::cache_by_domain_name::CacheByDomainName;
    use std::{collections::HashMap, net::IpAddr};



    use super::CacheByRecordType;

    //Constructor test
    #[test]
    fn constructor_test(){
        let cache_data = CacheByRecordType::new();

        assert!(cache_data.record_types_data.is_empty());
    }

    //Getter and setter test
    #[test]
    fn get_cache_data(){
        let cache_data = CacheByRecordType::new();

        let cache_data_hash = cache_data.get_cache_data();

        assert!(cache_data_hash.is_empty());
    }

    #[test]
    fn set_cache_data(){
        let mut cache_data = CacheByRecordType::new();

        let mut cache_data_hash = HashMap::new();
        let mut host_data = CacheByDomainName::new();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);
        host_data.add_to_host_data(domain_name, rr_cache);
        cache_data_hash.insert(Rtype::A, host_data);

        cache_data.set_cache_data(cache_data_hash);

        assert_eq!(cache_data.get_cache_data().len(), 1);
    }

    //Add to cache data test
    #[test]
    fn add_to_cache_data(){
        let mut cache_data = CacheByRecordType::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);

        cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache);

        assert_eq!(cache_data.get_cache_data().len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let rr_cache_2 = RRStoredData::new(resource_record_2);

        cache_data.add_to_cache_data(Rtype::TXT, domain_name.clone(), rr_cache_2);

        assert_eq!(cache_data.get_cache_data().len(), 2);

        let a_rdata_2 = Rdata::A(ARdata::new());
        let resource_record_3 = ResourceRecord::new(a_rdata_2);
        let rr_cache_3 = RRStoredData::new(resource_record_3);

        cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache_3);

        assert_eq!(cache_data.get_cache_data().len(), 2);
    }

    //Remove from cache data test
    #[test]
    fn remove_from_cache_data(){
        let mut cache_data = CacheByRecordType::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);

        cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache);

        assert_eq!(cache_data.get_cache_data().len(), 1);

        cache_data.remove_from_cache_data(domain_name.clone(), Rtype::A);

        let cache_hash = cache_data.get_cache_data();

        let host_data = cache_hash.get(&Rtype::A).unwrap();

        assert!(host_data.get_domain_names_data().is_empty());
    }

    //Get from cache data test
    #[test]
    fn get_from_cache_data(){
        let mut cache_data = CacheByRecordType::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let rr_cache = RRStoredData::new(resource_record);

        cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache);

        assert!(!cache_data.get_cache_data().is_empty());

        let rr_cache_vec = cache_data.get_from_cache_data(domain_name.clone(), Rtype::A).unwrap();

        assert_eq!(rr_cache_vec.len(), 1);

        let mut new_vec = Vec::new();
        new_vec.push(String::from("hola"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let rr_cache_2 = RRStoredData::new(resource_record_2);

        cache_data.add_to_cache_data(Rtype::TXT, domain_name.clone(), rr_cache_2);

        let rr_cache_vec_2 = cache_data.get_from_cache_data(domain_name.clone(), Rtype::TXT).unwrap();

        assert_eq!(rr_cache_vec_2.len(), 1);

        let rr_cache_vec_3 = cache_data.get_from_cache_data(DomainName::new(), Rtype::A);

        assert!(rr_cache_vec_3.is_none());
    }

    //remove oldest used test
    #[test]
    fn remove_oldest_used(){
        let mut cache_data = CacheByRecordType::new();

        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        let mut rr_cache = RRStoredData::new(resource_record);
        let now = Utc::now();
        let time_back = Duration::seconds(3600); 
        let new_time = now - time_back; 
        rr_cache.set_last_use(new_time);
        let mut domain_name_1 = DomainName::new();
        domain_name_1.set_name(String::from("notexpected"));
        let mut domain_name_2 = DomainName::new();
        domain_name_2.set_name(String::from("expected"));
    
        let mut new_vec = Vec::new();
        new_vec.push(String::from("uchile.cl"));
        let text_rdata = Rdata::TXT(TxtRdata::new(new_vec));
        let resource_record_2 = ResourceRecord::new(text_rdata);
        let mut rr_cache_2 = RRStoredData::new(resource_record_2);
        rr_cache_2.set_last_use(Utc::now());


        cache_data.add_to_cache_data(Rtype::A, domain_name_1.clone(), rr_cache);
        cache_data.add_to_cache_data(Rtype::TXT, domain_name_2.clone(), rr_cache_2);

        let _vec_rr_cache_a = cache_data.get_from_cache_data(domain_name_1.clone(), Rtype::A).unwrap();
        
        let a = cache_data.remove_oldest_used();
        
        let vec_rr_cache_txt_expected = cache_data.get_from_cache_data(domain_name_2, Rtype::TXT);
        let vec_rr_cache_a_expected = cache_data.get_from_cache_data(domain_name_1.clone(), Rtype::A).unwrap();

        assert_eq!(a,1);
        assert_eq!(vec_rr_cache_a_expected.len(), 1);
        assert!(vec_rr_cache_txt_expected.is_none());
    }

    //update response time test
    #[test]
    fn update_response_time(){
        let mut cache_data = CacheByRecordType::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        let mut rr_cache = RRStoredData::new(resource_record);
        rr_cache.set_response_time(1000);

        cache_data.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache);

        cache_data.update_response_time(domain_name.clone(), Rtype::A, 2000, ip_address.clone());

        let rr_cache_vec = cache_data.get_from_cache_data(domain_name.clone(), Rtype::A).unwrap();

        for rr_cache in rr_cache_vec {
            assert_eq!(rr_cache.get_response_time(), 2500);
        }
    }

    #[test]
    fn filter_timeout_cache_data_rtype_a() {
        use std::{thread, time};
        let mut cache_record_type = CacheByRecordType::new();
        let a_rdata = Rdata::A(ARdata::new());
        
        let mut resource_record_valid = ResourceRecord::new(a_rdata.clone());
        resource_record_valid.set_ttl(1000);
        let rr_cache_valid = RRStoredData::new(resource_record_valid.clone());

        let mut resource_record_invalid = ResourceRecord::new(a_rdata);
        resource_record_invalid.set_ttl(4);
        let rr_cache_invalid = RRStoredData::new(resource_record_invalid);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));

        cache_record_type.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache_valid);
        cache_record_type.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache_invalid);

        //check if the domain with A type has 2 RRStoredData
        if let Some(rr_cache_vec) = cache_record_type.get_from_cache_data(domain_name.clone(), Rtype::A){
            assert_eq!(rr_cache_vec.len(), 2);
        }

        println!("Before timeout: {:?}", Utc::now());
        thread::sleep(time::Duration::from_secs(5));
        println!("After timeout: {:?}", Utc::now());
        cache_record_type.filter_timeout_cache_data();

        //check if the len is 1 instead of 2 (one RRStoredData was eliminated)
        if let Some(rr_cache_vec) = cache_record_type.get_from_cache_data(domain_name.clone(), Rtype::A){
            assert_eq!(rr_cache_vec.len(), 1);
            //chek if the resource record who survives is the right one
            if let Some(rrstore_data_valid) = rr_cache_vec.get(0){
                let resource_record_after_filter = rrstore_data_valid.get_resource_record();
                assert_eq!(resource_record_after_filter, resource_record_valid);
            }
        }
        
    }

    #[test]
    fn filter_timeout_cache_data_rtype_ns() {
        use std::{thread, time};
        let mut cache_record_type = CacheByRecordType::new();
        let ns_rdata = Rdata::NS(NsRdata::new());
        
        let mut resource_record_valid = ResourceRecord::new(ns_rdata.clone());
        resource_record_valid.set_ttl(1000);
        let rr_cache_valid = RRStoredData::new(resource_record_valid.clone());

        let mut resource_record_invalid = ResourceRecord::new(ns_rdata);
        resource_record_invalid.set_ttl(4);
        let rr_cache_invalid = RRStoredData::new(resource_record_invalid);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));

        cache_record_type.add_to_cache_data(Rtype::NS, domain_name.clone(), rr_cache_valid);
        cache_record_type.add_to_cache_data(Rtype::NS, domain_name.clone(), rr_cache_invalid);

        //check if the domain with A type has 2 RRStoredData
        if let Some(rr_cache_vec) = cache_record_type.get_from_cache_data(domain_name.clone(), Rtype::NS){
            assert_eq!(rr_cache_vec.len(), 2);
        }

        println!("Before timeout: {:?}", Utc::now());
        thread::sleep(time::Duration::from_secs(5));
        println!("After timeout: {:?}", Utc::now());
        cache_record_type.filter_timeout_cache_data();

        //check if the len is 1 instead of 2 (one RRStoredData was eliminated)
        if let Some(rr_cache_vec) = cache_record_type.get_from_cache_data(domain_name.clone(), Rtype::NS){
            assert_eq!(rr_cache_vec.len(), 1);
            //chek if the resource record who survives is the right one
            if let Some(rrstore_data_valid) = rr_cache_vec.get(0){
                let resource_record_after_filter = rrstore_data_valid.get_resource_record();
                assert_eq!(resource_record_after_filter, resource_record_valid);
            }
        }
        
    }

    #[test]
    fn filter_timout_cache_data_2_differents_rtypes(){
        use std::{thread, time};
        let mut cache_record_type = CacheByRecordType::new();
        let a_rdata = Rdata::A(ARdata::new());
        let ns_rdata = Rdata::NS(NsRdata::new());
        
        let mut resource_record_valid = ResourceRecord::new(a_rdata.clone());
        resource_record_valid.set_ttl(1000);
        let rr_cache_valid = RRStoredData::new(resource_record_valid.clone());

        let mut resource_record_invalid = ResourceRecord::new(ns_rdata);
        resource_record_invalid.set_ttl(4);
        let rr_cache_invalid = RRStoredData::new(resource_record_invalid);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));

        cache_record_type.add_to_cache_data(Rtype::A, domain_name.clone(), rr_cache_valid);
        cache_record_type.add_to_cache_data(Rtype::NS, domain_name.clone(), rr_cache_invalid);

        //check if every record_types_data (HashMap for A and for NS) has 1 element 
        let record_types_data = cache_record_type.get_cache_data();
        //CacheByDomainName for A type
        if let Some(record_types_data_a) = record_types_data.get(&Rtype::A) {
            if let Some(rrstore_data_vec_a) = record_types_data_a.clone().get_from_host_data(domain_name.clone()){
                assert_eq!(rrstore_data_vec_a.len(), 1);
            }
        }
        //CacheByDomainName for NS type
        if let Some(record_types_data_ns) = record_types_data.get(&Rtype::NS) {
            if let Some(rrstore_data_vec_ns) = record_types_data_ns.clone().get_from_host_data(domain_name.clone()){
                assert_eq!(rrstore_data_vec_ns.len(), 1);
            }
        }

        println!("Before timeout: {:?}", Utc::now());
        thread::sleep(time::Duration::from_secs(5));
        println!("After timeout: {:?}", Utc::now());
        cache_record_type.filter_timeout_cache_data();

        if let Some(record_types_data_a) = record_types_data.get(&Rtype::A) {
            if let Some(rrstore_data_vec_a) = record_types_data_a.clone().get_from_host_data(domain_name.clone()){
                assert_eq!(rrstore_data_vec_a.len(), 1);
            }
        }

        if let Some(record_types_data_ns) = record_types_data.get(&Rtype::NS) {
            println!("{:?}", record_types_data_ns);
            assert!(false, "Si habia algo dentro del Rtype NS");
        } else {
            assert!(true);
        }

    

    }
}