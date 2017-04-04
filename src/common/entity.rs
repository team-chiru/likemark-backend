use common::types::StructType;
use common::types::FnType;
use common::TreeId;
<<<<<<< Updated upstream
=======
use std::marker;
>>>>>>> Stashed changes

#[derive(Debug, Default, Builder, Clone)]
pub struct Entity {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub path: Option<TreeId>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub struct_type: Option<StructType>,
    pub fn_type: Option<FnType>,
    pub rev_no: Option<i32>
<<<<<<< Updated upstream
=======
}

pub trait EntityComposite {
    fn is_dead(&self) -> bool;
    fn into_entities(&self) -> Vec<Entity>;
    fn from_entity(&Entity) -> Self;
    fn from_entities(Vec<Entity>) -> Vec<Self>
        where Self: marker::Sized;
>>>>>>> Stashed changes
}
