extern crate chrono;
extern crate rusqlite;

use common::bookmark::Bookmark;
use dao::query_parser::*;
use self::rusqlite::Connection;

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
            connection: Connection::open_in_memory().unwrap(),
            read_sql: load_sql_file("res/sql/bookmark/read.sql"),
            delete_sql: load_sql_file("res/sql/bookmark/delete.sql"),
            insert_sql: load_sql_file("res/sql/bookmark/insert.sql"),
            update_sql: load_sql_file("res/sql/bookmark/update.sql"),
            list_sql: load_sql_file("res/sql/bookmark/list.sql")
        }
    }

    pub fn insert(&self, b: Bookmark ) {
        //self.connection.execute(query, params: &[&ToSql])


    }

    pub fn delete(&self) {

    }

    pub fn read(&self, b: Bookmark) {

    }

    pub fn update(&self, b: Bookmark ){

    }

    pub fn list(&self, b: Bookmark) {
        let parsed_query = parse_query(&b.to_btree(), String::from(&*self.list_sql));

        match self.connection.execute(parsed_query.as_str(), &[] ) {
            Ok(listed) => println!("{} rows were listed", listed),
            Err(err) => println!("listed failed: {}", err),
        }
    }
}




pub fn hello_from_dao() -> String {
    "Hello, I am bookmark dao!".to_string()
}
