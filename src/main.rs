use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub mod httpx;
pub mod wsx;

const HOST: &str = "127.0.0.1";
const PORT: &str = "5000";

const _OK_RESPONSE: &str = "HTTP/1.1 200 Ok\r\n\r\n";
const _BAD_REQUEST: &str = "HTTP/1.1 500 Bad Request\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut req = String::new();

    match stream.read(&mut buf) {
        Ok(size) => {
            req.push_str(&String::from_utf8_lossy(&buf[..size]));

            if req.starts_with("GET / ") {
                let (status_line, content) = httpx::send::file("client/index.html").unwrap();
                stream
                    .write_all(format!("{}{}", status_line, content).as_bytes())
                    .unwrap();
            } else if req.starts_with("GET /ws ") {
                let (status_line, content) = wsx::header::upgrade(&req);
                stream.write(format!("{}{}", status_line, content).as_bytes()).unwrap();

                loop {
                    match stream.read(&mut buf) {
                        Ok(size) if size > 0 => {
                            let incoming_data = &buf[..size];
                            println!("Received WebSocket message: {:?}", incoming_data);
                        }
                        Err(e) => {
                            println!("WebSocket connection closed: {:?}", e);
                            break;
                        }
                        _ => {
                            println!("...");
                        }
                    }
                };
            } else {
                // Handle 404 Not Found
                stream.write_all(NOT_FOUND.as_bytes()).unwrap();
            }
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
