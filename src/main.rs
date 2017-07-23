extern crate bookmarkt;
extern crate env_logger;
extern crate rusqlite;

use bookmarkt::common::utils::load_file;
use bookmarkt::core::external::{ Converter, Netscape };
use bookmarkt::core::server;
//use bookmarkt::core::logic::html_parser::Parser;

fn main() {
    server::serve();
    println!("Server started on localhost:3000");
}
