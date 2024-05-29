use crate::dns_cache::DnsCache;

use std::num::NonZeroUsize;

#[derive(Clone, Debug)]
pub struct ResolverCache {
    cache_answer: DnsCache,
    cache_authority: DnsCache,
    cache_additional: DnsCache,
}

impl ResolverCache {
    pub fn new(size: Option<NonZeroUsize>) -> Self {
        let size = size.unwrap_or(NonZeroUsize::new(1667).unwrap());
        Self {
            cache_answer: DnsCache::new(Some(size)),
            cache_authority: DnsCache::new(Some(size)),
            cache_additional: DnsCache::new(Some(size)),
        }
    }
}
