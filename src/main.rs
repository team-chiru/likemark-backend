extern crate bookmarkt;
<<<<<<< HEAD
extern crate env_logger;
extern crate rusqlite;

use bookmarkt::common::utils::load_file;
use bookmarkt::core::external::{ Converter, Netscape };
//use bookmarkt::core::logic::html_parser::Parser;

fn main() {
    let bookmark_file_path = String::from("tests/core/external/input/netscape_nested.html");
    let f = load_file(&bookmark_file_path);
    let bookmark_string = f.unwrap();
    Netscape::parse(bookmark_string);
}
