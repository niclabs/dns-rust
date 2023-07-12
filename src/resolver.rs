pub mod async_resolver;
pub mod config;
pub mod lookup;
pub mod slist;

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
    // runtime:Mutex<Runtime>
}

impl Resolver {
    pub fn new(config: ResolverConfig) -> Self {

        // let mut builder = runtime::Builder::new_current_thread();
        // builder.enable_all();

        // let runtime = builder.build().unwrap(); 

        let resolver = Resolver {
            config: config,
            // runtime:Mutex::new(runtime),
        };

        resolver
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let addr:std::net::SocketAddr = self.get_config().get_addr();

        //TODO: poner addr
        let tcp_listener = TcpListener::bind("127.0.0.1:5333").await?;
        let udp_socket = UdpSocket::bind("127.0.0.1:5333").await?;
        let mut udp_buffer = [0u8; 512];

        loop {
            let mut tcp_incoming = tcp_listener.accept();

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
                            if let Err(err) = handle_udp_client(&udp_data, src, async_resolver).await {
                                eprintln!("Error handling UDP client: {}", err);
                            }
                        });
                    }
                }
            }
        }
        // self.runtime.lock()?.block_on(udp_result|tcp_result);
    }

    //TODO: Funcion que hara solo una consulta
    pub fn lookup(_domain_name: &str){
        unimplemented!();
    }

}

impl Resolver {
    // Getters
    fn get_config(&self) -> &ResolverConfig {
        &self.config
    }
}
async fn handle_tcp_client(
    _tcp_stream: tokio::net::TcpStream,
    _async_resolver: AsyncResolver,
) -> Result<(), Box<dyn Error>> {
    // Maneja la comunicación TCP echo
    
    Ok(())
}

async fn handle_udp_client(
    _udp_data: &[u8],
    _src: std::net::SocketAddr,
    _async_resolver: AsyncResolver,
) -> Result<(), Box<dyn Error>> {
    // Maneja la comunicación UDP echo
    Ok(())
}


#[cfg(test)]
mod resolver_test {

    use crate::resolver::{slist::Slist, config::ResolverConfig};

    use std::collections::HashMap;
    use std::net::{IpAddr, Ipv4Addr};
    use std::result;

    use super::*;

    #[test]
    fn example() {
        let resolver_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        // let mut config = ResolverConfig::new(None, resolver_addr);

        let conf_default = ResolverConfig::default();

        let resolver = Resolver::new(conf_default);

        resolver.run();

        // tokio::spawn(async move {
        //     resolver.run().await;
        // });

        // let response = resolver.lookup("example.com", "A").unwrap();

        }
}