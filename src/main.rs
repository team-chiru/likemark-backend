extern crate bookmarkt;
extern crate env_logger;

extern crate rusqlite;
use bookmarkt::core::logic::html_parser::Parser;

fn main() {
    let bookmark_file_path = String::from("res/bookmark_file/bookmark_chrome.html");
    let mut parser = Parser::new(bookmark_file_path);
    parser.import();
}
