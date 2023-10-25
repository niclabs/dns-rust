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
//Future returned fron AsyncResolver when performing a lookup with rtype A
pub struct LookupIpFutureStub  {
    name: DomainName,    // cache: DnsCache,
    query_answer: Arc<std::sync::Mutex<Pin<Box<dyn futures_util::Future<Output = Result<DnsMessage, ResolverError>> + Send>>>>,
    cache: DnsCache,
    conn: ClientConnectionType,
    waker: Option<Waker>,
}

impl Future for LookupIpFutureStub{ 
    type Output = Result<DnsMessage, ResolverError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[POLL FUTURE]");

        let query = self.query_answer.lock().unwrap().as_mut().poll(cx)  ;
        println!("[POLL query {:?}",query);

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
                    lookup_stub(self.name.clone(),self.cache.clone(),self.conn.clone(),self.waker.clone(),referenced_query));
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
        conn: ClientConnectionType,
    ) -> Self {
        println!("[LOOKUP CREATE FUTURE]");
        
        Self { 
            name: name,
            query_answer:  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed())),  //FIXME: cambiar a otro tipo el error/inicio
            cache: cache,
            conn: conn,
            waker: None,
            }

    }

}
pub async fn  lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
    name: DomainName,
    mut cache: DnsCache,
    conn: ClientConnectionType,
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


    //loop

    //FIXME:
    let response_result: Result<DnsMessage, ResolverError> = match conn {
        ClientConnectionType::TCP(client_conn) => {
            match client_conn.send(new_query) {
                Err(_) => Err(ResolverError::Message("Error: Receiving DNS message")),
                Ok(val) => {
                    Ok(val)
                },
            }
        }

        ClientConnectionType::UDP(client_conn) => {
            match client_conn.send(new_query) {
                Err(_) => Err(ResolverError::Message("Error: Receiving DNS message")),
                Ok(val) => {
                    Ok(val)},
            }
        }
    };  
    println!("[] {:?}",response_result);

    let mut future_query = referenced_query.lock().unwrap();
    *future_query = future::ready(response_result).boxed(); // TODO: check if it workingas expected
    
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
    
        let google_server: IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);
    
        let client_udp = ClientUDPConnection::new(google_server, timeout);
        let conn = ClientConnectionType::UDP(client_udp);
        let query =  Arc::new(Mutex::new(future::err(ResolverError::Message("Empty")).boxed()));
    
        lookup_stub(name, cache, conn, waker,query).await;
        // println!("[Test Result ] {:?}", result);
    }



    
}