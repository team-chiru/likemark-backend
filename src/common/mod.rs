pub mod utils;
pub mod types;

pub mod entity;
pub mod criteria;

// pub trait Entity = entity::Entity;
// pub type Criteria = criteria::Criteria;
// pub type Link = entity::Entity;
// pub type Node = entity::Entity;

pub mod link;
pub mod node;
pub mod tree_id;

pub type Link = link::Link;
pub type Node = node::Node;
pub type Entity = entity::Entity;
pub type TreeId = tree_id::TreeId;
pub type Criteria = criteria::Criteria;
