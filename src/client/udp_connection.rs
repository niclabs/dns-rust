
pub struct UDPConnection {
    name_server: SocketAddr,
    bind_addr: Option<SocketAddr>,
    timeout: Duration,
}

impl UDPConnection {
    // fn new(ip_addr:&str) -> self {
    //     UDPConnection
    // }


}

impl ClientConnection for UDPConnection {
    //TODO: funcion enviar
    fn send();
}
