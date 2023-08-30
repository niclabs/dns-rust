use crate::client::udp_connection::ClientUDPConnection;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::client::{client_connection::ClientConnection,Client};


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
                    self.query = Box::pin(lookup_stub(self.name.clone()));
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
        name: DomainName
    ) -> Self {
        println!("[LOOKUP FUTURE]");
        
        Self { 
            name: name,
            query: future::err(ResolverError::Message("Empty")).boxed(), //FIXME: cambiar a otro tipo el error/inicio
            }

    }
}

pub async fn lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
    name: DomainName
) -> Result<DnsMessage,ResolverError> {
    println!("[LOOKUP STUB]");

    //TODO: Buscar en cache

    //FIXME: sea parametro que se pase
    let server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let timeout:Duration = Duration::new(2, 0);
    
    //Connection type
    let conn = ClientUDPConnection::new(server, timeout);
    let mut udp_client = Client::new(conn);
    let response = udp_client.query(name, "A","IN" );

    // println!("[LOOKUP STUB] response = {:?}",response);

    Ok(response)
}









