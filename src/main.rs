use std::{
    io::{ Read, Write },
    net::{TcpListener, TcpStream},
};

pub mod wsx;
pub mod helpers;

const HOST: &str = "127.0.0.1";
const PORT: &str = "5000";
const _OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut req = String::new();

    match stream.read(&mut buf) {
        Ok(size) => {
            req.push_str(&String::from_utf8_lossy(&buf[..size]));

            let (status_line, content) = match &*req {
                r if r.starts_with("GET / ") => helpers::send::file("client/index.html").unwrap(),
                r if r.starts_with("GET /ws") => ( wsx::header::upgrade(r).unwrap(), String::from("Upgrade Successful") ),
                _ => ( NOT_FOUND.to_string(), String::from("404 BAD REQUEST This page does not exist.") )
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }

        Err(e) => println!("Error reading stream into buffer: {:?}", e),
    }
}

fn main() {
    let listener = match TcpListener::bind(HOST.to_owned() + ":" + PORT) {
        Ok(listener) => {
            println!("Game server Listening on {}:{}", &HOST, &PORT);
            listener
        }
        Err(e) => {
            panic!("Error instantiating TCP listener: {:?}", e);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error receiving stream from listener: {:?}", e),
        }
    }
}
