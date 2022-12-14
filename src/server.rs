use request::ParseError;

use crate::http::{Request, Response, StatusCode, request};
use std::convert::TryFrom;
use std::io::Read; // this is a trait
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        print!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        return Self {
            addr: addr
        };
    }

    pub fn run(self: Self, mut handler: impl Handler) {
        println!("Listening on {:?}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() { // same as a break statement
//                Ok(_) => { } // we tell the compiler to ignore the value
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024]; // create an array of 1024 zeros
                    match stream.read(&mut buffer) { // read is actually from std::io::Read
                        Ok(_) => {
                            println!("Received request from {}\r\n{}", addr, String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection {}", e),
                    };
                },
                Err(e) => println!("Failed to establish a connection {}", e),
                // _ => // default case 
                // "a" | "b" => // match either of them
            }
        }
    }
}
