use common::types::FnType;
use common::types::StructType;
use common::TreePath;

use common::entity::Entity;

use super::Composite;

use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: Uuid,
    pub path: TreePath,
    pub name: String,
    pub url: String,
    pub fn_type: FnType,
}

impl Composite<Entity> for Link {
    fn is_empty(&self) -> bool {
        if self.id == Uuid::nil() {
            true
        } else {
            false
        }
    }

    fn compose(e: &Entity) -> Self {
        let clone = e.clone();
        let empty = || String::from("");

        Link {
            id: match clone.uuid {
                Some(s) => match Uuid::from_str(&s) {
                    Ok(u) => u,
                    Err(msg) => panic!(msg)
                },
                None => Uuid::nil()
            },
            path: clone.path.unwrap_or(TreePath::new(empty())),
            name: clone.name.unwrap_or(empty()),
            url: clone.url.unwrap_or(empty()),
            fn_type: clone.fn_type.unwrap_or(FnType::None),
        }
    }

    fn compose_vec(entites: Vec<Entity>) -> Vec<Self> {
        let mut vec = Vec::new();
        for e in entites {
            if e.struct_type == Some(StructType::Link) {
                vec.push(Link::compose(&e));
            }
        }

        vec
    }

    fn decompose(&self) -> Vec<Entity> {
        let clone = self.clone();
        let uuid = match clone.id == Uuid::nil() {
            true => Uuid::new_v4(),
            false => clone.id
        };

        vec!(
            Entity {
                id: None,
                uuid: Some(uuid.hyphenated().to_string()),
                path: Some(clone.path),
                name: Some(clone.name),
                url: Some(clone.url),
                struct_type: Some(StructType::Link),
                fn_type: Some(clone.fn_type),
                rev_no: None
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
