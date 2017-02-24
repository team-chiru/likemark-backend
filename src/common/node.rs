use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::FromEntity;
use common::entity::Entity;

use std::collections::HashMap;
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

    fn from_entities(entities: Vec<Entity>) -> Vec<Self> {
        let mut node_map: HashMap<String, Node> = HashMap::new();
        let mut node_dir: HashMap<i32, TreeId> = HashMap::new();
        let mut lower = usize::max_value();
        let mut pre_links: Vec<Entity> = Vec::new();

        for e in entities {
            let ref path = e.tree_id;

            if e.struct_type == StructType::Node {
                if path.level() < lower {
                    lower = path.level();
                }

                node_dir.insert(e.id, path.clone());
                node_map.insert(path.id(), Node::from_entity(&e));
            } else {
                pre_links.push(e.clone());
            }
        }

        for e in pre_links {
            let inner = e.tree_id.level();

            let parent = match e.tree_id.key(inner - 1) {
                Some(p) => p,
                None => panic!("{:?}", e),
            };

            if let Some(node) = node_map.get_mut(&parent) {
                node.push(&e);
            }
        }

        let mut roots = Vec::new();
        for (_, node) in node_map {
            push_node(&node_dir, &mut roots, lower, node);
        }

        roots
    }
}

fn push_node<'a>(dir: &HashMap<i32, TreeId>, roots: &mut Vec<Node>, search: usize, node: Node) {
    let path = match dir.get(&node.id) {
        Some(tree_id) => tree_id,
        None => panic!("{:?}", node)
    };

    if path.level() == search {
        roots.push(node.clone());
    } else {
        for root in roots {
            let parent = match path.key(search) {
                Some(key) => key,
                None => panic!("{:?}", path)
            };

            let root_key = match dir.get(&root.id) {
                Some(key) => key,
                None => panic!("{:?}", root)
            };

            if parent == root_key.id() {
                push_node(dir, &mut root.nodes, search + 1, node.clone());
                break;
            }
        }
    }
}
