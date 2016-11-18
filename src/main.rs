#![feature(field_init_shorthand)]

extern crate chrono;
extern crate rusqlite;

#[macro_use]
extern crate bookmarkt;

use bookmarkt::common::utils;
use bookmarkt::common::bookmark::Link;
use bookmarkt::dao::bookmark_dao::LinkDao;

use rusqlite::Connection;
use std::path::Path;

fn main() {
    let db = Connection::open(Path::new("res/BOOKMARKT.db")).unwrap();

    let base_link = Link {
        id: 1,
        name: String::from("test"),
        url: String::from("test_url"),
        rev_no: 0
    };

    let dao = LinkDao {
        connection: &db,
        read_sql: utils::dump_file("res/sql/bookmark/read.sql"),
        delete_sql: utils::dump_file("res/sql/bookmark/delete.sql"),
        insert_sql: utils::dump_file("res/sql/bookmark/insert.sql"),
        update_sql: utils::dump_file("res/sql/bookmark/update.sql"),
        list_sql: utils::dump_file("res/sql/bookmark/list.sql")
    };

    //TEST clear
    println!("\nTEST CLEAR");
    dao.clear();
    /*
    // TEST INSERT
    //println!("\nTEST INSERT");
    //dao.insert(base_link.clone());

    // TEST READ
    println!("\nTEST READ");
    match dao.read(base_link.clone()) {
        Ok(b) => println!("{}", b.name),
        Err(_) => panic!("error")
    }

    // TEST DELETE
    // println!("\nTEST DELETE");
    // dao.delete(base_link.clone());

    // TEST UPDATE
    //println!("\nTEST UPDATE");
    //dao.update(base_link.clone());
*/
    //TEST LIST
    println!("\nTEST LIST");
    let links = dao.list(base_link.clone());
    for link in links {
        println!("{}", link.id);
    }


}
