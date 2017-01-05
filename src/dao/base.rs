extern crate rusqlite;

use common::Entity;
use common::Criteria;

pub struct SqlConfig {
    pub connection: rusqlite::Connection,
    pub read_sql: String,
    pub delete_sql: String,
    pub insert_sql: String,
    pub update_sql: String,
    pub list_sql: String,
}

pub trait Dao {
    fn insert(&self, e: &Entity) -> Result<i32, String>;
    fn delete(&self, c: &Criteria) -> Result<i32, String>;
    fn read(&self, c: &Criteria) -> Result<Entity, String>;
    fn update(&self, e: Entity) -> Result<i32, String>;
    fn list(&self, c: &Criteria) -> Result<Vec<Entity>, String>;
}
