use std::net::IpAddr;
use std::vec;

use crate::client::client_error::ClientError;
use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::class_qclass::Qclass;
use crate::message::resource_record::ResourceRecord;
use crate::async_resolver::{config::ResolverConfig,lookup::LookupFutureStub};
use crate::message::rdata::Rdata;
use crate::message::type_rtype::Rtype;
use crate::client::client_connection::ConnectionProtocol;
use crate::async_resolver::resolver_error::ResolverError;
use crate:: message::type_qtype::Qtype;

#[cfg(test)]

mod async_resolver_test {
    use tokio::io;

    use crate::client::client_error::ClientError;
    use crate::message::DnsMessage;
    use crate::message::class_qclass::Qclass;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::resource_record::ResourceRecord;
    use crate:: message::type_qtype::Qtype;
    use crate::message::type_rtype::Rtype;
    use crate::async_resolver::config::ResolverConfig;
    use super::AsyncResolver;
    use std::net::IpAddr;
    use std::str::FromStr;
    use std::time::Duration;
    use crate::domain_name::DomainName;
    
    #[test]
    fn create_async_resolver() {
        let config = ResolverConfig::default();
        let resolver = AsyncResolver::new(config.clone());
        assert_eq!(resolver.config, config);
        assert_eq!(resolver.config.get_timeout(), Duration::from_secs(TIMEOUT));
    }

}