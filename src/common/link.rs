use common::types::FnType;
use common::types::StructType;
use common::tree_id::TreeId;

use common::entity::EntityComposite;
use common::entity::Entity;
use common::tree_id::TreeId;

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
            path: clone.path,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
        }
    }

    fn from_entities(entites: Vec<Entity>) -> Vec<Self> {
        let mut vec = Vec::new();
        for e in entites {
            if e.struct_type == StructType::Link {
                vec.push(Link::from_entity(&e));
            }
        }

        vec
    }

    fn into_entities(&self) -> Vec<Entity> {
        let clone = self.clone();
        vec!(
            Entity {
                id: clone.id,
                path: clone.path,
                name: clone.name,
                url: clone.url,
                struct_type: StructType::Link,
                fn_type: clone.fn_type,
                rev_no: 0
            }
        )
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