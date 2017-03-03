extern crate rusqlite;
use common::entity::EntityComposite;
use common::criteria::Criteria;

pub struct SqlConfig {
    pub connection: rusqlite::Connection,
    pub read_sql: String,
    pub delete_sql: String,
    pub insert_sql: String,
    pub update_sql: String,
    pub list_sql: String,
}

pub trait Query {}

pub trait Dao {
    fn insert<T>(s: &SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: EntityComposite;
    fn delete<T>(s: &SqlConfig, c: &Criteria) -> Result<i32, String>
        where T: EntityComposite;
    fn read<T>(s: &SqlConfig, c: &Criteria) -> Result<T, String>
        where T: EntityComposite;
    fn update<T>(s: &SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: EntityComposite;
    fn list<T>(s: &SqlConfig, c: &Criteria) -> Result<Vec<T>, String>
        where T: EntityComposite;
}
