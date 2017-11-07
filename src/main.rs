use std::net::TcpListener;
use std::prelude::*;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::net::TcpStream;

extern crate web_server_self;
use web_server_self::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(Box::new(move || { 
            handel_stream(&mut stream);
        }));
    }
}


fn handel_stream(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let filename = "hello.json";
    let path = "./src/";
    let mut file = File::open(format!("{}{}", path, filename)).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let header = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";

    stream.write(format!("{}{}", header, contents).as_bytes()).unwrap();
    stream.flush().unwrap();
}