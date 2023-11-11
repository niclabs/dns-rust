use std::net::IpAddr;

use crate::client::client_error::ClientError;
use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::resource_record::ResourceRecord;
use crate::resolver::{config::ResolverConfig,lookup::LookupFutureStub};
use crate::message::rdata::Rdata;
use crate::client::client_connection::ConnectionProtocol;
use crate::resolver::resolver_error::ResolverError;
use crate:: message::type_qtype::Qtype;
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
    /// FIXME: DEBE RETORNAR CLIENT ERROR
    /// This method acts as an interface between the Client and the Resolver.
    /// It calls `inner_lookup(&self, domain_name: DomainName)` which will
    /// execute a look up of the given domain name asynchronously.
    pub async fn lookup_ip(&mut self, domain_name: &str, transport_protocol: &str) -> Result<Vec<IpAddr>, ClientError> {
        println!("[LOOKUP IP ASYNCRESOLVER]");

        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());

        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self.inner_lookup(domain_name_struct,Qtype::A).await;

        let result_rrs = self.parse_dns_msg(response);
        if let Ok(rrs) = result_rrs {
            let rrs_iter = rrs.into_iter();
            let ip_addresses: Result<Vec<IpAddr>, _> = rrs_iter.map(|rr| 
                {AsyncResolver::from_rr_to_ip(rr)}).collect();
            return ip_addresses;
        } else {
            Err(ClientError::TemporaryError("Error parsing response."))?
        }
    }

    // TODO: move and change as from method  of rr
    fn from_rr_to_ip(rr: ResourceRecord) -> Result<IpAddr, ClientError> {
        let rdata = rr.get_rdata();
        if let Rdata::SomeARdata(ip) = rdata {
            return Ok(ip.get_address());
        } else {
            Err(ClientError::TemporaryError("Response does not match type A."))?
        }
    }
 
    /// Parses the received `DnsMessage` and returns the corresponding RRs.
    /// 
    /// After receiving the response of the query, this method parses the DNS message
    /// of type `DnsMessage` to a `Vec<ResourceRecord>` with the corresponding resource
    /// records contained in the message. It will return the RRs if the response was
    /// successful. If the response was not successful, it will return the corresponding
    /// error message to the Client.
    fn parse_dns_msg(&self, response: Result<DnsMessage, ResolverError>) -> Result<Vec<ResourceRecord>, ClientError> {
        let dns_mgs = match response {
            Ok(val) => val,
            Err(_) => Err(ClientError::TemporaryError("no DNS message found"))?,
        };

        let header = dns_mgs.get_header();
        let rcode = header.get_rcode();
        if rcode == 0 {
            let answer = dns_mgs.get_answer();
            return Ok(answer);
        } 
        match rcode {
            1 => Err(ClientError::FormatError("The name server was unable to interpret the query."))?,
            2 => Err(ClientError::ServerFailure("The name server was unable to process this query due to a problem with the name server."))?,
            3 => Err(ClientError::NameError("The domain name referenced in the query does not exist."))?,
            4 => Err(ClientError::NotImplemented("The name server does not support the requested kind of query."))?,
            5 => Err(ClientError::Refused("The name server refuses to perform the specified operation for policy reasons."))?,
            _ => Err(ClientError::ResponseError(rcode))?,
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
    async fn inner_lookup(&self, domain_name: DomainName,qtype:Qtype) -> Result<DnsMessage, ResolverError> {

        // Async query
        let response = LookupFutureStub::lookup(
            domain_name,
            qtype,
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
    pub async fn lookup(&mut self, domain_name: &str, transport_protocol: &str, qtype:&str ) -> Result<Vec<ResourceRecord>, ResolverError>{
        println!("[LOOKUP ASYNCRESOLVER]");

        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());
        let qtype_struct = Qtype::from_str_to_qtype(qtype);
        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self.inner_lookup(domain_name_struct,qtype_struct).await;
        
        //TODO: parse header and personalised error type FIXME: SHOULD look all types
        return self.parse_dns_msg(response).map_err(Into::into)
        // match response {
        //     Ok(val) => {
        //         let rdata = val.get_answer()[0].get_rdata();
        //         Ok(rdata)      
        //     }
        //     Err(_) => Err(ResolverError::Message("Error Response"))?,
        // }
    }

}

//TODO: FK test config and documentation

#[cfg(test)]
mod async_resolver_test {
    use crate::client::client_error::ClientError;
    use crate::client::config::TIMEOUT;
    use crate::message::DnsMessage;
    use crate::message::class_qclass::Qclass;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use crate:: message::type_qtype::Qtype;
    use crate::resolver::config::ResolverConfig;
    use super::AsyncResolver;
    use std::net::IpAddr;
    use std::time::Duration;
    use crate::domain_name::DomainName;
    
    #[test]
    fn create_async_resolver() {
        let config = ResolverConfig::default();
        let resolver = AsyncResolver::new(config.clone());
        assert_eq!(resolver.config, config);
        assert_eq!(resolver.config.get_timeout(), Duration::from_secs(TIMEOUT));
    }

    #[tokio::test]
    async fn inner_lookup() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let qtype = Qtype::A;
        let response = resolver.inner_lookup(domain_name,qtype).await;

        //FIXME: add assert
        assert!(response.is_ok());
    } 

    #[tokio::test]
    async fn inner_lookup_ns() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let qtype = Qtype::NS;
        let response = resolver.inner_lookup(domain_name,qtype).await;
        assert!(response.is_ok());

        //FIXME: add assert
        println!("Response: {:?}",response);
    }

    #[tokio::test]
    async fn host_name_to_host_address_translation() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "TCP";
        let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);
        
        assert!(ip_addresses[0].is_ipv4());
    
        assert!(!ip_addresses[0].is_unspecified());
    }

    #[ignore]
    #[tokio::test]
    async fn lookup_ns() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        let qtype = "NS";
        let response = resolver.lookup(domain_name, transport_protocol,qtype).await.unwrap();
        
        println!("RESPONSE : {:?}",response);
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

    #[test]
    fn parse_dns_msg_ip() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::SomeARdata(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::SomeARdata(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }


    //TODO: test max number of retry
    #[tokio::test]
    async fn max_number_of_retry() {
        let mut config = ResolverConfig::default();
        let max_retries = 6;
        config.set_retry(max_retries);
        let mut resolver = AsyncResolver::new(config);

        // Realiza una resolución de DNS que sabes que fallará
        //let result = resolver.lookup_ip("nonexistent-domain.com", "UDP").await;

        let mut retries_attempted = 0;

        // Realiza la resolución de DNS que sabes que fallará
        while retries_attempted < max_retries {
            let result = resolver.lookup_ip("nonexistent-domain.com", "UDP").await;
             retries_attempted += 1;

            if result.is_ok() {
                break; // La resolución tuvo éxito, sal del bucle
            }
        }
        if retries_attempted == max_retries {
            assert!(retries_attempted == max_retries, "Número incorrecto de reintentos");
        } else {
            panic!("La resolución DNS tuvo éxito antes de lo esperado");
        }
        
    }

 
    #[tokio::test]
    async fn use_udp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);
        
        assert!(ip_addresses[0].is_ipv4());
    
        assert!(!ip_addresses[0].is_unspecified());
    }
  
    #[tokio::test]
    async fn use_tcp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "TCP";
        let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);
        
        assert!(ip_addresses[0].is_ipv4());
    
        assert!(!ip_addresses[0].is_unspecified());
    }

    //TODO: use UDP but fails and use TCP
    #[tokio::test]
    async fn use_udp_or_tcp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "nonexistent-domain.com";

        let udp_result = resolver.lookup_ip(domain_name, "UDP").await;
    
        let tcp_result = resolver.lookup_ip(domain_name, "TCP").await;
    
       
        println!("RESPONSE (UDP): {:?}", udp_result);
        println!("RESPONSE (TCP): {:?}", tcp_result);
    
        // Verifica que ambas operaciones hayan fallado
        
    }


    //TODO: diferent types of errors

    //TODO: bad domain name written

    //TODO: prbar diferentes qtype


}