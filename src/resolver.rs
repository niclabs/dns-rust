pub mod async_resolver;
pub mod config;
pub mod lookup;
pub mod slist;

use crate::dns_cache;
use crate::domain_name::{DomainName, self};
use crate::message::{DnsMessage};
use crate::resolver::async_resolver::AsyncResolver;
use crate::resolver::config::ResolverConfig;
use crate::message::type_rtype::Rtype;

use tokio::runtime::{self,Runtime};
use tokio::net::{TcpListener,UdpSocket};
use tokio::io::{BufReader,AsyncBufRead, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::broadcast;
use tokio::pin;

use std::error::Error;
use std::sync::Mutex;
pub struct Resolver {
    config: ResolverConfig,
}

impl Resolver {
    pub fn new(config: ResolverConfig) -> Self {

        let resolver = Resolver {
            config: config,
        };

        resolver
    }

    pub async fn run(&self)  {
        println!("RUNNING");
        let addr:std::net::SocketAddr = self.get_config().get_addr();

        //TODO: poner addr
        let tcp_listener = TcpListener::bind("127.0.0.1:5333").await.unwrap();
        let udp_socket = UdpSocket::bind("127.0.0.1:5333").await.unwrap();
        let mut udp_buffer = [0u8; 512];

        loop {
            println!("[LOOP]");
            let tcp_incoming = tcp_listener.accept();
            

            tokio::select! {
                tcp_result = tcp_incoming => {
                    if let Ok((tcp_stream, _)) = tcp_result {
                        let async_resolver = AsyncResolver::new(self.get_config());
                        tokio::spawn(async move {
                            if let Err(err) = handle_tcp_client(tcp_stream, async_resolver).await {
                                eprintln!("Error handling TCP client: {}", err);
                            }
                        });
                    }
                },
                udp_result = udp_socket.recv_from(&mut udp_buffer) => {
                    if let Ok((size, src)) = udp_result {
                        let udp_data = udp_buffer[..size].to_vec(); // Clonar los datos en un nuevo Vec<u8>
                        let async_resolver = AsyncResolver::new(self.get_config());
                        tokio::spawn(async move {
                            if let Err(err) = handle_udp_client(&udp_data, src, async_resolver,udp_socket).await {
                                eprintln!("Error handling UDP client: {}", err);
                            }
                        });
                    }
                }
            }
        }
    }

    //TODO: Funcion que hara solo una consulta
    pub fn lookup(&self, dns_query:DnsMessage){
        let async_resolver = AsyncResolver::new(self.get_config());
        async_resolver.inner_lookup(dns_query);

    }

}

// Getters
impl Resolver {
    
    fn get_config(&self) -> &ResolverConfig {
        &self.config
    }
}

async fn handle_tcp_client(
    mut tcp_stream: tokio::net::TcpStream,
    async_resolver: AsyncResolver,
) -> Result<(), Box<dyn Error>> {
    println!("[TCP]");
    //TODO:transformar bytes a DNSMESSAGE
    // async_resolver.inner_lookup();
    let mut buf = Vec::with_capacity(4096);

    // Try to read data, this may still fail with `WouldBlock`
    // if the readiness event is a false positive.
    match tcp_stream.try_read_buf(&mut buf) {
        Ok(n) => {
            println!("read {} bytes", n);
        },
        Err(e) => {
            println!("[ERROR]");
            return Err(e.into());
        }
    }

    // Imprimir los bytes recibidos
    println!("Bytes recibidos TCP: {:?}", buf);
    
    Ok(())
}

async fn handle_udp_client(
    udp_data: &[u8],
    _src: std::net::SocketAddr,
    async_resolver: AsyncResolver,
    udp_socket:UdpSocket
) -> Result<(), Box<dyn Error>> {
    //TODO:transformar bytes a DNSMESSAGE
    println!("Bytes recibidos UDP: {:?}", udp_data);
    // async_resolver.inner_lookup();



    Ok(())
}




#[cfg(test)]
mod resolver_test {
    use super::*;

    #[tokio::test]
    async fn example() {
        let conf_default = ResolverConfig::default();
        let resolver = Resolver::new(conf_default);

        resolver.run().await; 

        //Correr en otra consola 
        //dig @127.0.0.1 -p 5333 uchile.cl +tcp
        //dig @127.0.0.1 -p 5333 uchile.cl 
    }
}