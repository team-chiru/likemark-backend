use common::types::FnType;
use common::types::StructType;

use common::entity::FromEntity;
use common::entity::Entity;

use common::criteria::CriteriaBuilder;
use common::criteria::Criteria;

#[derive(Debug, Clone)]
pub struct Link {
    id: i32,
    name: String,
    url: String,
    fn_type: FnType,
}

impl FromEntity for Link {
    fn from_entity(entity: &Entity) -> Self {
        let clone = entity.clone();

        Link {
            id: clone.id,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
        }
    }

    fn from_entities(entites: Vec<Entity>) -> Vec<Self> {
        let mut vec = Vec::new();
        for entity in entites {
            vec.push(Link::from_entity(&entity));
        }

        vec
    }
}

impl CriteriaBuilder for Link {
    fn criteria(&self) -> Criteria {
        let struct_type = StructType::Link;
        Criteria::new().struct_type(struct_type.into()).build()
    }
}
