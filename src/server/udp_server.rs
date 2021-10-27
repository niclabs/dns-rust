use std::net::UdpSocket;
use std::thread;

/// Resolves an UDP connection when it arrives.
///
/// The implementation of this function will depend on the purpose of the server.
///
/// # Examples
///
/// ```
/// let host_address = "127.0.0.1:34254";
///
/// // Creates an UDP socket
/// let socket = UdpSocket::bind(host_address).expect("Failed to bind host socket");
///
/// loop {
///     thread::spawn(move || {
///         handle_client(stream);
///     });
/// }
/// ```
///
fn handle_client(socket: UdpSocket) {
    let mut buf = [0; 100];

    // Receives a msg from the client
    let (_number_of_bytes, src_address) = socket.recv_from(&mut buf).expect("No data received");
    let buf_to_str = String::from_utf8(buf.to_vec()).unwrap();

    println!("Received msg: {}", buf_to_str);

    // Sends the echo msg to client
    socket
        .send_to(&buf, &src_address)
        .expect("failed to send message");
}

pub fn main() {
    let host_address = "127.0.0.1:34254";

    // Creates an UDP socket
    let socket = UdpSocket::bind(host_address).expect("Failed to bind host socket");

    loop {
        let socket_clone = socket.try_clone().expect("couldn't clone the socket");
        thread::spawn(move || {
            handle_client(socket_clone);
        });
    }
}

mod test {
    use crate::client::udp_client;
    use crate::server::udp_server;
    use std::net::UdpSocket;
    use std::thread;

    #[test]
    fn handle_client_test() {
        let server_host_address = "127.0.0.1:34254";

        // Creates an UDP socket
        let server_socket =
            UdpSocket::bind(server_host_address).expect("Failed to bind host socket");

        let handle = thread::spawn(|| {
            udp_server::handle_client(server_socket);
        });

        udp_client::main();

        handle.join().unwrap();
    }
}
