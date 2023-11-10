use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::header::Header;
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

/// Future returned from `AsyncResolver` when performing a lookup with Rtype A.
/// 
/// This implementation of `Future` is used to send a single query to a DNS server.
/// When this future is polled by `AsyncResolver`, 
pub struct LookupFutureStub {
    /// Domain Name associated with the query.
    name: DomainName,
    /// Qtype of search query
    record_type: Qtype,
    /// Resolver configuration.
    config: ResolverConfig,
    /// Future that contains the response of the query.
    /// 
    /// The `Output` of this future is a `Result<DnsMessage, ResolverError>`.
    /// The returned `DnsMessage` contains the corresponding response of the query.
    query_answer: Arc<std::sync::Mutex<Pin<Box<dyn futures_util::Future<Output = Result<DnsMessage, ResolverError>> + Send>>>>,
    /// Cache for the resolver.
    cache: DnsCache,
    /// Waker for the future.
    waker: Option<Waker>,
}

impl Future for LookupFutureStub { 
    type Output = Result<DnsMessage, ResolverError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let query = self.query_answer.lock().unwrap().as_mut().poll(cx)  ;

        match query {
            Poll::Pending => {
                return Poll::Pending;
            },
            Poll::Ready(Err(_)) => {
                self.waker = Some(cx.waker().clone());
                
                let referenced_query = Arc::clone(&self.query_answer); 
                tokio::spawn(
                    lookup_stub(
                        self.name.clone(),
                        self.record_type,
                        self.cache.clone(),
                        self.config.get_name_servers(),
                        self.waker.clone(),
                        referenced_query,
                        self.config.clone()));
                
                return Poll::Pending;
            },
            Poll::Ready(Ok(ip_addr)) => {
                return Poll::Ready(Ok(ip_addr));
            }
        }
    }
}
    
impl LookupFutureStub {

    /// Creates a new `LookupIpFutureStub` with the given configuration.
    /// 
    /// The resulting future created by default contains an empty `DnsMessage`
    /// which is going to be replaced by the response of the query after
    /// `LookupIpFutureStub` is polled.
    pub fn lookup(
        name: DomainName,
        qtype: Qtype,
        config: ResolverConfig,
        cache: DnsCache
    ) -> Self {
        
        Self { 
            name: name,
            record_type: qtype,
            config: config,
            query_answer:  
            Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed())),  //FIXME: cambiar a otro tipo el error/inicio
            cache: cache,
            waker: None,
        }
    }

}

/// Perfoms the lookup of a Domain Name acting as a Stub Resolver.
/// 
/// This function performs the lookup of the requested records asynchronously. 
/// The given `waker` is used to wake up the task when the query is answered. 
/// The `referenced_query` is used to update the future that contains the response of the query.
/// 
/// After creating the query with the given parameters, the function sends it to the name servers 
/// specified in the configuration. 
/// 
/// When a response is received, the function performs the parsing of the response to a `DnsMessage`.
/// After the response is checked, the function updates the future that contains the response of the query.
pub async fn  lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
    name: DomainName,
    record_type: Qtype,
    mut cache: DnsCache,
    name_servers: Vec<(ClientUDPConnection, ClientTCPConnection)>,
    waker: Option<Waker>,
    referenced_query:Arc<std::sync::Mutex<Pin<Box<dyn futures_util::Future<Output = Result<DnsMessage, ResolverError>> + Send>>>>,
    config: ResolverConfig,
) -> Result<DnsMessage,ResolverError>{

    // Create random generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query
    let mut new_query = DnsMessage::new_query_message(
        name.clone(),
        record_type,
        Qclass::IN,
        0,
        false,
        query_id
    );

    if let Some(cache_lookup) = cache.get(name.clone(), Rtype::A) {

        // Add Answer
        let answer: Vec<ResourceRecord> = cache_lookup
                                            .iter()
                                            .map(|rr_cache_value| rr_cache_value.get_resource_record())
                                            .collect::<Vec<ResourceRecord>>();
        new_query.set_answer(answer);

    }

    // Create Server failure query 
    let mut response = new_query.clone().to_owned();
    let mut new_header:Header = response.get_header();
    new_header.set_rcode(2);
    new_header.set_qr(true);
    response.set_header(new_header);

    let mut retry_count = 0;

    for (conn_udp,conn_tcp) in name_servers.iter() { 
        
        if retry_count > config.get_retry() {
            break;
        }
        
        match config.get_protocol() { 
            ConnectionProtocol::UDP=> {
                let result_response = conn_udp.send(new_query.clone());
                
                response = match result_response {
                    Ok(response_message) => {
                        match DnsMessage::from_bytes(&response_message) {
                            Ok(dns_message) => dns_message,
                            Err(_) => Err(ResolverError::Parse("The name server was unable to interpret the query.".to_string()))?,
                        }
                    },
                    Err(_) => response,
                }
            }
            ConnectionProtocol::TCP => {
                let result_response = conn_tcp.send(new_query.clone());
                
                response = match result_response {
                    Ok(response_message) => {
                        match DnsMessage::from_bytes(&response_message) {
                            Ok(dns_message) => dns_message,
                            Err(_) => Err(ResolverError::Parse("The name server was unable to interpret the query.".to_string()))?,
                        }
                    },
                    Err(_) => response,
                }
            }
            _ => continue,
        } 

        retry_count = retry_count + 1;
    }

    // Wake up task
    if let Some(waker) = waker {
        waker.wake();
    }
    let mut future_query = referenced_query.lock().unwrap();
    *future_query = future::ready(Ok(response.clone())).boxed();

    Ok(response)
   
}
#[cfg(test)]
mod async_resolver_test {
    // use tokio::runtime::Runtime;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::{ domain_name::DomainName, dns_cache::DnsCache};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::time::Duration;
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

        let record_type = Qtype::A;

        let lookup_future = LookupFutureStub::lookup(
            domain_name,
            record_type,
            config,
            cache
        );

        assert_eq!(lookup_future.name, DomainName::new_from_string("example.com".to_string()));
        assert_eq!(lookup_future.config.get_addr(),SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333));
        assert_eq!(lookup_future.cache.get_max_size(), 20);
        assert_eq!(lookup_future.cache.get_size(), 1);
        
    }
     
    #[tokio::test]
    async fn lookup_stub_a_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let cache = DnsCache::new();
        let waker = None;
        let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::A;
        
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response = lookup_stub(domain_name,record_type, cache, name_servers, waker,query,config).await.unwrap();

        assert_eq!(response.get_header().get_qr(),true);
        assert_ne!(response.get_answer().len(),0);
    }   

    #[tokio::test]
    async fn lookup_stub_ns_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let cache = DnsCache::new();
        let waker = None;
    
        let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));

        // Create vect of name servers
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::NS;
        
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response = lookup_stub(domain_name, record_type, cache, name_servers, waker,query,config).await.unwrap();

        assert_eq!(response.get_header().get_qr(),true);
        assert_ne!(response.get_answer().len(),0);

    } 

    #[tokio::test]
    async fn lookup_stub_max_tries(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let waker = None;
        let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;

        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 
        config.set_retry(1);
        let cache = DnsCache::new();
        
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
        
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response = lookup_stub(domain_name, record_type, cache, name_servers, waker,query,config).await.unwrap();
        

        
        println!("response_future {:?}",response);
  
        assert_eq!(response.get_header().get_ancount(), 0);
        assert_eq!(response.get_header().get_rcode() , 2);
        assert_eq!(response.get_header().get_qr(),true);
    }

    #[ignore]
    #[tokio::test]
    async fn poll_lookup_a(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;

        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 
        
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
        config.set_retry(3);
        let cache = DnsCache::new();

        let response_future = LookupFutureStub::lookup(domain_name, record_type ,config, cache).await;
        println!("response_future {:?}",response_future);

        assert_eq!(response_future.is_ok(), true);    
        let response = response_future.unwrap();
        // assert_eq!(response_future.unwrap().get_header().get_ancount(), 0);
        assert_eq!(response.get_header().get_rcode() , 2);
        // assert_eq!(response_future.unwrap().get_header().get_rcode() , 2);  //FIXME:
    }

    #[ignore]
    #[tokio::test]
    async fn poll_lookup_max_tries(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;

        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 
        
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
        config.set_retry(1);
        let cache = DnsCache::new();

        let response_future = LookupFutureStub::lookup(domain_name, record_type ,config, cache).await;
        println!("response_future {:?}",response_future);

        assert_eq!(response_future.is_ok(), true);    
        // assert_eq!(response_future.unwrap().get_header().get_ancount(), 0);
        assert_eq!(response_future.unwrap().get_header().get_rcode() , 2);
        // assert_eq!(response_future.unwrap().get_header().get_rcode() , 2);  //FIXME:
    }

    //TODO: add cache test 
    
}