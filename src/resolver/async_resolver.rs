//use std::io;
use std::net::{IpAddr, Ipv4Addr};

use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
//use crate::message::class_qclass::Qclass;
//use crate::message::type_qtype::Qtype;
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
    pub async fn lookup_ip(&self, domain_name: &str, transport_protocol: &str) -> Result<IpAddr, ResolverError> {
        println!("[LOOKUP IP ASYNCRESOLVER]");

        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());

        let transport_protocol_struct = ConnectionProtocol::from_str_to_connection_type(transport_protocol);

        let result = self.inner_lookup_ip(domain_name_struct, transport_protocol_struct).await;
        result
           
    }

    async fn inner_lookup_ip(&self, domain_name: DomainName, transport_protocol: ConnectionProtocol) -> Result<IpAddr, ResolverError> {

        // Get connection type
        let name_servers= self.config.get_name_servers();

        //Async query
        let response = LookupIpFutureStub::lookup(domain_name, self.cache.clone(),name_servers, transport_protocol).await;
        
        println!("[LOOKUP IP RESPONSE => {:?}]",response);
        let ip_addr = match response {
            Ok(val) => {
                let rdata = val.get_answer()[0].get_rdata(); //FIXME:
    
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

    /// RFC 1034
    /// 5.2. Client-resolver interface
    /// 
    /// Host address to host name translation
    pub async fn reverse_query() {
        unimplemented!()
    }

    /// RFC 1034
    /// Client-resolver interface
    /// 
    /// General lookup function
    pub async fn lookup() {
        unimplemented!()
    }




}

#[cfg(test)]
mod async_resolver_test {
    use crate::client::config::TIMEOUT;
    use crate::resolver::config::ResolverConfig;
    use crate::resolver::resolver_error::ResolverError;
    use super::AsyncResolver;
    

    
    #[tokio::test]
     async fn lookup_ip() {

         let resolver = AsyncResolver::new(ResolverConfig::default());
        
         //let runtime = Runtime::new().unwrap();
         let response = resolver.lookup_ip("example.com", "UDP");

         println!("[TEST FINISH=> {}]",response.await.unwrap());
         // TODO: add assert test Ip example.com

         //let response = runtime.block_on(resolver.lookup_ip("niclabs.cl","TCP"));

         // TODO: add assert test ip niclabs.cl

     }

     #[tokio::test]
    async fn lookupip_example() {
        println!("[TEST INIT]");

        let resolver = AsyncResolver::new(ResolverConfig::default());
      
        let response = resolver.lookup_ip("example.com", "UDP").await.unwrap();

        println!("[TEST FINISH=> {}]",response);
   
    }

    #[tokio::test]
    async fn host_name_to_host_address_translation() {
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        let ip_address = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        
        assert!(ip_address.is_ipv4());
    
        assert!(!ip_address.is_unspecified());
    }

    // async fn reverse_query() {
    //     let resolver = AsyncResolver::new(ResolverConfig::default());
    //     let ip_address = "192.168.0.1"; 
    //     let domain_name = resolver.reverse_query(ip_address).await;
    
    //     // Realiza aserciones para verificar que domain_name contiene un nombre de dominio válido.
    //     assert!(!domain_name.is_empty(), "El nombre de dominio no debe estar vacío");
    
    //     // Debe verificar que devuelve el nombre de dominio correspondiente a la dirección IP dada.
    //     // Dependiendo de tu implementación, puedes comparar el resultado con un valor esperado.
    //     // Por ejemplo, si esperas que la dirección IP "192.168.0.1" se traduzca a "ejemplo.com":
    //     assert_eq!(domain_name, "ejemplo.com", "El nombre de dominio debe ser 'ejemplo.com'");
    // }
    
    #[tokio::test]
    async fn timeout() {
        // Crea una instancia de tu resolutor con la configuración adecuada
        let resolver = AsyncResolver::new(ResolverConfig::default());
    
        // Intenta resolver un nombre de dominio que no existe o no está accesible
        let domain_name = "nonexistent-example.com";
        let transport_protocol = "UDP";
    
        // Configura un timeout corto para la resolución (ajusta según tus necesidades)
        let timeout_duration = std::time::Duration::from_secs(2);
        
        let result = tokio::time::timeout(timeout_duration, async {
            resolver.lookup_ip(domain_name, transport_protocol).await
        }).await;
        

        
        // Verifica que el resultado sea un error de timeout
        match result {
            Ok(Ok(_)) => {
                panic!("Se esperaba un error de timeout, pero se resolvió exitosamente");
            }
            Ok(Err(err)) => {
               assert!(true);
            }
            Err(_) => {
                panic!("El timeout no se manejó correctamente");
            }
        }
    }


}