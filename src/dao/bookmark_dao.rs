extern crate chrono;
extern crate rusqlite;

use common::bookmark::Bookmark;
use dao::query_parser::*;
use self::rusqlite::Connection;
use std::path::Path;

#[derive(Debug)]
pub struct BookmarkDao {
    connection: rusqlite::Connection,
    read_sql: String,
    delete_sql: String,
    insert_sql: String,
    update_sql: String,
    list_sql: String
}


impl BookmarkDao {
    pub fn new() -> BookmarkDao {
        BookmarkDao {
            connection: Connection::open(Path::new("res/BOOKMARKT.db")).unwrap(),
            read_sql: load_sql_file("res/sql/bookmark/read.sql"),
            delete_sql: load_sql_file("res/sql/bookmark/delete.sql"),
            insert_sql: load_sql_file("res/sql/bookmark/insert.sql"),
            update_sql: load_sql_file("res/sql/bookmark/update.sql"),
            list_sql: load_sql_file("res/sql/bookmark/list.sql")
        }
    }

    pub fn insert(&self, b: Bookmark) {
        let insert_query = parse_query(&b.to_btree(), String::from(&*self.insert_sql));

        match self.connection.execute(insert_query.as_str(), &[] ) {
            Ok(insert) => panic!("{} rows were inserted", insert),
            Err(err) => panic!("insert failed: {}", err),
        }
    }

    pub fn delete(&self, b: Bookmark) {
        let delete_query = parse_query(&b.to_btree(), String::from(&*self.delete_sql));

        match self.connection.execute(delete_query.as_str(), &[] ) {
            Ok(delete) => panic!("{} row was deleted", delete),
            Err(err) => panic!("delete failed: {}", err),
        }
    }

    pub fn read(&self, b: Bookmark) {
        let read_query = parse_query(&b.to_btree(), String::from(&*self.read_sql));

        match self.connection.execute(read_query.as_str(), &[] ) {
            Ok(read) => panic!("{} rows were readed", read),
            Err(err) => panic!("reading failed: {}", err),
        }
    }

    pub fn update(&self, b: Bookmark) {
        let update_query = parse_query(&b.to_btree(), String::from(&*self.update_sql));

        match self.connection.execute(update_query.as_str(), &[] ) {
            Ok(update) => panic!("{} rows were listed", update),
            Err(err) => panic!("listed failed: {}", err),
        }
    }

    pub fn list(&self, b: Bookmark) {
        let list_query = parse_query(&b.to_btree(), String::from(&*self.list_sql));

        match self.connection.execute(list_query.as_str(), &[] ) {
            Ok(list) => panic!("{} rows were listed", list),
            Err(err) => panic!("listed failed: {}", err),
        }
    }
}




pub fn hello_from_dao() -> String {
    "Hello, I am bookmark dao!".to_string()
}
