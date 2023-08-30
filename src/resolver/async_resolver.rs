use std::io;
use std::net::{IpAddr, Ipv4Addr};

use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::resolver::{config::ResolverConfig,lookup::LookupIpFutureStub};
use crate::message::rdata::Rdata;

pub struct AsyncResolver{
    // config: ResolverConfig,  FIXME: ver si conviene para configurara tiposd e consultas que aceptara resolver
    cache: DnsCache,
    config: ResolverConfig,
// runtime:Mutex<Runtime> //FIXME: obliga correr fun async
}

impl AsyncResolver{

    pub fn new(config: ResolverConfig)-> Self{
        let async_resolver = AsyncResolver{
            cache: DnsCache::new(),
            config: config,
        };
        async_resolver
    } 
    
    pub async fn lookup_ip(&self, domain_name: &str) -> Result<IpAddr, io::Error> {
        println!("[LOOKUP IP ASYNCRESOLVER]");
        
        // TODO: verificaciones
        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());

        // TODO: Revisar cache

        //Async query
        let response = LookupIpFutureStub::lookup(domain_name_struct, self.cache.clone()).await;

        println!("[LOOKUP IP RESPONSE => {:?}]",response);
        let ip_addr = match response {
            Ok(val) => {
                let rdata = val.get_answer()[0].get_rdata();
    
                match rdata {
                    Rdata::SomeARdata(ip) => ip.get_address(), // Supongo que A es el tipo correcto
                    _ => IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                }
            }
            _ => panic!("[ERROR]"),
        };

        // TODO: Eliminar esto 
        Ok(ip_addr)   
    }

    #[allow(unused_variables)]
    pub async fn lookup(&self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        unimplemented!()
    }

}

#[cfg(test)]
mod async_resolver_test {
    use tokio::runtime::Runtime;
    use crate::resolver::config::ResolverConfig;
    use super::AsyncResolver;

    
    #[test]
    fn lookup_ip() {

        let resolver = AsyncResolver::new(ResolverConfig::default());
        
        let runtime = Runtime::new().unwrap();
        let response = runtime.block_on(resolver.lookup_ip("example.com"));

        // TODO: add assert test Ip example.com

        let response = runtime.block_on(resolver.lookup_ip("niclabs.cl"));

        // TODO: add assert test ip niclabs.cl

    }

    #[tokio::test]
    #[ignore]
    async fn lookupip_example() {

        let resolver = AsyncResolver::new(ResolverConfig::default());
      
        let response = resolver.lookup_ip("example.com").await.unwrap();

        println!("[TEST => {}]",response);
    }

}