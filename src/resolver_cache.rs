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
        self.cache_answer.add(domain_name, resource_record, qtype, qclass, rcode);
    }

    /// Add an element to the authority cache.
    pub fn add_authority(&mut self, domain_name: DomainName, resource_record: ResourceRecord, qtype: Qtype, qclass: Qclass, rcode: Option<Rcode>) {
        self.cache_authority.add(domain_name, resource_record, qtype, qclass, rcode);
    }

    /// Add an element to the additional cache.
    pub fn add_additional(&mut self, domain_name: DomainName, resource_record: ResourceRecord, qtype: Qtype, qclass: Qclass, rcode: Option<Rcode>) {
        self.cache_additional.add(domain_name, resource_record, qtype, qclass, rcode);
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
            if rr.get_ttl() > 0 {
                self.add_answer(qname.clone(), rr.clone(), qtype, qclass, rcode);
            }
        });

        authorities.iter()
        .for_each(|rr| {
            if rr.get_ttl() >0 {
                self.add_authority(qname.clone(), rr.clone(), qtype, qclass, rcode);
            }
        });

        additionals.iter()
        .for_each(|rr| {
            if rr.get_ttl() > 0 {
                if rr.get_rtype() != Rtype::OPT {
                    self.add_additional(qname.clone(), rr.clone(), qtype, qclass, rcode);
                }
            }
        });
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
