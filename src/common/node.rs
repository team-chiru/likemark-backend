use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::FromEntity;
use common::entity::Entity;
use std::cmp::Ordering;

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    name: String,
    url: String,
    fn_type: FnType,
    link: Vec<Link>,
    nodes: Vec<Node>,
}

impl Node {
    fn set_children(&self, children: &[Entity]) {

    }
}

impl FromEntity for Node {
    fn from_entity(entity: Entity) -> Self {
        Node {
            id: entity.id,
            name: entity.name,
            url: entity.url,
            fn_type: entity.fn_type,
            link: Vec::new(),
            nodes: Vec::new()
        }
    }

    //TODO entity matcher for nodes
    fn from_entities(mut entites: Vec<Entity>) -> Vec<Self> {
        let mut node_vec: Vec<Node> = Vec::new();
        let mut link_vec: Vec<Link> = Vec::new();

        //let base_length = 2;
        entites.sort_by(|e1, e2| {
            if e1.tree_id.len() < e2.tree_id.len() {
                Ordering::Less
            } else if e1.tree_id.len() > e2.tree_id.len() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        println!("{:?}", node_vec);
        node_vec
    }
}
