pub mod send {
    use std::{
        fs::File,
        io::Read
    };

    pub fn file(file_name: String) -> Option<String> {
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

    pub fn bad_req(msg: String) -> String {
        let res = format!(
            "HTTP/1.1 400 Not Found\r\ncontent-length: {}\r\ncontent-type: text/plain\r\n\r\n{}",
            msg.len(),
            msg
        );

        res
    }
}
