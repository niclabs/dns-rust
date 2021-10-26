use std::net::{TcpStream};
//use std::io::{self, BufRead, BufReader, Write};
use std::io::{BufRead, BufReader, Write};
use std::str::from_utf8;

pub fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server"); 
    let mut buffer: Vec<u8> = Vec::new();

    // let mut msg = String::new();
    // io::stdin().read_line(&mut msg).expect("Could not read from stdin");
    let default_message = String::from("Hello world!\n");

    println!("Sending msg: {}", &default_message);

    let msg_bytes = default_message.into_bytes();
    
    stream.write(&msg_bytes).expect("Could not write to server");

    let mut reader = BufReader::new(&stream); 
    reader.read_until(b'\n', &mut buffer).expect("Could not read buffer");
    print!("Received msg: {}", from_utf8(&buffer).expect("Could not write buffer as string"));
}
