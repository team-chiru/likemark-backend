extern crate bookmarkt;

use bookmarkt::dao::bookmark_dao::*;
use bookmarkt::dao::query_parser::*;
use bookmarkt::logic::services::*;


fn main() {
    println!("{}", hello_from_dao());
    println!("{}", hello_from_logic());

    println!("{}", load_sql_file("res/sql/bookmark/read.sql"));
}
