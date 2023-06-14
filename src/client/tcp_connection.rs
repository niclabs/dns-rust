
pub struct TCPConnection {
    name_server: SocketAddr,
    bind_addr: Option<SocketAddr>,
    timeout: Duration,
}

impl TCPConnection {
    // fn new(ip_addr:&str) -> self {
    //     UDPConnection
    // }


}

impl ClientConnection for TCPConnection {
    //TODO: funcion enviar
}
