pub mod async_resolver;
pub mod config;
pub mod lookup;
pub mod slist;

use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::{message::DnsMessage, domain_name::DomainName};
use crate::resolver::async_resolver::AsyncResolver;
use crate::resolver::config::ResolverConfig;

use tokio::net::{TcpListener,UdpSocket};



use std::error::Error;
pub struct Resolver {
    config: ResolverConfig,
}

impl Resolver {
    pub fn new(config: ResolverConfig) -> Self {

        let resolver = Resolver {
            config: config,
        };

        resolver
    }

    pub async fn run(&self)  {
        unimplemented!();
    }

    pub fn lookup(&self, dns_query:DnsMessage){
        unimplemented!();
    }

}

// Getters
impl Resolver {
    
    fn get_config(&self) -> &ResolverConfig {
        &self.config
    }
}

pub struct StubResolver {
    async_resolver: AsyncResolver
}

impl StubResolver {
    
    pub fn new(config: ResolverConfig) -> Self {

        let async_resolver = AsyncResolver::new(config);

        let stub_resolver = StubResolver {
            async_resolver 
        };

        stub_resolver
    }


    pub fn lookup_ip(&self, domain_name: &str) { // TODO: Cambiar a trait de nombre
        self.async_resolver.lookup_ip(domain_name);
    }

    pub fn lookup(&self, domain_name: DomainName, qtype:Qtype, qclass:Qclass) {
        unimplemented!()
    }
}


#[cfg(test)]
mod resolver_test {
    use super::*;

    #[tokio::test]
    async fn example() {
        let conf_default = ResolverConfig::default();
        let resolver = Resolver::new(conf_default);

        resolver.run().await; 

        //Correr en otra consola 
        //dig @127.0.0.1 -p 5333 uchile.cl +tcp
        //dig @127.0.0.1 -p 5333 uchile.cl 
    }
}


#[cfg(test)]
mod stub_resolver_test {

    use super::{StubResolver, config::ResolverConfig};


    #[test]
    fn lookup_ip() {
        let resolver = StubResolver::new(ResolverConfig::default());

        let response = resolver.lookup_ip("example.com");
         
        // TODO: Add test
    }

    

}