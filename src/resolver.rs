pub mod async_resolver;
pub mod config;
pub mod lookup;
pub mod slist;

use crate::domain_name::DomainName;
use crate::message::{DnsMessage};
use crate::resolver::async_resolver::AsyncResolver;
use crate::resolver::config::ResolverConfig;
use crate::message::type_rtype::Rtype;

use tokio::runtime::{self,Runtime};
pub struct Resolver{
    // runtime: Mutex<Runtime>,
    async_resolver: AsyncResolver,
}


impl Resolver {

    pub fn new(config: ResolverConfig)-> Self {

        let async_resolver = AsyncResolver::new();

        let resolver = Resolver{
            async_resolver: async_resolver,
        };

        unimplemented!();
    }


    pub fn lookup(&self,domain_name: &str, rtype:&str)-> Result<DnsMessage, ()> { // TODO: Agregar Error
        
        //change to structs
        let rtype_struct = Rtype::from_str_to_rtype(rtype);
        let mut domain_name_struct = DomainName::new();
        domain_name_struct.set_name(domain_name.to_string());

        //llamar al metodo lookup de async resolver
        let _lookup = self.async_resolver.inner_lookup(domain_name, rtype_struct);
        
        unimplemented!();
    }

}

#[cfg(test)]
mod resolver_test {

    use crate::resolver::{slist::Slist, config::ResolverConfig};

    use std::collections::HashMap;
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;

    #[test]
    fn example() {
        //create config
        let resolver_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));

        //sbelt creation
        // FIXME: ver qu ehace en caso de que si le quiera pasar sbelt
        // let mut sbelt = Slist::new();
        // let mut ns_list = Vec::new();
        // let servers = HashMap::new();
        // ns_list.push(servers);
        // sbelt.set_ns_list(ns_list);

        let mut config = ResolverConfig::new(None, resolver_addr);

        let resolver = Resolver::new(config);

        let response = resolver.lookup("example.com", "A").unwrap();

        // TODO: Test de los parametros de la respuesta, esperado para example.com
    }
}