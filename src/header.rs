extern crate sha1_smol;
extern crate base64;

use base64::prelude::*;

const GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub mod header_names {
    pub const PROTOCOL: &str = "Sec-WebSocket-Protocol";
    pub const KEY: &str = "Sec-WebSocket-Key";
    pub const ACCEPT: &str = "Sec-WebSocket-Accept";
}

fn form_res(ws_k: &str) -> &str {
    let hash = sha1_smol::Sha1::from(ws_k.to_owned() + GUID).digest().bytes();
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
        println!("{:?}", l);
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

