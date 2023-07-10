use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::type_rtype::Rtype;
use crate::message::DnsMessage;


//Lookup seria el nuevo ResolverQuery
pub struct LookupFuture{
    cache: DnsCache,
    names: Vec<DomainName>,
    rtype: Rtype,
    query: DnsMessage,
}

impl LookupFuture{
    pub fn lookup(){
        unimplemented!();
    }
}