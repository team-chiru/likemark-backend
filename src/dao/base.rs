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

impl SqlConfig {
    pub fn get_read_sql(&self) -> String {
        self.read_sql.clone()
    }

    pub fn get_delete_sql(&self) -> String {
        self.delete_sql.clone()
    }

    pub fn get_insert_sql(&self) -> String {
        self.insert_sql.clone()
    }

    pub fn get_update_sql(&self) -> String {
        self.update_sql.clone()
    }

    pub fn get_list_sql(&self) -> String {
        self.list_sql.clone()
    }
}

pub trait Dao {
    fn insert(&self, e: &Entity) -> Result<i32, String>;
    fn delete(&self, c: &Criteria) -> Result<i32, String>;
    fn read(&self, c: &Criteria) -> Result<Entity, String>;
    fn update(&self, e: Entity) -> Result<i32, String>;
    fn list(&self, c: &Criteria) -> Result<Vec<Entity>, String>;
}
