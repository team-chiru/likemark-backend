extern crate bookmarkt;
extern crate chrono;


use bookmarkt::common::bookmark::Bookmark;
use bookmarkt::dao::bookmark_dao::*;
use bookmarkt::dao::query_parser::*;
use bookmarkt::logic::services::*;

use chrono::offset::local::Local;

fn main() {
    println!("---- test structure ----");
    println!("{}", hello_from_dao());
    println!("{}", hello_from_logic());

    println!("---- test load sql ----");
    //println!("{}", load_sql_file("res/sql/bookmark/read.sql"));
    let read_sql = load_sql_file("res/sql/bookmark/read.sql");
    let insert_sql = load_sql_file("res/sql/bookmark/insert.sql");
    let delete_sql = load_sql_file("res/sql/bookmark/delete.sql");
    let update_sql = load_sql_file("res/sql/bookmark/update.sql");
    let list_sql = load_sql_file("res/sql/bookmark/list.sql");

    println!("---- test yaml ----");
    let now = Local::now();
    let b = Bookmark { id: 1, name: String::from("test"), url: String::from("test.com"), time_created: now, stamp: now, rev_no: 0};
    //println!("{}", b.to_yaml());

    println!("---- test replace ----");
    let btree = b.to_btree();
    println!("{}", parse_query(&btree, read_sql));
    println!("{}", parse_query(&btree, delete_sql));
    println!("{}", parse_query(&btree, insert_sql));
    println!("{}", parse_query(&btree, update_sql));
    println!("{}", parse_query(&btree, list_sql));
}
