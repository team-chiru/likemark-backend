use common::types::StructType;
use common::types::FnType;
use std::marker;

pub struct Entity {
    pub id: i32,
    pub parent_id: String,
    pub name: String,
    pub url: String,
    pub struct_type: StructType,
    pub fn_type: FnType,
    pub rev_no: i32,
}

pub trait FromEntity {
    fn from_entity(Entity) -> Self;
    fn from_entities(Vec<Entity>) -> Vec<Self>
        where Self: marker::Sized;
}
