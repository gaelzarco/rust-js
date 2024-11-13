use std::{
    io::{ Read, Write },
    net::{TcpListener, TcpStream},
};

pub mod wsx;
pub mod helpers;

const HOST: &str = "127.0.0.1";
const PORT: &str = "5000";

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut r = String::new();

    match stream.read(&mut buf) {
        Ok(size) => {
            r.push_str(&String::from_utf8_lossy(&buf[..size]));
            match r {
                _ if r.starts_with("GET / ") => {
                    // Res file contents
                    let r = helpers::send::file(String::from("client/index.html")).unwrap();

                    // Send response to client
                    stream.write(r.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                _ if r.starts_with("GET /ws_test") => {
                    let r = wsx::handle_req(r);

                    stream.write(r.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                _ => {
                    let r = helpers::send::bad_req(String::from("404 BAD REQUEST This page does not exist."));
                    stream.write(r.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }

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
