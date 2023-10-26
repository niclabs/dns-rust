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
use crate::client::client_connection::ClientConnectionType;

use crate::client::udp_connection::ClientUDPConnection;
use crate::client::tcp_connection::ClientTCPConnection;
//Future returned fron AsyncResolver when performing a lookup with rtype A
pub struct LookupIpFutureStub  {
    name: DomainName,    // cache: DnsCache,
    query_answer: Arc<std::sync::Mutex<Pin<Box<dyn futures_util::Future<Output = Result<DnsMessage, ResolverError>> + Send>>>>,
    cache: DnsCache,
    name_servers: Vec<(ClientUDPConnection, ClientTCPConnection)>,
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
                    lookup_stub(self.name.clone(),self.cache.clone(),self.name_servers.clone(),self.waker.clone(),referenced_query));
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
    
impl LookupIpFutureStub{
    pub fn lookup(
        name: DomainName,
        cache:DnsCache,
        name_servers: Vec<(ClientUDPConnection, ClientTCPConnection)>,
    ) -> Self {
        println!("[LOOKUP CREATE FUTURE]");
        
        Self { 
            name: name,
            query_answer:  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed())),  //FIXME: cambiar a otro tipo el error/inicio
            cache: cache,
            name_servers: name_servers,
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
) {
    println!("[LOOKUP STUB]");

    // Create random generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    let mut new_query = DnsMessage::new_query_message(
        name.clone(),
        Qtype::A,
        Qclass::IN,
        0,
        false,
        query_id
    );

    // Loop up in cache
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

    //loop
    for (conn_udp,conn_tcp) in name_servers.iter() { 
        
        // UDP
        let result_response = conn_udp.send(new_query.clone());
        match result_response {
            Ok(response_ok) => {
                response = response_ok;
                println!("***********************************");
                break;
            },
            Err(_) => (),//TODO: when udp dont workout send with
        };


        //TCP
    }
   
    // println!("[] {:?}",response);

    let mut future_query = referenced_query.lock().unwrap();
    *future_query = future::ready(Ok(response)).boxed(); // TODO: check if it workingas expected
    
    //wake up task
    if let Some(waker) = waker {
        println!("[LOOKUP STUB] wake");
        waker.wake();
    }

    println!("[LOOKUP STUB] return");
    
}




#[cfg(test)]
mod async_resolver_test {
    // use tokio::runtime::Runtime;
    use crate::client::udp_connection::ClientUDPConnection;
    use crate::{ domain_name::DomainName, dns_cache::DnsCache};
    use super::lookup_stub;
    use tokio::time::Duration;
    use std::net::{IpAddr, Ipv4Addr};
    use super::*;
     
    #[tokio::test]
    async fn lookup_stub_test() {
        let name = DomainName::new_from_string("example.com".to_string());
        let cache = DnsCache::new();
        let waker = None;
    
        let query =  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed()));

        // Create vect of name servers
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
    
        let name_servers = vec![(conn_udp,conn_tcp)];
        lookup_stub(name, cache, name_servers, waker,query).await;
        // println!("[Test Result ] {:?}", result);
    }



    
}