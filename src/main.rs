#![feature(field_init_shorthand)]

extern crate chrono;
extern crate rusqlite;

#[macro_use]
extern crate bookmarkt;

use bookmarkt::common::bookmark::Link;
use bookmarkt::dao::bookmark_dao::LinkDao;

use rusqlite::Connection;
use std::path::Path;

fn main() {
    let db = Connection::open(Path::new("res/BOOKMARKT.db")).unwrap();

    let base_bookmark = Link {
        id: 0,
        name: String::from("test"),
        url: String::from("test_url"),
        rev_no: 0
    };

    let dao = LinkDao {
        connection: &db,
        read_sql: dump_file!("res/sql/bookmark/read.sql"),
        delete_sql: dump_file!("res/sql/bookmark/delete.sql"),
        insert_sql: dump_file!("res/sql/bookmark/insert.sql"),
        update_sql: dump_file!("res/sql/bookmark/update.sql"),
        list_sql: dump_file!("res/sql/bookmark/list.sql")
    };

    // TEST READ
    println!("\nTEST READ");
    match dao.read(base_bookmark) {
        Ok(b) => println!("{}", b.name),
        Err(_) => panic!("error")
    }

    // test insert
    println!("\nTEST INSERT");

    // test delete
    println!("\nTEST DELETE");

    // test list
    println!("\nTEST LIST");

}
