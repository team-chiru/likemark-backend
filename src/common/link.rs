use common::types::FnType;
use common::types::StructType;
use common::utils::QueryValue;
use common::tree_id::TreeId;

use common::entity::EntityComposite;
use common::entity::Entity;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub path: TreeId,
    pub name: String,
    pub url: String,
    pub fn_type: FnType,
}

impl EntityComposite for Link {
    fn from_entity(entity: &Entity) -> Self {
        let clone = entity.clone();

        Link {
            id: clone.id,
            path: clone.tree_id,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
        }
    }

    fn map_query(&self) -> HashMap<String, QueryValue> {
        let mut hash: HashMap<String, QueryValue> = HashMap::new();
        let clone = self.clone();

        hash.insert(String::from("id"), QueryValue::Integer(self.id));
        hash.insert(String::from("tree_id"), QueryValue::String(self.path.id()));
        hash.insert(String::from("name"), QueryValue::String(clone.name));
        hash.insert(String::from("url"), QueryValue::String(clone.url));

        let fn_type = clone.fn_type.into();
        hash.insert(String::from("fn_type"), QueryValue::String(fn_type));

        let struct_type = StructType::Link.into();
        hash.insert(String::from("struct_type"), QueryValue::String(struct_type));

        hash
    }

    fn from_entities(entites: Vec<Entity>) -> Vec<Self> {
        let mut vec = Vec::new();
        for entity in entites {
            vec.push(Link::from_entity(&entity));
        }

        vec
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.path == other.path &&
        self.url == other.url &&
        self.fn_type == other.fn_type
    }
}
