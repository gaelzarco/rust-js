use std::{
    fs::File,
    io::{ Read, Write },
    net::{TcpListener, TcpStream},
};
use base64::prelude::*;

const HOST: &str = "127.0.0.1";
const PORT: &str = "5000";

fn file_res(file_name: String) -> Option<String> {
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

fn bad_req_res(msg: String) -> String {
    let res = format!(
        "HTTP/1.1 400 Not Found\r\ncontent-length: {}\r\ncontent-type: text/plain\r\n\r\n{}",
        msg.len(),
        msg
    );

    res
}

fn form_res(ws_k: String) -> String {
    let guid = String::from("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let hash = sha1_smol::Sha1::from(ws_k + &guid).digest().bytes();
    let b64_hash = BASE64_STANDARD.encode(hash);

    let res = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\nSec-WebSocket-Protocol: chat\r\n\r\n",
       b64_hash
    );

    res
}

fn handle_req(req: String) -> String {
    let split_r = req.rsplit("\r\n");
    let mut ws_k = String::from("");
    let mut err: bool = false; 

    for l in split_r {
        if l.starts_with("Sec-WebSocket-Key:") {
            let k = l.to_owned().split_off(18);
            ws_k.push_str(k.trim());
            continue
        } 

        if l.starts_with("Sec-WebSocket-Protocol:") {
            let p = l.to_owned().split_off(23);

            if !p.contains("chat") {
                err = true;
                break
            }
        }

        if l.starts_with("Host:") {
            let h = l.to_owned().split_off(6);

            if h != "localhost:5000" {
                err = true;
                break
            }
        }

    }

    if err {
        println!("ERROR: RECEIVED POOR CLIENT WEBSOCKET CLIENT HANDSHAKE REQUEST");
        let res = bad_req_res(String::from("BAD CLIENT WEBSOCKET HANDSHAKE INITIATION")); 
        return res
    }

    println!("SUCCESSFULLY RECEIVED WEBSOCKET HANDSHAKE REQUEST");
    let res = form_res(ws_k);
    println!("RESPONSE SENT: {}", res);
    res
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut r = String::new();

    match stream.read(&mut buf) {
        Ok(size) => {
            r.push_str(&String::from_utf8_lossy(&buf[..size]));
            match r {
                _ if r.starts_with("GET / ") => {
                    // Res file contents
                    let r = file_res(String::from("client/index.html")).unwrap();

                    // Send response to client
                    stream.write(r.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                _ if r.starts_with("GET /ws_test") => {
                    let r = handle_req(r);

                    stream.write(r.as_bytes()).unwrap();
                    stream.flush().unwrap();

                }
                _ => {
                    let res = bad_req_res(String::from("404 BAD REQUEST This page does not exist."));
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
