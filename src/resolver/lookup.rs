use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::type_rtype::Rtype;
use crate::message::resource_record::ResourceRecord;
use crate::client::client_connection::ClientConnection;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use futures_util::{FutureExt,task::Waker};
use std::pin::Pin;
use std::task::{Poll,Context};
use rand::{thread_rng, Rng};
//TODO: Eliminar librerias
use futures_util::{future::Future,future};
use super::resolver_error::ResolverError;
use std::sync:: {Mutex,Arc};
use crate::client::client_connection::ConnectionProtocol;
use crate::resolver::config::ResolverConfig;
use crate::client::udp_connection::ClientUDPConnection;
use crate::client::tcp_connection::ClientTCPConnection;

/// Future returned fron AsyncResolver when performing a lookup with rtype A
pub struct LookupIpFutureStub  {
    name: DomainName,
    config: ResolverConfig,
    query_answer: Arc<std::sync::Mutex<Pin<Box<dyn futures_util::Future<Output = Result<DnsMessage, ResolverError>> + Send>>>>,
    cache: DnsCache,
    waker: Option<Waker>,
}

impl Future for LookupIpFutureStub{ 
    type Output = Result<DnsMessage, ResolverError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[POLL FUTURE]");

        let query = self.query_answer.lock().unwrap().as_mut().poll(cx)  ;

        match query {
            Poll::Pending => {
                println!("  [return pending]");
                return Poll::Pending;
            },
            Poll::Ready(Err(_)) => {
                println!("  [ready empty]");
                self.waker = Some(cx.waker().clone());
                
                let referenced_query = Arc::clone(&self.query_answer); //same as self.query.clone()
                tokio::spawn(
                    lookup_stub(
                        self.name.clone(),
                        self.cache.clone(),
                        self.config.get_name_servers(),
                        self.waker.clone(),
                        referenced_query,
                        self.config.clone()));
                println!("  [return pending]");
                return Poll::Pending;
            },
            Poll::Ready(Ok(ip_addr)) => {
                println!("  [return ready]");
                return Poll::Ready(Ok(ip_addr));
            }
        }
    }

}
    
impl LookupIpFutureStub {
    pub fn lookup(
        name: DomainName,
        config: ResolverConfig,
        cache: DnsCache
    ) -> Self {
        println!("[LOOKUP CREATE FUTURE]");
        
        Self { 
            name: name,
            config: config,
            query_answer:  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed())),  //FIXME: cambiar a otro tipo el error/inicio
            cache: cache,
            waker: None,
        }

    }

}
pub async fn  lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
    name: DomainName,
    mut cache: DnsCache,
    name_servers: Vec<(ClientUDPConnection, ClientTCPConnection)>,
    waker: Option<Waker>,
    referenced_query:Arc<std::sync::Mutex<Pin<Box<dyn futures_util::Future<Output = Result<DnsMessage, ResolverError>> + Send>>>>,
    config: ResolverConfig,
) {
    println!("[LOOKUP STUB]");

    // Create random generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query
    let mut new_query = DnsMessage::new_query_message(
        name.clone(),
        Qtype::A,
        Qclass::IN,
        0,
        false,
        query_id
    );

    if let Some(cache_lookup) = cache.get(name.clone(), Rtype::A) {
        println!("[LOOKUP STUB] cached data {:?}",cache_lookup);

        //Add Answer
        let answer: Vec<ResourceRecord> = cache_lookup
                                            .iter()
                                            .map(|rr_cache_value| rr_cache_value.get_resource_record())
                                            .collect::<Vec<ResourceRecord>>();
        new_query.set_answer(answer);
        // return Ok(new_query);
    }

    // Create Server failure query 
    let mut response = new_query.clone().to_owned();
    response.get_header().set_rcode(2); 

    let mut retry_count = 0;

    for (conn_udp,conn_tcp) in name_servers.iter() { 
        println!("[LOOKUP STUB] retry {}",retry_count);
        
        if retry_count > config.get_retry() {
            println!("[LOOKUP STUB] max tries");
            break;
        }
        
        match config.get_protocol() { 
            ConnectionProtocol::UDP=> {
                let result_response = conn_udp.send(new_query.clone());
                
                match result_response {
                    Ok(response_ok) => {
                        response = response_ok;
                        break;
                    }
                    Err(_) => {
                        // TODO: when UDP do not works use TCP
                    }
                }
            }
            ConnectionProtocol::TCP => {
                let result_response = conn_tcp.send(new_query.clone());
                
                match result_response {
                    Ok(response_ok) => {
                        response = response_ok;
                        break;
                    }
                    Err(_) => ()
                }
            }
            _ => continue,
        }  

        retry_count = retry_count + 1;
    }

    // Wake up task
    if let Some(waker) = waker {
        println!("  [wake up task]");
        waker.wake();
    }
    let mut future_query = referenced_query.lock().unwrap();
    *future_query = future::ready(Ok(response.clone())).boxed();
    println!("[save answer]")
   
}


#[cfg(test)]
mod async_resolver_test {
    use crate::dns_cache::cache_data;
    // use tokio::runtime::Runtime;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::{ domain_name::DomainName, dns_cache::DnsCache};
    use super::lookup_stub;
    use tokio::time::Duration;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use super::*;

    #[test]
    fn lookup(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let domain_name_cache = DomainName::new_from_string("test.com".to_string());
        let config: ResolverConfig = ResolverConfig::default();
        
        let mut cache: DnsCache = DnsCache::new();
        cache.set_max_size(20);

        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        cache.add(domain_name_cache, resource_record);

        let lookup_future = LookupIpFutureStub::lookup(
            domain_name,
            config,
            cache
        );

        assert_eq!(lookup_future.name, DomainName::new_from_string("example.com".to_string()));
        assert_eq!(lookup_future.config.get_addr(),SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333));
        assert_eq!(lookup_future.cache.get_max_size(), 20);
        assert_eq!(lookup_future.cache.get_size(), 1);

    }

    // TODO: test poll (not shure)

    // TODO: lookup_stub test

    // TODO: lookup_stub test save query in lookup struct

    // TODO: lookup_stub numer of retries

    #[ignore]
    #[tokio::test]
    async fn lookup_stub_max_tries(){

        let domain_name = DomainName::new_from_string("example.com".to_string());

        let mut config: ResolverConfig = ResolverConfig::default();
        config.set_retry(1);

        let cache = DnsCache::new();
        let waker = None;
    
        let query =  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed()));

        // Create vector of name servers
        let non_existente_server:IpAddr = IpAddr::V4(Ipv4Addr::new(234,1 ,4, 44));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(non_existente_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(non_existente_server, timeout);

        let name_servers = vec![(conn_udp,conn_tcp)];

        lookup_stub(domain_name, cache, name_servers, waker,query,config).await;

        //FIXME: is not working


    }

     
    #[tokio::test]
    async fn lookup_stub_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let cache = DnsCache::new();
        let waker = None;
    
        let query =  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed()));

        // Create vect of name servers
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        
        let name_servers = vec![(conn_udp,conn_tcp)];
        lookup_stub(domain_name, cache, name_servers, waker,query,config).await;

    }   
}