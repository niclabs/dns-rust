use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;

/// Resolves an TCP connection when it arrives.
///
/// The implementation of this function will depend on the purpose of the server.
///
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 100];
    'outer: while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 { 
                println!("Connection with {} was closed", stream.peer_addr().unwrap());
                break 'outer;
            }
            // echo 
            stream.write(&buffer[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}

pub fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection ok
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}

mod tests {
    use std::net::{TcpListener};
    use crate::tcp_client;
    use crate::tcp_server; 
    use std::thread;

    #[test]
    fn handle_client_test() {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");

        let _client = thread::spawn(|| {
            tcp_client::main();
        });

        'handling: for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                        tcp_server::handle_client(stream)
                    });
                    break 'handling; 
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        drop(listener);  
    }
}
