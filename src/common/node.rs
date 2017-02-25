use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::FromEntity;
use common::entity::Entity;

use std::collections::HashMap;
use std::collections::BTreeMap;
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

impl FromEntity for Node {
    fn from_entity(entity: &Entity) -> Self {
        let clone = entity.clone();

        Node {
            id: clone.id,
            path: clone.path,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
            links: Vec::new(),
            nodes: Vec::new()
        }
    }

    fn from_entities(entities: Vec<Entity>) -> Vec<Self> {
        let mut node_map: BTreeMap<String, Node> = BTreeMap::new();
        let mut node_dir: HashMap<i32, TreeId> = HashMap::new();
        let mut lower = usize::max_value();
        let mut pre_links: Vec<Entity> = Vec::new();

        for e in entities {
            let ref path = e.path;
            if e.struct_type == StructType::Node {
                if level(path) < lower {
                    lower = level(path);
                }

                node_dir.insert(e.id, path.clone());
                node_map.insert(path.id(), Node::from_entity(&e));
            } else {
                pre_links.push(e.clone());
            }
        }

        for e in pre_links {
            let ref path = e.path;
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
            push_node(&node_dir, &mut roots, lower, node);
        }

        println!("{:?}\n", roots);
        roots
    }
}

fn push_node(dir: &HashMap<i32, TreeId>, roots: &mut Vec<Node>, search_level: usize, node: Node) {
    let path = match dir.get(&node.id) {
        Some(path) => path,
        None => panic!("{:?}", node)
    };

    if level(path) == search_level {
        roots.push(node);
    } else {
        for root in roots {
            let parent_id = match key(path, search_level) {
                Some(id) => id,
                None => panic!("{:?}", path)
            };

            let root_key = match dir.get(&root.id) {
                Some(key) => key,
                None => panic!("{:?}", root)
            };

            println!("{:?} - {:?}\n", parent_id, root_key.id());
            if parent_id == root_key.id() {
                push_node(dir, &mut root.nodes, search_level + 1, node);
                break;
            }
        }
    }
}
