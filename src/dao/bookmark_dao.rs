extern crate chrono;
extern crate rusqlite;

use common::bookmark::Bookmark;
use dao::query_parser::load_sql_file;
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

    pub fn insert(&self, b: Bookmark ) -> bool {
        //self.connection.execute(query, params: &[&ToSql])
        false

    }

    pub fn delete(&self) -> bool {
        false
    }

    pub fn read(&self, b: Bookmark) -> bool {
        false
    }

    pub fn update(&self, b: Bookmark ) -> bool {
        false
    }

    pub fn list(&self, b: Bookmark) -> bool {
        false
    }
}




pub fn hello_from_dao() -> String {
    "Hello, I am bookmark dao!".to_string()
}
