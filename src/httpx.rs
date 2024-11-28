pub mod send {
    use std::{
        fs::File,
        io::Read
    };

    pub fn file(file_name: &str) -> Option<(String, String)> {
        let mut file_str = String::new();

        match File::open(file_name).unwrap().read_to_string(&mut file_str) {
            Ok(_) => {
                // Prepare HTTP response with headers
                let headers = format!(
                    "HTTP/1.1 200 Ok\r\ncontent-length: {}\r\ncontent-type: text/html\r\n\r\n{}",
                    file_str.len(),
                    file_str
                );

                Some((headers, file_str))
            }
            Err(e) => {
                println!("Error reading file: {:?}", e);
                None
            }
        }
    }

}
