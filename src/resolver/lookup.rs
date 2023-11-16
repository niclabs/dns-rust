use crate::client::client_error::ClientError;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::header::Header;
use crate::client::client_connection::ClientConnection;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use futures_util::{FutureExt,task::Waker};
use std::pin::Pin;
use std::task::{Poll,Context};
use rand::{thread_rng, Rng};
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
    ) -> Self {
        
        Self { 
            name: name,
            record_type: qtype,
            config: config,
            query_answer:  
            Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed())),  //FIXME: cambiar a otro tipo el error/inicio
            waker: None,
        }
    }

}

/// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.3.1
/// 5.3.1. Stub resolvers
/// 
/// One option for implementing a resolver is to move the resolution
/// function out of the local machine and into a name server which supports
/// recursive queries.  This can provide an easy method of providing domain
/// service in a PC which lacks the resources to perform the resolver
/// function, or can centralize the cache for a whole local network or
/// organization.
/// 
/// All that the remaining stub needs is a list of name server addresses
/// that will perform the recursive requests.  This type of resolver
/// presumably needs the information in a configuration file, since it
/// probably lacks the sophistication to locate it in the domain database.
/// The user also needs to verify that the listed servers will perform the
/// recursive service; a name server is free to refuse to perform recursive
/// services for any or all clients.  The user should consult the local
/// system administrator to find name servers willing to perform the
/// service.
///
/// This type of service suffers from some drawbacks.  Since the recursive
/// requests may take an arbitrary amount of time to perform, the stub may
/// have difficulty optimizing retransmission intervals to deal with both
/// lost UDP packets and dead servers; the name server can be easily
/// overloaded by too zealous a stub if it interprets retransmissions as new
/// requests.  Use of TCP may be an answer, but TCP may well place burdens
/// on the host's capabilities which are similar to those of a real
/// resolver.
/// 
/// Perfoms the lookup of a Domain Name acting as a Stub Resolver.
/// This function performs the lookup of the requested records asynchronously. 
/// The given `waker` is used to wake up the task when the query is answered. 
/// The `referenced_query` is used to update the future that contains the response of the query.
/// 
/// After creating the query with the given parameters, the function sends it to the name servers 
/// specified in the configuration. 
/// 
/// When a response is received, the function performs the parsing of the response to a `DnsMessage`.
/// After the response is checked, the function updates the future that contains the response of the query.
/// 
/// # Example
/// ```
/// let domain_name = DomainName::new_from_string("example.com".to_string());
/// let cache = DnsCache::new();
/// let waker = None;
/// let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));
///
/// let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
/// let timeout: Duration = Duration::from_secs(20);
///
/// let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
/// let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
/// 
/// let config = ResolverConfig::default();
/// let record_type = Qtype::A;
/// 
/// let name_servers = vec![(conn_udp,conn_tcp)];
/// let response = lookup_stub(domain_name,record_type, cache, name_servers, waker,query,config).await.unwrap();
/// ```
///
pub async fn lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
    name: DomainName,
    record_type: Qtype,
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
    let new_query = DnsMessage::new_query_message(
        name.clone(),
        record_type,
        Qclass::IN,
        0,
        false,
        query_id
    );

    // Create Server failure query 
    let mut response = new_query.clone().to_owned();
    let mut new_header: Header = response.get_header();
    new_header.set_rcode(2);
    new_header.set_qr(true);
    response.set_header(new_header);
    let mut result_dns_msg = Ok(response.clone());

    let mut retry_count = 0;

    for (conn_udp,conn_tcp) in name_servers.iter() { 
        
        if retry_count > config.get_retry() {
            break;
        }
        
        match config.get_protocol() { 
            ConnectionProtocol::UDP => {
                let result_response = conn_udp.send(new_query.clone());
                result_dns_msg = parse_response(result_response);
            }
            ConnectionProtocol::TCP => {
                let result_response = conn_tcp.send(new_query.clone());
                result_dns_msg = parse_response(result_response);
            }
            _ => continue,
        } 
        retry_count = retry_count + 1;
    }

    // Wake up task
    if let Some(waker) = waker {
        waker.wake();
    }

    let response_dns_msg = match result_dns_msg.clone() {
        Ok(response_message) => response_message,
        Err(_) => response,
    };
    let mut future_query = referenced_query.lock().unwrap();
    *future_query = future::ready(Ok(response_dns_msg)).boxed();

    result_dns_msg
}

/// Parse the received response datagram to a `DnsMessage`.
/// 
/// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-7.3
/// 
/// 7.3. Processing responses
/// The first step in processing arriving response datagrams is to parse the
/// response.  This procedure should include:
/// 
///    - Check the header for reasonableness.  Discard datagrams which
///      are queries when responses are expected.
/// 
///    - Parse the sections of the message, and insure that all RRs are
///      correctly formatted.
/// 
///    - As an optional step, check the TTLs of arriving data looking
///      for RRs with excessively long TTLs.  If a RR has an
///      excessively long TTL, say greater than 1 week, either discard
///      the whole response, or limit all TTLs in the response to 1
///      week.
fn parse_response(response_result: Result<Vec<u8>, ClientError>) -> Result<DnsMessage, ResolverError> {
    let dns_msg = response_result.map_err(Into::into)
        .and_then(|response_message| {
            DnsMessage::from_bytes(&response_message)
                .map_err(|_| ResolverError::Parse("The name server was unable to interpret the query.".to_string()))
        })?;
    let header = dns_msg.get_header();
    if header.get_qr() {
        return Ok(dns_msg);
    }
    Err(ResolverError::Parse("Message is a query. A response was expected.".to_string()))
}

#[cfg(test)]
mod async_resolver_test {
    // use tokio::runtime::Runtime;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::{ domain_name::DomainName, dns_cache::DnsCache};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;
    use std::time::Duration;
    use super::{*, parse_response};

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
        );

        assert_eq!(lookup_future.name, DomainName::new_from_string("example.com".to_string()));
        assert_eq!(lookup_future.config.get_addr(),SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333));
    }
     
    #[tokio::test]
    async fn lookup_stub_a_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let waker = None;
        let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::A;
        
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response = lookup_stub(domain_name,record_type, name_servers, waker,query,config).await.unwrap();

        assert_eq!(response.get_header().get_qr(),true);
        assert_ne!(response.get_answer().len(),0);
    }   

    #[tokio::test]
    async fn lookup_stub_ns_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
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
        let response = lookup_stub(domain_name, record_type, name_servers, waker,query,config).await.unwrap();

        assert_eq!(response.get_header().get_qr(),true);
        assert_ne!(response.get_answer().len(),0);

    } 

    #[tokio::test] 
    async fn lookup_stub_max_tries_0() {
       
        let max_retries =0;

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let waker = None;
        let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;
    
        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 
            
        config.set_retry(max_retries);
    
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
            
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response = lookup_stub(domain_name, record_type, name_servers, waker,query,config).await;
        println!("response {:?}",response);

        response.unwrap_err();
                
    }
           


    #[tokio::test] 
    async fn poll_lookup_a(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;

        let mut config: ResolverConfig = ResolverConfig::default();
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)); 
        
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
        config.set_retry(3);

        let response_future = LookupFutureStub::lookup(domain_name, record_type, config).await;
        println!("response_future {:?}",response_future);

        assert_eq!(response_future.is_ok(), true);    
        let response = response_future.unwrap();
        assert_eq!(response.get_header().get_ancount(), 1);
        assert_eq!(response.get_header().get_rcode() , 0);
    }

    #[tokio::test] 
    async fn poll_lookup_a_empty(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;

        let mut config: ResolverConfig = ResolverConfig::default();
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(38, 44, 1, 22)); 
        
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
        config.set_retry(3);

        let response_future = LookupFutureStub::lookup(domain_name, record_type ,config).await;
        println!("response_future {:?}",response_future);

        assert_eq!(response_future.is_ok(), true);    
        let response = response_future.unwrap();
        assert_eq!(response.get_header().get_ancount(), 0);
        assert_eq!(response.get_header().get_rcode() , 2);

        let answer = response.get_answer();
        assert!(answer.is_empty());
    }

    #[tokio::test] 
    async fn poll_lookup_max_tries_0(){

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;

        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 
        
        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);
        config.set_name_servers(vec![(conn_udp,conn_tcp)]);
        config.set_retry(0);

        let response_future = LookupFutureStub::lookup(domain_name, record_type ,config).await;
        println!("response_future {:?}",response_future);

        assert_eq!(response_future.is_ok(), false);    
       


    }

    #[tokio::test]
    async fn lookup_ip_cache_test() {

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let record_type = Qtype::A;
        
        let config: ResolverConfig = ResolverConfig::default();
        
        let addr = IpAddr::from_str("93.184.216.34").unwrap();
        let a_rdata = ARdata::new_from_addr(addr);
        let rdata = Rdata::SomeARdata(a_rdata);
        let rr = ResourceRecord::new(rdata);

        let mut cache = DnsCache::new();
        cache.set_max_size(1);
        cache.add(domain_name.clone(), rr);

        let _response_future = LookupFutureStub::lookup(domain_name, record_type, config).await;
        
        // TODO: test
    }  

    #[test]
    fn parse_response_ok() {
        let bytes: [u8; 50] = [
            //test passes with this one
            0b00100100, 0b10010101, 0b10010010, 0b00000000, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
            101, 115, 116, 3, 99, 111, 109, 0, 0, 16, 0, 1, 3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0,
            1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];
        let response_result: Result<Vec<u8>, ClientError> = Ok(bytes.to_vec());
        let response_dns_msg = parse_response(response_result);
        assert!(response_dns_msg.is_ok());
        if let Ok(dns_msg) = response_dns_msg {
            assert_eq!(dns_msg.get_header().get_qr(), true);
            assert_eq!(dns_msg.get_header().get_ancount(), 1);
            assert_eq!(dns_msg.get_header().get_rcode(), 0);
            println!("The message is: {:?}", dns_msg);
        }
    }
}