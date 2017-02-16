extern crate rusqlite;
use common::entity::FromEntity;
//use common::criteria::CriteriaBuilder;

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
    // fn insert(&self, e: &Entity) -> Result<i32, String>;
    // fn delete(&self, c: &Criteria) -> Result<i32, String>;
    fn read<T>(s: SqlConfig, c: &T) -> Result<T, String>
        where T: FromEntity;
    // fn update(&self, e: Entity) -> Result<i32, String>;
    // fn list(&self, c: &Criteria) -> Result<Vec<Entity>, String>;
}

//pub struct EntityDao {}
//impl Dao for EntityDao {
//    fn read<T, C>(s: SqlConfig, c: &C) -> Result<T, String>
//        where T: Entity,
//              C: Criteria
//    {
//        Ok()
//    }
//}
