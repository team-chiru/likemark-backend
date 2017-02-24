use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::FromEntity;
use common::entity::Entity;

use std::collections::BTreeMap;
use common::tree_id::*;

#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    name: String,
    url: String,
    fn_type: FnType,
    links: Vec<Link>,
    nodes: Vec<Node>,
}

impl Node {
    fn push(&mut self, entity: &Entity) {
        match entity.struct_type {
            StructType::Link => {
                self.links.push(Link::from_entity(&entity));
            },
            StructType::Node => {
                self.nodes.push(Node::from_entity(&entity));
            }
        }
    }
}

impl FromEntity for Node {
    fn from_entity(entity: &Entity) -> Self {
        let clone = entity.clone();

        Node {
            id: clone.id,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
            links: Vec::new(),
            nodes: Vec::new()
        }
    }

    //TODO entity matcher for nodes
    fn from_entities(entities: Vec<Entity>) -> Vec<Self> {
        let mut node_map: BTreeMap<String, Node> = BTreeMap::new();
        let mut lower = usize::max_value();

        for e in entities.iter() {
            let ref path = e.tree_id;

            if e.struct_type == StructType::Node {
                if path.level() < lower {
                    lower = path.level();
                }

                node_map.insert(path.id(), Node::from_entity(e));
            }
        }

        for e in entities.iter() {
            let inner = e.tree_id.level();

            let parent = match e.tree_id.key(inner - 1) {
                Some(p) => p,
                None => panic!("{:?}", e),
            };

            if let Some(node) = node_map.get_mut(&parent) {
                node.push(e);
            }
        }

        let mut roots = Vec::new();
        println!("{:?}\n", node_map);
        for (key, node) in node_map.into_iter() {
            if level(&key) == lower {
                roots.push(node);
            }
        }

        roots
    }
}
