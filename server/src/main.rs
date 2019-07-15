use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf: [u8; 128] = [0; 128];
        let _len = stream.read(&mut buf);
        print!(
            "{} > {}",
            stream.local_addr().unwrap(),
            str::from_utf8(&buf).unwrap()
        );
        let _len = stream.write(b"ok\n").unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1414").unwrap();
    println!("server listening on port 1414");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(_e) => {
                panic!("connection failed");
            }
        }
    }
}
