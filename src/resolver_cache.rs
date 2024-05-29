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

        authorities.iter()
        .for_each(|rr| {
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
