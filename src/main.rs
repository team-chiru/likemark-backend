#![feature(field_init_shorthand)]

extern crate chrono;
extern crate rusqlite;

#[macro_use]
extern crate bookmarkt;

use bookmarkt::dao::bookmark_dao::BookmarkDao;
use rusqlite::Connection;
use std::path::Path;

fn main() {
    let db = Connection::open(Path::new("res/BOOKMARKT.db")).unwrap();

    let dao = BookmarkDao {
        connection: db,
        read_sql: dump_file!("res/sql/bookmark/read.sql"),
        delete_sql: dump_file!("res/sql/bookmark/delete.sql"),
        insert_sql: dump_file!("res/sql/bookmark/insert.sql"),
        update_sql: dump_file!("res/sql/bookmark/update.sql"),
        list_sql: dump_file!("res/sql/bookmark/list.sql")
    };

    
}
