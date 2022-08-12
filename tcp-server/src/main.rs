use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        println!("connection established");
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        stream.write(&buf).unwrap();
    }
}
