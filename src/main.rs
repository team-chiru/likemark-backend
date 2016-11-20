#![feature(field_init_shorthand)]

extern crate chrono;
extern crate rusqlite;

#[macro_use]
extern crate bookmarkt;

use bookmarkt::common::utils;
use bookmarkt::common::bookmark::Link;
use bookmarkt::common::bookmark::LinkCriteria;
use bookmarkt::dao::bookmark_dao::LinkDao;

use rusqlite::Connection;
use std::path::Path;

fn main() {
    let db = Connection::open(Path::new("res/BOOKMARKT.db")).unwrap();

<<<<<<< HEAD
    let bb = Link {
=======
    let base_link = Link {
>>>>>>> 85cea06dcb94f4c586cc5a7c1a209e1321e527aa
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
<<<<<<< HEAD
    let read_criteria = LinkCriteria::new().id(1);
    match dao.read(&read_criteria) {
=======
    match dao.read(base_link.clone()) {
>>>>>>> 85cea06dcb94f4c586cc5a7c1a209e1321e527aa
        Ok(b) => println!("{}", b.name),
        Err(_) => panic!("error")
    }

<<<<<<< HEAD
    // TEST INSERT
    println!("\nTEST INSERT");
    dao.insert(&bb);

    // TEST DELETE
    // println!("\nTEST DELETE");
    // dao.delete(bb.clone());

    // TEST UPDATE
    println!("\nTEST UPDATE");

    let updated = Link {
        id: bb.id,
        name: String::from("updated"),
        url: bb.url,
        rev_no: bb.id + 1
    };
    dao.update(&updated);

    // TEST LIST
    println!("\nTEST LIST");
    let list_criteria = LinkCriteria::new().name(String::from("test"));
    let links = dao.list(&list_criteria);
=======
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
>>>>>>> 85cea06dcb94f4c586cc5a7c1a209e1321e527aa
    for link in links {
        println!("{}", link.id);
    }


}
