use common::types::FnType;
use common::types::StructType;
use common::link::*;
use common::utils::*;

use common::entity::EntityComposite;
use common::entity::Entity;

use std::collections::BTreeMap;
use std::collections::HashMap;
use common::tree_id::*;

#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    path: TreeId,
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

impl EntityComposite for Node {
    fn from_entity(entity: &Entity) -> Self {
        let clone = entity.clone();

        Node {
            id: clone.id,
            path: clone.tree_id,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
            links: Vec::new(),
            nodes: Vec::new()
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

        let struct_type = StructType::Node.into();
        hash.insert(String::from("struct_type"), QueryValue::String(struct_type));

        hash
    }

    fn from_entities(entities: Vec<Entity>) -> Vec<Self> {
        let mut node_map: BTreeMap<String, Node> = BTreeMap::new();
        let mut lower = usize::max_value();
        let mut pre_links: Vec<Entity> = Vec::new();

        for e in entities {
            let ref path = e.tree_id;
            if e.struct_type == StructType::Node {
                if level(path) < lower {
                    lower = level(path);
                }

                node_map.insert(path.id(), Node::from_entity(&e));
            } else {
                pre_links.push(e.clone());
            }
        }

        for e in pre_links {
            let ref path = e.tree_id;
            let parent = match key(&path, level(&path) - 1) {
                Some(p) => p,
                None => panic!("{:?}", e),
            };

            if let Some(node) = node_map.get_mut(&parent) {
                node.push(&e);
            }
        }

        let mut roots = Vec::new();
        for (_, node) in node_map {
            push_node(&mut roots, lower, node);
        }

        roots
    }
}

fn push_node(roots: &mut Vec<Node>, search_level: usize, node: Node) {
    let ref path = node.clone().path;

    if level(path) == search_level {
        roots.push(node);
    } else {
        for root in roots {
            let parent_id = match key(path, search_level) {
                Some(id) => id,
                None => panic!("{:?}", path)
            };

            let root_key = &root.path;
            if parent_id == root_key.id() {
                push_node(&mut root.nodes, search_level + 1, node);
                break;
            }
        }
    }
}
