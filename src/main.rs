use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const GET:&[u8; 16] = b"GET / HTTP/1.1\r\n";
const STATUS_OK:&str = "HTTP/1.1 200 OK";
const STATUS_ERROR:&str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn web_response(mut stream : TcpStream, content : String, status_line : String) {
    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(GET) {
        let content = fs::read_to_string("index.html").unwrap();
        web_response(stream, content, STATUS_OK.to_string());
    } else {
        let content = fs::read_to_string("404.html").unwrap();
        web_response(stream, content, STATUS_ERROR.to_string());
    }
}
