use std::net::UdpSocket;


pub fn main() {
    let host_address = "127.0.0.1:34255";
    let send_address = "127.0.0.1:34254";
    let default_message = String::from("Hello world!");

    // Creates an UDP socket
    let socket = UdpSocket::bind(host_address).expect("Failed to bind host socket");

    println!("Sending msg: {}", &default_message);

    // Converts the string msg to bytes
    let msg_bytes = default_message.into_bytes();

    // Sends the bytes to send_address
    socket.send_to(&msg_bytes, &send_address).expect("failed to send message");

    let mut buf = [0; 100];

    // Receives an echo msg from the server
    let (_number_of_bytes, _src_address) = socket.recv_from(&mut buf).expect("No data received");
    let buf_to_str = String::from_utf8(buf.to_vec()).unwrap();

    println!("Received msg: {}", buf_to_str);
}
