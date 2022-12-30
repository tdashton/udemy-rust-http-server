#![allow(dead_code)]

use http::method::Method;
use http::request::Request;
use server::Server;
use std::env;

use crate::website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_borrow: &str = &string;
    let string_literal = "1023";

    let pos_colon = string.find(":").unwrap();
    let string_slice = &string[pos_colon + 1..]; // this is a reference a string slice of the last part of the String

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("{}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
