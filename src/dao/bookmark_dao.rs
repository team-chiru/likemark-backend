extern crate chrono;
extern crate rusqlite;

use common::bookmark::Bookmark;
use self::rusqlite::Connection;

#[derive(Debug)]
pub struct BookmarkDao {
    connection: rusqlite::Connection
}


impl BookmarkDao {
    pub fn new() -> BookmarkDao {
        let conn = Connection::open_in_memory().unwrap();
        BookmarkDao { connection:conn }
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
