use common::types::StructType;
use common::types::FnType;
use common::tree_id::*;
use std::marker;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: i32,
    pub tree_id: TreeId,
    pub name: String,
    pub url: String,
    pub struct_type: StructType,
    pub fn_type: FnType,
    pub rev_no: i32,
}

pub trait FromEntity {
    fn from_entity(&Entity) -> Self;
    fn from_entities(Vec<Entity>) -> Vec<Self>
        where Self: marker::Sized;
}
