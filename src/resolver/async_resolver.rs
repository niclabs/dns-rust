use std::net::IpAddr;

use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::resolver::{config::ResolverConfig,lookup::LookupFutureStub};
use crate::message::rdata::Rdata;
use crate::client::client_connection::ConnectionProtocol;
use crate::resolver::resolver_error::ResolverError;

/// Asynchronous resolver for DNS queries.
/// 
/// This struct contains a cache and a configuration for the resolver. 
/// The cache is used to store the responses of the queries and the
/// configuration is used to set the parameters of the resolver.
/// 
/// The `AsyncResolver` struct is used to send queries to a DNS server in 
/// a asynchronous way. This means that the queries are sent and the
/// resolver continues with the execution of the program without waiting
/// for the response of the query.
/// 
/// Each query corresponds to a future that is going to be spawned using
/// `lookup_ip` method. 
pub struct AsyncResolver{
    /// Cache for the resolver.
    cache: DnsCache,
    /// Configuration for the resolver.
    config: ResolverConfig ,
}

impl AsyncResolver {

    /// Creates a new `AsyncResolver` with the given configuration.
    /// 
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// use dns_resolver::resolver::async_resolver::AsyncResolver;
    /// 
    /// let config = ReolverConfig::default();
    /// let resolver = AsyncResolver::new(config.clone());
    /// assert_eq!(resolver.config, config);
    /// ```
    pub fn new(config: ResolverConfig)-> Self {
        let async_resolver = AsyncResolver {
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

        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self.inner_lookup(domain_name_struct).await;
        
        //TODO: parse header and personalised error type 
        match response {
            Ok(val) => {
                let rdata = val.get_answer()[0].get_rdata();
                
                match rdata {
                    Rdata::SomeARdata(ip) => Ok(ip.get_address()), // Supongo que A es el tipo correcto
                    _ => Err(ResolverError::Message("Error Response"))?,
                }
            }
            Err(_) => Err(ResolverError::Message("Error Response"))?,
        }
    }

    /// Host name to address translation.
    /// 
    /// Performs a DNS lookup for the given domain name and returns the 
    /// corresponding IP address. This lookup is done asynchronously using
    /// the future `LookupIpFutureStub`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// use dns_resolver::resolver::async_resolver::AsyncResolver;
    /// 
    /// let resolver = AsyncResolver::new(ResolverConfig::default());
    /// let domain_name = DomainName::new_from_string("example.com".to_string());
    /// let response = resolver.inner_lookup(domain_name).await;
    /// assert!(response.is_ok());
    /// ```
    async fn inner_lookup(&self, domain_name: DomainName) -> Result<DnsMessage, ResolverError> {

        // Async query
        let response = LookupFutureStub::lookup(
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
    use crate::client::config::TIMEOUT;
    use crate::domain_name::DomainName;
    use crate::resolver::config::ResolverConfig;
    use super::AsyncResolver;
    use std::time::Duration;
    
    #[test]
    fn create_async_resolver() {
        let config = ResolverConfig::default();
        let resolver = AsyncResolver::new(config.clone());
        assert_eq!(resolver.config, config);
        assert_eq!(resolver.config.get_timeout(), Duration::from_secs(TIMEOUT));
    }

    //TODO: test inner_lookup
    #[tokio::test]
    async fn inner_lookup() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let response = resolver.inner_lookup(domain_name).await;
        assert!(response.is_ok());
    }

    #[ignore]
    #[tokio::test] //TODO
    async fn lookup_ip() {

        let mut resolver = AsyncResolver::new(ResolverConfig::default());
    
        //let runtime = Runtime::new().unwrap();
        let response = resolver.lookup_ip("example.com", "UDP");

        println!("[TEST FINISH=> {}]",response.await.unwrap());
        // TODO: add assert test Ip example.com

        //let response = runtime.block_on(resolver.lookup_ip("niclabs.cl","TCP"));

        // TODO: add assert test ip niclabs.cl

    }

    #[ignore]
    #[tokio::test]  //TODO
    async fn lookupip_example() {
        println!("[TEST INIT]");

        let mut resolver = AsyncResolver::new(ResolverConfig::default());
      
        let response = resolver.lookup_ip("example.com", "UDP").await.unwrap();

        println!("[TEST FINISH=> {}]",response);
   
    }

    #[ignore]
    #[tokio::test]
    async fn host_name_to_host_address_translation() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
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
    
    #[ignore]
    #[tokio::test]
    async fn timeout() {
        // Crea una instancia de tu resolutor con la configuración adecuada
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
    
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
            Ok(Err(_err)) => {
               assert!(true);
            }
            Err(_) => {
                panic!("El timeout no se manejó correctamente");
            }
        }
    }


    //TODO: test max number of retry

    //TODO: use UDP

    //TODO: use TCP

    //TODO: use UDP but fails and use TCP

    //TODO: diferent types of errors

    //TODO: bad domain name written
}