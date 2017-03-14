extern crate bookmarkt;
extern crate env_logger;
extern crate rusqlite;

use bookmarkt::common::utils::load_file;
use bookmarkt::core::external::netscape::Netscape;
use bookmarkt::core::external::converter::Converter;
//use bookmarkt::core::logic::html_parser::Parser;

fn main() {
    let bookmark_file_path = String::from("res/bookmark_file/bookmark_chrome.html");
    let bookmark_string = load_file(&bookmark_file_path).unwrap();
    Netscape::parse(bookmark_string);
}
