use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "5000";

fn file_to_str(file_name: String) -> Option<String> {
    let mut file_str = String::new();

    match File::open(file_name).unwrap().read_to_string(&mut file_str) {
        Ok(_) => {
            // Prepare HTTP response with headers
            let res = format!(
                "HTTP/1.1 200 Ok\r\ncontent-length: {}\r\ncontent-type: text/html\r\n\r\n{}",
                file_str.len(),
                file_str
            );

            Some(res)
        }
        Err(e) => {
            println!("Error reading file: {:?}", e);
            None
        }
    }
}

fn bad_request_res(msg: String) -> String {
    let res = format!(
        "HTTP/1.1 400 Not Found\r\ncontent-length: {}\r\ncontent-type: text/plain\r\n\r\n{}",
        msg.len(),
        msg
    );

    res
}

fn handle_client(mut stream: TcpStream) {
    let mut buff = [0; 1024];
    let mut req = String::new();

    match stream.read(&mut buff) {
        Ok(size) => {
            req.push_str(&String::from_utf8_lossy(&buff[..size]));
            match req {
                _ if req.starts_with("GET / ") => {
                    // Retrieve file contents
                    let res = file_to_str(String::from("client/index.html")).unwrap();

                    // Send response to client
                    stream.write(res.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                _ => {
                    let res =
                        bad_request_res(String::from("404 BAD REQUEST This page does not exist."));

                    stream.write(res.as_bytes()).unwrap();
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
