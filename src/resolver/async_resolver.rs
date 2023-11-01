use std::net::IpAddr;

use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::resolver::{config::ResolverConfig,lookup::LookupIpFutureStub};
use crate::message::rdata::Rdata;
use crate::client::client_connection::ConnectionProtocol;
use crate::resolver::resolver_error::ResolverError;

pub struct AsyncResolver{
    cache: DnsCache,
    config: ResolverConfig ,
}

impl AsyncResolver {

    pub fn new(config: ResolverConfig)-> Self{
        let async_resolver = AsyncResolver{
            cache: DnsCache::new(),
            config: config,
        };
        async_resolver
    } 

    /// RFC 1034
    /// 5.2. Client-resolver interface
    /// 
    /// Host name to host address translation
    /// 
    pub async fn lookup_ip(&mut self, domain_name: &str, transport_protocol: &str) -> Result<IpAddr, ResolverError> {
        println!("[LOOKUP IP ASYNCRESOLVER]");

        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());

        let transport_protocol_struct = ConnectionProtocol::from_str_to_connection_type(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self.inner_lookup(domain_name_struct).await;
        
        match response {
            Ok(val) => {
                let rdata = val.get_answer()[0].get_rdata();
    
                match rdata {
                    Rdata::SomeARdata(ip) => Ok(ip.get_address()), // Supongo que A es el tipo correcto
                    _ => Err(ResolverError::Message("Error Response")),
                }
            }
            Err(_) => Err(ResolverError::Message("Error Response")),
        }
    }

    async fn inner_lookup(&self, domain_name: DomainName) -> Result<DnsMessage, ResolverError> {

        // Async query
        let response = LookupIpFutureStub::lookup(
            domain_name,
            self.config.clone(),
            self.cache.clone())
            .await;

        response
    }

    /// RFC 1034
    /// 5.2. Client-resolver interface
    /// 
    /// Host address to host name translation
    /// 
    pub async fn reverse_query() {
        unimplemented!()
    }

    /// RFC 1034
    /// Client-resolver interface
    /// 
    /// General lookup function
    /// 
    pub async fn lookup() {
        unimplemented!()
    }

}

//TODO: FK test config and documentation

#[cfg(test)]
mod async_resolver_test {
    use crate::resolver::config::ResolverConfig;
    use super::AsyncResolver;
    
    #[tokio::test]
    async fn lookup_ip() {

        let mut resolver = AsyncResolver::new(ResolverConfig::default());
    
        //let runtime = Runtime::new().unwrap();
        let response = resolver.lookup_ip("example.com", "UDP");

        println!("[TEST FINISH=> {}]",response.await.unwrap());
        // TODO: add assert test Ip example.com

        //let response = runtime.block_on(resolver.lookup_ip("niclabs.cl","TCP"));

        // TODO: add assert test ip niclabs.cl

    }
}
