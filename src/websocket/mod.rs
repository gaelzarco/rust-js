use crate::helpers::bad_req_res;
use crate::websocket::header::{ create_hash, protocol_switch };

pub mod header;

pub fn handle_req(req: String) -> String {
    let split_r = req.rsplit("\r\n");
    let mut wsk = "";
    let mut err: bool = false; 

    for l in split_r {
        if l.starts_with("Sec-WebSocket-Key:") {
            if let Some(k) = l.split(":").last() {
                wsk = k.trim();
            } else {
                err = true;
                break
            }
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
    let wsk_hash = create_hash(wsk);
    let pro_switch = protocol_switch(wsk_hash);
    pro_switch 
}
