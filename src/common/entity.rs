use common::types::StructType;
use common::types::FnType;
use common::tree_id::*;
use common::utils::QueryValue;
use std::marker;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: i32,
    pub path: TreeId,
    pub name: String,
    pub url: String,
    pub struct_type: StructType,
    pub fn_type: FnType,
    pub rev_no: i32,
}

impl Entity {
    pub fn map_query(&self) -> HashMap<String, QueryValue> {
        let mut hash: HashMap<String, QueryValue> = HashMap::new();
        let clone = self.clone();

        hash.insert(String::from("id"), QueryValue::Integer(self.id));
        hash.insert(String::from("tree_id"), QueryValue::String(self.path.id()));
        hash.insert(String::from("name"), QueryValue::String(clone.name));
        hash.insert(String::from("url"), QueryValue::String(clone.url));

        let fn_type = clone.fn_type.into();
        hash.insert(String::from("fn_type"), QueryValue::String(fn_type));

        let struct_type = clone.struct_type.into();
        hash.insert(String::from("struct_type"), QueryValue::String(struct_type));

        hash
    }
}

pub trait EntityComposite {
    fn into_entities(&self) -> Vec<Entity>;
    fn from_entity(&Entity) -> Self;
    fn from_entities(Vec<Entity>) -> Vec<Self>
        where Self: marker::Sized;
}
