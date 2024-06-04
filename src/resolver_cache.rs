use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::resource_record::ResourceRecord;
use crate::message::type_qtype::Qtype;
use crate::message::type_rtype::Rtype;
use crate::message::class_qclass::Qclass;
use crate::message::rcode::Rcode;
use crate::message::DnsMessage;

use std::num::NonZeroUsize;

#[derive(Clone, Debug)]
pub struct ResolverCache {
    cache_answer: DnsCache,
    cache_authority: DnsCache,
    cache_additional: DnsCache,
}

impl ResolverCache {

    /// Create a new ResolverCache with the given size.
    pub fn new(size: Option<NonZeroUsize>) -> Self {
        let size = size.unwrap_or(NonZeroUsize::new(1667).unwrap());
        Self {
            cache_answer: DnsCache::new(Some(size)),
            cache_authority: DnsCache::new(Some(size)),
            cache_additional: DnsCache::new(Some(size)),
        }
    }

    /// Create a new ResolverCache with the given sizes.
    pub fn with_sizes(
        size_answer: Option<NonZeroUsize>,
        size_authority: Option<NonZeroUsize>,
        size_additional: Option<NonZeroUsize>,
    ) -> Self {
        Self {
            cache_answer: DnsCache::new(size_answer),
            cache_authority: DnsCache::new(size_authority),
            cache_additional: DnsCache::new(size_additional),
        }
    }

    /// Add an element to the answer cache.
    pub fn add_answer(&mut self, domain_name: DomainName, resource_record: ResourceRecord, qtype: Qtype, qclass: Qclass, rcode: Option<Rcode>) {
        if resource_record.get_ttl() > 0 {
            self.cache_answer.add(domain_name, resource_record, qtype, qclass, rcode);
        }
    }

    /// Add an element to the authority cache.
    pub fn add_authority(&mut self, domain_name: DomainName, resource_record: ResourceRecord, qtype: Qtype, qclass: Qclass, rcode: Option<Rcode>) {
        if resource_record.get_ttl() > 0 {
            self.cache_authority.add(domain_name, resource_record, qtype, qclass, rcode);
        }
    }

    /// Add an element to the additional cache.
    pub fn add_additional(&mut self, domain_name: DomainName, resource_record: ResourceRecord, qtype: Qtype, qclass: Qclass, rcode: Option<Rcode>) {
        if resource_record.get_ttl() > 0 {
            if resource_record.get_rtype() != Rtype::OPT {
                self.cache_additional.add(domain_name, resource_record, qtype, qclass, rcode);
            }
        }
    }

    /// Adds an answer to the cache
    pub fn add(&mut self, message: DnsMessage) {
        let qname = message.get_question().get_qname();
        let qtype = message.get_question().get_qtype();
        let qclass = message.get_question().get_qclass();

        let answers = message.get_answer();
        let authorities = message.get_authority();
        let additionals = message.get_additional();

        let rcode = Some(message.get_header().get_rcode());

        answers.iter()
        .for_each(|rr| {
            self.add_answer(qname.clone(), rr.clone(), qtype, qclass, rcode);
        
        });

        println!("authority: {:?}", authorities.len());

        authorities.iter()
        .for_each(|rr| {
            println!("Adding authority");
            self.add_authority(qname.clone(), rr.clone(), qtype, qclass, rcode);
            
        });

        additionals.iter()
        .for_each(|rr| {
                self.add_additional(qname.clone(), rr.clone(), qtype, qclass, rcode);
        });
    }

    /// Gets elements from the answer cache
    pub fn get_answer(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) -> Option<Vec<ResourceRecord>> {
        let rr_stored_data = self.cache_answer.get(domain_name, qtype, qclass);

        if let Some(rr_stored_data) = rr_stored_data {
            let mut rr_vec = Vec::new();
            for rr_data in rr_stored_data {
                rr_vec.push(rr_data.get_resource_record().clone());
            }
            Some(rr_vec)
        } else {
            None
        }
    }

    /// Gets elements from the authority cache
    pub fn get_authority(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) -> Option<Vec<ResourceRecord>> {
        let rr_stored_data = self.cache_authority.get(domain_name, qtype, qclass);

        if let Some(rr_stored_data) = rr_stored_data {
            let mut rr_vec = Vec::new();
            for rr_data in rr_stored_data {
                rr_vec.push(rr_data.get_resource_record().clone());
            }
            Some(rr_vec)
        } else {
            None
        }
    }

    /// Gets elements from the additional cache
    pub fn get_additional(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) -> Option<Vec<ResourceRecord>> {
        let rr_stored_data = self.cache_additional.get(domain_name, qtype, qclass);

        if let Some(rr_stored_data) = rr_stored_data {
            let mut rr_vec = Vec::new();
            for rr_data in rr_stored_data {
                rr_vec.push(rr_data.get_resource_record().clone());
            }
            Some(rr_vec)
        } else {
            None
        }
    }

    pub fn get_rcode(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) -> Option<Rcode> {
        let rr_stored_data = self.cache_answer.get(domain_name, qtype, qclass);

        if let Some(rr_stored_data) = rr_stored_data {
            Some(rr_stored_data[0].get_rcode())
        } else {
            None
        }
    }

    /// Gets an response from the cache
    pub fn get(&mut self, query: DnsMessage) -> Option<DnsMessage> {
        self.timeout();

        let domain_name = query.get_question().get_qname();
        let qtype = query.get_question().get_qtype();
        let qclass = query.get_question().get_qclass();

        let mut message = DnsMessage::new();
        let mut header = query.get_header();
        let rcode = self.get_rcode(domain_name.clone(), qtype, qclass);
        header.set_rcode(rcode.unwrap_or(Rcode::NOERROR));

        let question = query.get_question().clone();

        let query_id = query.get_query_id();

        message.set_header(header);
        message.set_question(question);
        message.set_query_id(query_id);

        let answers = self.get_answer(domain_name.clone(), qtype, qclass);
        let authorities = self.get_authority(domain_name.clone(), qtype, qclass);
        let additionals = self.get_additional(domain_name.clone(), qtype, qclass);

        if let Some(answers) = answers {
            message.set_answer(answers);
        }

        if let Some(authorities) = authorities {
            message.set_authority(authorities);
        }

        if let Some(additionals) = additionals {
            message.set_additional(additionals);
        }

        if message.get_answer().is_empty() && 
           message.get_authority().is_empty() && 
           message.get_additional().is_empty() {
            None
        } else {
            Some(message)
        }
    }

    /// Removes an element from the answer cache.
    pub fn remove_answer(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        self.cache_answer.remove(domain_name, qtype, qclass);
    }

    /// Removes an element from the authority cache.
    pub fn remove_authority(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        self.cache_authority.remove(domain_name, qtype, qclass);
    }

    /// Removes an element from the additional cache.
    pub fn remove_additional(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        self.cache_additional.remove(domain_name, qtype, qclass);
    }

    /// Removes an element from the cache.
    pub fn remove(&mut self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        self.remove_answer(domain_name.clone(), qtype, qclass);
        self.remove_authority(domain_name.clone(), qtype, qclass);
        self.remove_additional(domain_name.clone(), qtype, qclass);
    }

    /// Performs the timeout of cache by removing the elements that have expired for the answer cache.
    pub fn timeout_answer(&mut self) {
        self.cache_answer.timeout_cache();
    }

    /// Performs the timeout of cache by removing the elements that have expired for the authority cache.
    pub fn timeout_authority(&mut self) {
        self.cache_authority.timeout_cache();
    }

    /// Performs the timeout of cache by removing the elements that have expired for the additional cache.
    pub fn timeout_additional(&mut self) {
        self.cache_additional.timeout_cache();
    }

    /// Performs the timeout of cache by removing the elements that have expired.
    pub fn timeout(&mut self) {
        self.timeout_answer();
        self.timeout_authority();
        self.timeout_additional();
    }
}

impl ResolverCache {

    /// Get the answer cache.
    pub fn get_cache_answer(&self) -> &DnsCache {
        &self.cache_answer
    }

    /// Get the authority cache.
    pub fn get_cache_authority(&self) -> &DnsCache {
        &self.cache_authority
    }

    /// Get the additional cache.
    pub fn get_cache_additional(&self) -> &DnsCache {
        &self.cache_additional
    }
}


impl ResolverCache {

    /// Set the answer cache.
    pub fn set_cache_answer(&mut self, cache: DnsCache) {
        self.cache_answer = cache;
    }

    /// Set the authority cache.
    pub fn set_cache_authority(&mut self, cache: DnsCache) {
        self.cache_authority = cache;
    }

    /// Set the additional cache.
    pub fn set_cache_additional(&mut self, cache: DnsCache) {
        self.cache_additional = cache;
    }
}

#[cfg(test)]
mod resolver_cache_test{
    use super::*;
    use crate::message::type_rtype::Rtype;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::question::Question;
    use std::net::IpAddr;
    use crate::message::rdata::aaaa_rdata::AAAARdata;

    #[test]
    fn constructor_test() {
        let resolver_cache = ResolverCache::new(None);
        assert_eq!(resolver_cache.get_cache_answer().get_max_size(), NonZeroUsize::new(1667).unwrap());
        assert_eq!(resolver_cache.get_cache_authority().get_max_size(), NonZeroUsize::new(1667).unwrap());
        assert_eq!(resolver_cache.get_cache_additional().get_max_size(), NonZeroUsize::new(1667).unwrap());
    }

    #[test]
    fn with_sizes_test() {
        let resolver_cache = ResolverCache::with_sizes(Some(NonZeroUsize::new(100).unwrap()), Some(NonZeroUsize::new(200).unwrap()), Some(NonZeroUsize::new(300).unwrap()));
        assert_eq!(resolver_cache.get_cache_answer().get_max_size(), NonZeroUsize::new(100).unwrap());
        assert_eq!(resolver_cache.get_cache_authority().get_max_size(), NonZeroUsize::new(200).unwrap());
        assert_eq!(resolver_cache.get_cache_additional().get_max_size(), NonZeroUsize::new(300).unwrap());
    }

    #[test]
    fn get_cache_answer(){
        let resolver_cache = ResolverCache::new(None);
        let cache = resolver_cache.get_cache_answer();
        assert_eq!(cache.get_max_size(), NonZeroUsize::new(1667).unwrap());
    }

    #[test]
    fn get_cache_authority(){
        let resolver_cache = ResolverCache::new(None);
        let cache = resolver_cache.get_cache_authority();
        assert_eq!(cache.get_max_size(), NonZeroUsize::new(1667).unwrap());
    }

    #[test]
    fn get_cache_additional(){
        let resolver_cache = ResolverCache::new(None);
        let cache = resolver_cache.get_cache_additional();
        assert_eq!(cache.get_max_size(), NonZeroUsize::new(1667).unwrap());
    }

    #[test]
    fn set_cache_answer(){
        let mut resolver_cache = ResolverCache::new(None);
        let cache = DnsCache::new(None);
        resolver_cache.set_cache_answer(cache.clone());
        assert_eq!(resolver_cache.get_cache_answer().get_max_size(), cache.get_max_size());
    }

    #[test]
    fn set_cache_authority(){
        let mut resolver_cache = ResolverCache::new(None);
        let cache = DnsCache::new(None);
        resolver_cache.set_cache_authority(cache.clone());
        assert_eq!(resolver_cache.get_cache_authority().get_max_size(), cache.get_max_size());
    }

    #[test]
    fn set_cache_additional(){
        let mut resolver_cache = ResolverCache::new(None);
        let cache = DnsCache::new(None);
        resolver_cache.set_cache_additional(cache.clone());
        assert_eq!(resolver_cache.get_cache_additional().get_max_size(), cache.get_max_size());
    }

    #[test]
    fn add_answer() {
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);

        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        resource_record.set_ttl(1000);

        resolver_cache.add_answer(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr = resolver_cache.cache_answer.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr[0].get_resource_record(), resource_record);
    }

    #[test]
    fn add_authority() {
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);

        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        resource_record.set_ttl(1000);

        resolver_cache.add_authority(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr = resolver_cache.cache_authority.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr[0].get_resource_record(), resource_record);
    }

    #[test]
    fn add_additional() {
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());
        let ip_address = IpAddr::from([127, 0, 0, 0]);
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);

        resource_record.set_name(domain_name.clone());
        resource_record.set_type_code(Rtype::A);
        resource_record.set_ttl(1000);

        resolver_cache.add_additional(domain_name.clone(), resource_record.clone(), Qtype::A, Qclass::IN, None);

        let rr = resolver_cache.cache_additional.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr[0].get_resource_record(), resource_record);
    }

    #[test]
    fn add(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        let mut message = DnsMessage::new();
        let mut header = message.get_header();
        header.set_rcode(Rcode::NOERROR);
        message.set_header(header);

        message.set_query_id(1);


        let mut question = Question::new();
        question.set_qname(domain_name.clone());
        question.set_qtype(Qtype::A);
        question.set_qclass(Qclass::IN);

        message.set_question(question);

        message.set_answer(vec![resource_record_1.clone()]);
        message.set_authority(vec![resource_record_2.clone()]);
        message.set_additional(vec![resource_record_3.clone()]);

        resolver_cache.add(message.clone());

        let rr_answer = resolver_cache.cache_answer.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();
        let rr_authority = resolver_cache.cache_authority.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();
        let rr_additional = resolver_cache.cache_additional.get(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr_answer[0].get_resource_record(), resource_record_1);
        assert_eq!(rr_authority[0].get_resource_record(), resource_record_2);
        assert_eq!(rr_additional[0].get_resource_record(), resource_record_3);
    }

    #[test]
    fn get_answer(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_answer(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_answer(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_answer(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        let rr = resolver_cache.get_answer(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr[0], resource_record_1);
        assert_eq!(rr[1], resource_record_2);
        assert_eq!(rr[2], resource_record_3);
    }

    #[test]
    fn get_authority(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_authority(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_authority(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_authority(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        let rr = resolver_cache.get_authority(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr[0], resource_record_1);
        assert_eq!(rr[1], resource_record_2);
        assert_eq!(rr[2], resource_record_3);
    }

    #[test]
    fn get_additional(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_additional(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_additional(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_additional(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        let rr = resolver_cache.get_additional(domain_name.clone(), Qtype::A, Qclass::IN).unwrap();

        assert_eq!(rr[0], resource_record_1);
        assert_eq!(rr[1], resource_record_2);
        assert_eq!(rr[2], resource_record_3);
    }

    #[test]
    fn get(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_answer(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_authority(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_additional(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        let qname = DomainName::new_from_string("www.example.com".to_string());
        let qtype = Qtype::A;
        let qclass = Qclass::IN;
        let op_code = 0;
        let rd = true;
        let id = 1;
        
        let query = DnsMessage::new_query_message(qname.clone(), qtype.clone(), qclass.clone(), op_code.clone(), rd.clone(), id.clone());

        let message = resolver_cache.get(query).unwrap();

        assert_eq!(message.get_answer()[0], resource_record_1);
        assert_eq!(message.get_authority()[0], resource_record_2);
        assert_eq!(message.get_additional()[0], resource_record_3);

        assert_eq!(message.get_header().get_rcode(), Rcode::NOERROR);
        assert_eq!(message.get_query_id(), 1);
        assert_eq!(message.get_question().get_qname(), qname);
    }

    #[test]
    fn remove_answer(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_answer(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_answer(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_answer(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        resolver_cache.remove_answer(domain_name.clone(), Qtype::A, Qclass::IN);

        let rr = resolver_cache.get_answer(domain_name.clone(), Qtype::A, Qclass::IN);

        assert_eq!(rr, None);
    }

    #[test]
    fn remove_authority(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_authority(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_authority(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_authority(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        resolver_cache.remove_authority(domain_name.clone(), Qtype::A, Qclass::IN);

        let rr = resolver_cache.get_authority(domain_name.clone(), Qtype::A, Qclass::IN);

        assert_eq!(rr, None);
    }

    #[test]
    fn remove_additional(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_additional(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_additional(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_additional(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);
        
        resolver_cache.remove_additional(domain_name.clone(), Qtype::A, Qclass::IN);

        let rr = resolver_cache.get_additional(domain_name.clone(), Qtype::A, Qclass::IN);

        assert_eq!(rr, None);
    }

    #[test]
    fn remove(){
        let mut resolver_cache = ResolverCache::new(None);

        let domain_name = DomainName::new_from_string("www.example.com".to_string());

        let ip_address_1 = IpAddr::from([127, 0, 0, 0]);
        let ip_address_2 = IpAddr::from([127, 0, 0, 1]);
        let ip_address_3 = IpAddr::from([127, 0, 0, 2]);

        let mut a_rdata_1 = ARdata::new();
        let mut a_rdata_2 = ARdata::new();
        let mut a_rdata_3 = ARdata::new();

        a_rdata_1.set_address(ip_address_1);
        a_rdata_2.set_address(ip_address_2);
        a_rdata_3.set_address(ip_address_3);

        let rdata_1 = Rdata::A(a_rdata_1);
        let rdata_2 = Rdata::A(a_rdata_2);
        let rdata_3 = Rdata::A(a_rdata_3);

        let mut resource_record_1 = ResourceRecord::new(rdata_1);

        resource_record_1.set_name(domain_name.clone());
        resource_record_1.set_type_code(Rtype::A);
        resource_record_1.set_ttl(1000);

        let mut resource_record_2 = ResourceRecord::new(rdata_2);

        resource_record_2.set_name(domain_name.clone());
        resource_record_2.set_type_code(Rtype::A);
        resource_record_2.set_ttl(1000);

        let mut resource_record_3 = ResourceRecord::new(rdata_3);

        resource_record_3.set_name(domain_name.clone());
        resource_record_3.set_type_code(Rtype::A);
        resource_record_3.set_ttl(1000);

        resolver_cache.add_answer(domain_name.clone(), resource_record_1.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_authority(domain_name.clone(), resource_record_2.clone(), Qtype::A, Qclass::IN, None);
        resolver_cache.add_additional(domain_name.clone(), resource_record_3.clone(), Qtype::A, Qclass::IN, None);

        let qname = DomainName::new_from_string("www.example.com".to_string());
        let qtype = Qtype::A;
        let qclass = Qclass::IN;
        let op_code = 0;
        let rd = true;
        let id = 1;
        
        let query = DnsMessage::new_query_message(qname.clone(), qtype.clone(), qclass.clone(), op_code.clone(), rd.clone(), id.clone());

        resolver_cache.remove(domain_name.clone(), Qtype::A, Qclass::IN);

        let message = resolver_cache.get(query);

        assert_eq!(message, None);
    }
}