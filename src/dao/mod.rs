mod sql_config;
pub type SqlConfig = sql_config::SqlConfig;

pub mod query;

pub mod entity_dao;
pub type EntityDao = entity_dao::EntityDao;

pub use super::common::*;

mod dao;
pub use self::dao::Dao;
