// pub mod bookmark_dao;
pub mod query_parser;
pub mod base;

// pub mod link_dao;
// pub mod node_dao;
pub mod entity_dao;
pub type EntityDao = entity_dao::EntityDao;

pub use super::common::*;

mod test;
