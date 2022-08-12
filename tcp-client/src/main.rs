use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();
    // getting response back
    let mut buf = [0; 5];

    stream.read(&mut buf).unwrap();
    println!(
        "response back from the server: {:?}",
        str::from_utf8(&buf).unwrap()
    );
}
