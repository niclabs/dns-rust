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

use futures_util::FutureExt;
use std::pin::Pin;
use std::task::{Poll,Context};
//TODO: Eliminar librerias
use std::net::{IpAddr,Ipv4Addr};
use futures_util::{future::Future,future};
use super::resolver_error::ResolverError;
use std::time::Duration;

//Future returned fron AsyncResolver when performing a lookup with rtype A
pub struct LookupIpFutureStub {
    name: DomainName,    // cache: DnsCache,
    query: Pin< Box< dyn Future <Output = Result<DnsMessage, ResolverError >>  > >,
    cache: DnsCache
}


impl  Future for LookupIpFutureStub {
    type Output = Result<DnsMessage, ResolverError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[POLL FUTURE]");

        loop {
            let query = self.query.as_mut().poll(cx);
            println!("[POLL] query ");

            match query {
                Poll::Pending => {
                    println!("  [Pending]");
                    return Poll::Pending;
                },
                Poll::Ready(Err(_)) => {
                    println!("  [ready err]");
                    self.query = Box::pin(lookup_stub(self.name.clone(), self.cache.clone()));
                },
                Poll::Ready(Ok(ip_addr)) => {
                    println!("  [Ready]");
                    return Poll::Ready(Ok(ip_addr));
                }
            }
        }
    }

}
    


impl LookupIpFutureStub {
    pub fn lookup(
        name: DomainName,
        cache:DnsCache
    ) -> Self {
        println!("[LOOKUP FUTURE]");
        
        Self { 
            name: name,
            query: future::err(ResolverError::Message("Empty")).boxed(), //FIXME: cambiar a otro tipo el error/inicio
            cache: cache
            }

    }
}

pub async fn lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
    name: DomainName,
    mut cache: DnsCache
) -> Result<DnsMessage,ResolverError> {
    println!("[LOOKUP STUB]");
    //FIXME: change values
    let server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let timeout:Duration = Duration::new(2, 0);
    
    //Connection type
    let conn = ClientUDPConnection::new(server, timeout);
    let mut udp_client = Client::new(conn);

    if let Some(cache_lookup) = cache.get(name.clone(), Rtype::A) {
        //Create query 
        let mut response_query = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Qtype::A,
            Qclass::IN,
            0,
            false,
            1
        );

        let mut question = Question::new();
        question.set_qclass(Qclass::IN);
        response_query.set_question(question);

        //Add Answer
        let answer: Vec<ResourceRecord> = cache_lookup
                                            .iter()
                                            .map(|rr_cache_value| rr_cache_value.get_resource_record())
                                            .collect::<Vec<ResourceRecord>>();
        response_query.set_answer(answer);
        return Ok(response_query);
    }
    let response = udp_client.query(name, "A","IN" );

    // println!("[LOOKUP STUB] response = {:?}",response);

    Ok(response)
}









