use crate::cache_data::host_data;
use crate::client::udp_connection::ClientUDPConnection;
use crate::dns_cache::DnsCache;
use crate::domain_name::{DomainName, self};
use crate::message::type_rtype::Rtype;
use crate::message::DnsMessage;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::client::{client_connection::ClientConnection,Client};
use crate::resolver::slist::Slist;

use std::{time::Duration};
use chrono::DateTime;
use futures_util::FutureExt;
use rand::{thread_rng, Rng};
use std::pin::Pin;
use std::task::{Poll,Context};
//TODO: Eliminar librerias
use std::net::{IpAddr,Ipv4Addr};
use std::io;
use futures_util::{future::Future,future};

use super::Resolver;
use super::resolver_error::ResolverError;


//Future returned fron AsyncResolver when performing a lookup with rtype A
pub struct LookupIpFutureStub {
    name: DomainName,
    // Servers for lookups
    hosts: Vec<IpAddr>,
    // cache: DnsCache,
    query: Pin< Box< dyn Future <Output = Result<IpAddr, ResolverError >>  > >,
    final_ip: Option<IpAddr>
}


impl  Future for LookupIpFutureStub {
    type Output = Result<IpAddr, ResolverError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[POLL FUTURE]");

        //Try polling answer query
        let response:Poll<Result<IpAddr, ResolverError>> = self.query.as_mut().poll(cx);  
        println!("[POLL FUTURE] theres response? => {:?}",response);


        let should_retry = match response { 
            //If query is not Ready Pending
            Poll::Pending => {
                println!("[POLL FUTURE] pending");
                return  Poll::Pending},

            //If query is ready 
            Poll::Ready(Ok(answer)) => {
                println!("[POLL FUTURE] Ok");
                false},
            Poll::Ready(Err(message)) =>{  //FIXME: cambiar a otro tipo el inicial
                //First poll
                println!("[POLL FUTURE] Poll::Ready(Error)");
                true

            }

            _ => {
                println!("[POLL FUTURE] otro");
                true

            }
        };

        if should_retry {
            println!("[POLL FUTURE] should try");
            self.query = LookupIpFutureStub::lookup_stub(self.name.clone()).boxed();
            println!("[POLL FUTURE] do lookup stub");
        }


        return Poll::Pending;
        
        
    }
    
}

impl LookupIpFutureStub {
    pub fn lookup(
        name: DomainName,
        hosts: Vec<IpAddr>,
        cache:DnsCache,
        final_ip: Option<IpAddr>
    ) -> Self {
        println!("[LOOKUP FUTURE]");
        
        Self { 
            name: name,
            hosts: hosts,
            query: future::err(ResolverError::Message("Empty")).boxed(), //FIXME: cambiar a otro tipo el error/inicio
            final_ip:final_ip
        }

    }

    async fn lookup_stub( //FIXME: podemos ponerle de nombre lookup_strategy y que se le pase ahi un parametro strategy que diga si son los pasos o si funciona como stub
        name: DomainName
        
    ) -> Result<IpAddr,ResolverError> {
        println!("[LOOKUP STUB]");


        //TODO: Buscar en cache


        //FIXME: sea parametro que se pase
        let server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout:Duration = Duration::new(2, 0);

        
        //Connection type
        let conn = ClientUDPConnection::new(server, timeout);
        let mut udp_client = Client::new(conn);
        let response = udp_client.query(name, "A","IN" ).to_owned();

        println!("[LOOKUP STUB] response = {:?}",response);

        Err(ResolverError::Message("Not implemented"))
    }
    
}









