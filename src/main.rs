use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0u8; 1024];
                let n = stream.read(&mut buf).unwrap();
                let request = String::from_utf8_lossy(&buf[..n]);
                let mut request_iter = request.split("\r\n\r\n");
                let req_header = request_iter.next().unwrap();
                let mut req_header_iter = req_header.split("\r\n");
                let req_line = req_header_iter.next().unwrap();
                let mut req_iter = req_line.split_whitespace();
                let mut target: Option<&str> = None;
                let mut version: Option<&str> = None;
                match req_iter.next() {
                    Some("GET") => {
                        target = req_iter.next();
                        version = req_iter.next();
                    }
                    _ => {}
                }
                let mut host: Option<&str> = None;
                let headers = req_header_iter.next().unwrap();
                let mut header_iter = headers.split("\r\n");
                while let Some(header) = header_iter.next() {
                    let mut line_iter = header.split_whitespace();
                    let method = line_iter.next().unwrap();
                    match method {
                        "Host" => {
                            host = line_iter.next();
                        }
                        _ => {}
                    }
                }
                if target == Some("/") {
                    stream
                        .write_all(
                            format!("{} 200 OK\r\n\r\n", version.unwrap())
                                .as_str()
                                .as_bytes(),
                        )
                        .unwrap();
                } else {
                    stream
                        .write_all(
                            format!("{} 404 Not Found\r\n\r\n", version.unwrap())
                                .as_str()
                                .as_bytes(),
                        )
                        .unwrap();
                }
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
