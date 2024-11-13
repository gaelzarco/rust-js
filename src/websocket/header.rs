extern crate sha1_smol;
extern crate base64;
use base64::prelude::*;

const GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub mod protocol_types {
    pub const PROTOCOL: &str = "Sec-WebSocket-Protocol";
    pub const KEY: &str = "Sec-WebSocket-Key";
    pub const ACCEPT: &str = "Sec-WebSocket-Accept";
}

pub fn create_hash(ws_k: &str) -> String {
    let hash = sha1_smol::Sha1::from(ws_k.to_owned() + GUID).digest().bytes();
    let b64_hash = BASE64_STANDARD.encode(hash);
    b64_hash
}

pub fn protocol_switch(hash: String) -> String {
    let res = format!(
        "HTTP/1.1 101 Switching Protocols\r\n
        Upgrade: websocket\r\n
        Connection: Upgrade\r\n{}: {}\r\n
        {}: chat\r\n\r\n",
        protocol_types::PROTOCOL,
        protocol_types::ACCEPT,
        hash
    );

    res
}
