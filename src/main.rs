extern crate bookmarkt;
extern crate env_logger;
extern crate rusqlite;

use bookmarkt::common::utils::load_file;
use bookmarkt::core::external::netscape::Netscape;
use bookmarkt::core::external::base::Converter;
//use bookmarkt::core::logic::html_parser::Parser;

fn main() {
    let bookmark_file_path = String::from("./res/bookmark_file/bookmark_safari.html");
    let f = load_file(&bookmark_file_path);
    let bookmark_string = f.unwrap();

    Netscape::parse(bookmark_string);
}
