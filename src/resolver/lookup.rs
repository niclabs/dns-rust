use crate::cache_data::host_data;
use crate::client::udp_connection::ClientUDPConnection;
use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::type_rtype::Rtype;
use crate::message::DnsMessage;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::client::client_connection::ClientConnection;
use crate::resolver::slist::Slist;

use std::{time::Duration};
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
    // Servers for lookups
    hosts: Vec<IpAddr>,
    query: Result<IpAddr, ResolverError>,
    final_ip: Option<IpAddr>
}


impl  Future for LookupIpFutureStub {
    type Output = Result<IpAddr, ResolverError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!();
        
    }
    
}

impl LookupIpFutureStub {
    pub fn lookup(
        hosts: Vec<IpAddr>,
        cache:DnsCache,
        final_ip: Option<IpAddr>
    ) -> Self {
        
        Self { 
            hosts: hosts,
            query: Err(ResolverError::Message("NONE")),
            final_ip:final_ip
        }

    }
    
}








//Lookup seria el nuevo ResolverQuery
pub struct Lookup {
    //Cache 
    cache: DnsCache,
    // names: Vec<DomainName>,
    //Recursive Desire
    // rd: bool,
    //Cache use
    // use_cache :bool,
    // Queries quantity for each query, before the resolver panic in a Temporary Error
    // retry:u16,
    //Record Type
    rtype: Rtype,
    
    query: DnsMessage,
}

impl Future for Lookup {
    //TODO: Cambiar io::Error a ResolveError
    type Output = Result<DnsMessage, io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)->Poll<Self::Output>{
        unimplemented!();
    }
}

impl Lookup {
    
    pub fn lookup(domain_name:DomainName, qtype:Qtype, qclass:Qclass, 
                server:&Slist, cache:DnsCache, timeout:Duration ) {

        //FIXME: enviar a todos los servidores

        
        //Connection type
        //TODO: Cambiar a que envie a todos los servidores
        let google_resolver:IpAddr =  IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let conn = ClientUDPConnection::new(google_resolver, timeout);

        // Create random generator
        let mut rng = thread_rng();
        // Create query id
        let query_id: u16 = rng.gen();
        // Create query msg
        let query: DnsMessage = DnsMessage::new_query_message(
            domain_name,
            qtype, 
            qclass,
            0,
            false,
            query_id,
        );
        
        //conn is in charge of send query
        let response:DnsMessage = match conn.send(query) {
            Ok(dns_message) => dns_message,
            Err(e) => panic!("Error: {}",e),
        };

        //TODO: actualizar cache,


    }
}