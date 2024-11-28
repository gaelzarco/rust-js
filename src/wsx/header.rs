use base64::prelude::*;
use sha1_smol::Sha1;

const GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub mod protocol_types {
    pub const PROTOCOL: &str = "Sec-WebSocket-Protocol";
    pub const KEY: &str = "Sec-WebSocket-Key";
    pub const ACCEPT: &str = "Sec-WebSocket-Accept";
}

pub fn create_hash(ws_k: &str) -> String {
    let hash = Sha1::from(ws_k.to_owned() + GUID).digest().bytes();
    BASE64_STANDARD.encode(hash)
}

pub fn upgrade(req: &str) -> (String, String) {
    let split_r = req.rsplit("\r\n");
    let mut wsk = "";
    let mut err: bool = false;

    for l in split_r {
        if l.starts_with("Sec-WebSocket-Key:") {
            if let Some(k) = l.split(":").last() {
                wsk = k.trim();
            } else {
                err = true;
                break;
            }
            continue;
        }

      //  if l.starts_with("Sec-WebSocket-Protocol:") {
      //      let p = l.to_owned().split_off(23);

      //      if !p.contains("chat") {
      //          err = true;
      //          break;
      //      }
      //  }

      //  if l.starts_with("Host:") {
      //      let h = l.to_owned().split_off(6);

      //      if h != "localhost:5000" {
      //          err = true;
      //          break;
      //      }
      //  }
    }

    let wsk_hash = create_hash(wsk);

    if err {
        (
            format!("HTTP/1.1 400 Bad Request\r\n\r\n"),
            String::from("400 Bad Request. Missing Headers."),
        )
    } else {
        (
            format!(
                "HTTP/1.1 101 Switching Protocols\r\n
                Upgrade: websocket\r\n
                Connection: Upgrade\r\n
                {}: {}\r\n
                {}: chat\r\n\r\n",
                protocol_types::ACCEPT,
                wsk_hash,
                protocol_types::PROTOCOL
            ),
            String::from("101 Switching Protocols. Upgrading to WebSocket Protocol."),
        )
    }
}
