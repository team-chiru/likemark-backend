extern crate rusqlite;

use common::bookmark::Entity;
use common::bookmark::Criteria;

trait Dao {
    fn insert(&self, e: &Entity) -> Result<i32, String>;
    fn delete(&self, c: &Criteria) -> Result<i32, String>;
    fn read(&self, c: &Criteria) -> Result<Entity, String>;
    fn update(&self, e: &Entity) -> Result<i32, String>;
}

trait SqliteDao: Dao {
    fn get_connection() -> rusqlite::Connection {
        match Connection::open_in_memory() {
            Ok(c) => c,
            Err(err) => panic!("OPEN TEST DB FAILED: {}", err),
        }
    }
}

#[derive(Debug)]
pub struct LinkDao {
    pub connection: rusqlite::Connection,
    pub read_sql: String,
    pub delete_sql: String,
    pub insert_sql: String,
    pub update_sql: String,
    pub list_sql: String,
}
