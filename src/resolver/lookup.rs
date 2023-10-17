use crate::client::udp_connection::ClientUDPConnection;
use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::type_rtype::Rtype;
use crate::message::resource_record::ResourceRecord;
use crate::client::{client_connection::ClientConnection,Client};
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::message::question::Question;
use crate::client::client_error::ClientError;

use std::sync::Arc;
use futures_util::{FutureExt,task::Waker};
use tokio::io::AsyncWriteExt;
use std::pin::Pin;
use std::task::{Poll,Context};
//TODO: Eliminar librerias
use std::net::{IpAddr,Ipv4Addr};
use futures_util::{future::Future,future};
use super::resolver_error::ResolverError;
use std::time::Duration;
use std::sync:: Mutex;
use crate::client::client_connection::ClientConnectionType;

//Future returned fron AsyncResolver when performing a lookup with rtype A
pub struct LookupIpFutureStub  {
    name: DomainName,    // cache: DnsCache,
    query:Pin< Box< dyn Future<Output = Result<DnsMessage, ResolverError>> >>,
    cache: DnsCache,
    conn: ClientConnectionType,
    waker: Option<Waker>,
}

impl Future for LookupIpFutureStub{ 
    type Output = Result<DnsMessage, ResolverError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[POLL FUTURE]");

        let query = self.query.as_mut().poll(cx);
        println!("[POLL query {:?}",query);

        match query {
            Poll::Pending => {
                println!("  [return pending]");
                return Poll::Pending;
            },
            Poll::Ready(Err(_)) => {
                println!("  [ready empty]");
                self.waker = Some(cx.waker().clone());

                tokio::spawn(
                    lookup_stub(self.name.clone(),self.cache.clone(),self.conn.clone(),self.waker.clone()));
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
        println!("[LOOKUP CREATE FUTURE]");;        
        
        Self { 
            name: name,
            query: future::err(ResolverError::Message("Empty")).boxed(), //FIXME: cambiar a otro tipo el error/inicio
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
    // query:Pin< Box< dyn Future <Output = Result<DnsMessage, ResolverError >>  > >,
) -> Result<DnsMessage,ResolverError> {
    println!("[LOOKUP STUB]");

    let mut new_query = DnsMessage::new_query_message(
        DomainName::new_from_string("example.com".to_string()),
        Qtype::A,
        Qclass::IN,
        0,
        false,
        1
    );

    let mut question = Question::new();
    question.set_qclass(Qclass::IN);
    new_query.set_question(question);



    //Loop up in cache
    if let Some(cache_lookup) = cache.get(name.clone(), Rtype::A) {
        println!("[LOOKUP STUB] cached data {:?}",cache_lookup);

        //Add Answer
        let answer: Vec<ResourceRecord> = cache_lookup
                                            .iter()
                                            .map(|rr_cache_value| rr_cache_value.get_resource_record())
                                            .collect::<Vec<ResourceRecord>>();
        new_query.set_answer(answer);
        return Ok(new_query);
    }

    //FIXME:
    let responseResult: Result<DnsMessage, ResolverError> = match conn {
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
    //para que en siguient eciclo de tokio despierte esta task
    if let Some(waker) = waker {
        println!("[LOOKUP STUB] wake");
        waker.wake();
    }

    println!("[LOOKUP STUB] return");
    responseResult
}




#[cfg(test)]
mod async_resolver_test {
    // use tokio::runtime::Runtime;
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
    
        let result = lookup_stub(name, cache, conn, waker).await;
        println!("[Test Result ] {:?}", result);
    }



    
}