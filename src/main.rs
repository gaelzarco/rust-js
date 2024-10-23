use std::{
    fs::File,
    io::{ Read, Write },
    net::{ TcpListener, TcpStream },
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "5000";

fn file_to_str(file_name: String) -> Option<String> {
    let mut file_str = String::new();

    match File::open(file_name).unwrap().read_to_string(&mut file_str) {
        Ok(_) => {
            Some(file_str)
        },
        Err(e) => {
            println!("Error reading file: {:?}", e);
            None 
        }
    } 
}

fn handle_client(mut stream: TcpStream) {
    let mut buff = [0; 1024];
    let mut req = String::new();

    match stream.read(&mut buff) {
        Ok(size) => {
            req.push_str(&String::from_utf8_lossy(&buff[..size]));
            match req {
               _r if req.starts_with("GET / ") => {
                   println!("Client requesting index page...");
                    // Serve the file content
                   match file_to_str(String::from("client/index.html")) {
                        Some(content) => {
                            // Prepare HTTP response with headers
                            let res = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                                content.len(),
                                content
                            );
                            
                            println!("Response sent to client: {}", res);
                            // Send response to client
                            stream.write(res.as_bytes()).unwrap();
                            stream.flush().unwrap();
                        }
                        None => {
                            
                        }
                   }

               },
               _ => (),
            }
        }
        Err(e) => println!("Error reading stream into buffer: {:?}", e),
    }
}

fn main() {
    let listener = match TcpListener::bind(HOST.to_owned() + ":" + PORT) {
        Ok(listener) => {
            println!("Game server Listening on {}:{}", &HOST, &PORT );
            listener
        },
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
