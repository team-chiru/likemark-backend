extern crate rusqlite;

use common::Entity;
use common::Criteria;

pub struct SqlConfig {
    pub connection: rusqlite::Connection,
    read_sql: String,
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
    fn insert(e: &Entity, config: &SqlConfig) -> Result<i32, String>;
    fn delete(c: &Criteria, config: &SqlConfig) -> Result<i32, String>;
    fn read(c: &Criteria, config: &SqlConfig) -> Result<Entity, String>;
    fn update(e: Entity, config: &SqlConfig) -> Result<i32, String>;
    fn list(c: &Criteria, config: &SqlConfig) -> Result<Vec<Entity>, String>;
}
