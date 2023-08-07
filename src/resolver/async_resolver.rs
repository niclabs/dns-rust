use std::io;
use std::net::IpAddr;

use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::resolver::config::ResolverConfig;

use rand::{thread_rng, Rng};


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

    pub async fn lookup_ip(&self, domain_name: DomainName) -> Result<DnsMessage, io::Error> {
        
        // Create random generator
        let mut rng = thread_rng();
        // Create query id
        let query_id: u16 = rng.gen();
        // Create query msg
        let msg: DnsMessage = DnsMessage::new_query_message(
            domain_name,
            Qtype::A, 
            Qclass::IN,
            0,
            false,
            query_id,
        );
        
        Ok(msg)
    }

    pub async fn lookup(&self, domain_name: DomainName, qtype: Qtype, qclass: Qclass) {
        unimplemented!()
    }

}