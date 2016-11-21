#![feature(field_init_shorthand)]

extern crate chrono;
extern crate rusqlite;

#[macro_use]
extern crate bookmarkt;

use bookmarkt::common::utils;
use bookmarkt::common::bookmark::*;
use bookmarkt::dao::bookmark_dao::LinkDao;

use rusqlite::Connection;
use std::path::Path;

fn main() {
    let db = Connection::open(Path::new("res/BOOKMARKT.db")).unwrap();

    let base_link = Link {
        id: 8,
        name: String::from("test"),
        url: String::from("test_url"),
        rev_no: 0
    };

    let dao = LinkDao {
        connection: db,
        read_sql: utils::dump_file("res/sql/bookmark/read.sql"),
        delete_sql: utils::dump_file("res/sql/bookmark/delete.sql"),
        insert_sql: utils::dump_file("res/sql/bookmark/insert.sql"),
        update_sql: utils::dump_file("res/sql/bookmark/update.sql"),
        list_sql: utils::dump_file("res/sql/bookmark/list.sql")
    };

    //TEST clear
    println!("\nTEST CLEAR");
    //dao.clear();

    // TEST INSERT
    //println!("\nTEST INSERT");
    //dao.insert(base_link.clone());

    // TEST READ
    println!("\nTEST READ");
    let read_c = LinkCriteria::new();
    match dao.read(&read_c.id(1)) {
        Ok(b) => println!("{}", b.to_yaml()),
        Err(_) => panic!("error")
    }

    // TEST DELETE
    println!("\nTEST DELETE");
    let delete_c = LinkCriteria::new();
    match dao.delete(&delete_c.id(7)) {
        Ok(id) => println!("ROW WITH ID:{} HAS BEEN DELETED SUCCESSFULLY", id),
        Err(err) => println!("{}", err)
    };

    // TEST UPDATE
    println!("\nTEST UPDATE");
    match dao.update(base_link.clone()) {
        Ok(id) => println!("ROW WITH ID:{} HAS BEEN UPDATED SUCCESSFULLY", id),
        Err(err) => println!("{}", err)
    };

    //TEST LIST
    println!("\nTEST LIST");
    let list_c = LinkCriteria::new();
    let links = dao.list(&list_c.name(String::from("test")));
    for link in links.expect("list failed") {
        println!("{}", link.to_yaml());
    }
}
