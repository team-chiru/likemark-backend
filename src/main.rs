extern crate bookmarkt;
extern crate chrono;

use bookmarkt::common::bookmark::Bookmark;
use bookmarkt::dao::bookmark_dao::*;
//use bookmarkt::dao::query_parser::*;
use bookmarkt::logic::services::*;

use chrono::offset::local::Local;

fn main() {
    println!("{}", hello_from_dao());
    println!("{}", hello_from_logic());

    //println!("{}", load_sql_file("res/sql/bookmark/read.sql"));

    let now = Local::now();
    let b = Bookmark { id: 1, name: String::from("test"), url: String::from("test"), stamp: now, rev_no: 0};
    println!("{}", b.to_yaml());



}
