use crate::client::config;
use crate::dns_cache::DnsCache;
use crate::domain_name::{DomainName, self};
use crate::message::DnsMessage;
use crate::message::type_rtype::Rtype;
use crate::resolver::config::{ResolverConfig};

pub struct AsyncResolver{
    // config: ResolverConfig,  FIXME: ver si conviene para configurara tiposd e consultas que aceptara resolver
    cache: DnsCache,
    use_cache: bool,
    recursive_available: bool,
    // runtime:Mutex<Runtime> //FIXME: obliga correr fun async

}

impl AsyncResolver{

    pub fn new(config:&ResolverConfig)-> Self{
        let async_resolver = AsyncResolver{
            cache: DnsCache::new(),
            use_cache:config.get_recursive_available(),
            recursive_available:config.get_recursive_available(),
        };
        async_resolver
    } 

    pub fn echo(&self){
        println!("ECHO SERVER");
    }
    

    pub async fn inner_lookup(&self, dns_query:DnsMessage) {
        //TODO:logica del resolver
        println!("[INNER LOOKUP]");
    }

    ///Crea una lista con los nombres a consultar, se crea a partir del nombre de domiinio
    pub fn build_names(&self,_full_name: DomainName){
        unimplemented!();
    }

}