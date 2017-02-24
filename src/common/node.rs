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
    tree_id: TreeId,
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

    fn nodes_mut(&mut self) -> &mut Vec<Node> {
        &mut self.nodes
    }
}

impl FromEntity for Node {
    fn from_entity(entity: &Entity) -> Self {
        let clone = entity.clone();

        Node {
            id: clone.id,
            tree_id: clone.tree_id,
            name: clone.name,
            url: clone.url,
            fn_type: clone.fn_type,
            links: Vec::new(),
            nodes: Vec::new()
        }
    }

    //TODO entity matcher for nodes
    fn from_entities(entities: Vec<Entity>) -> Vec<Self> {
        println!("{:?}\n", entities);

        let mut node_map: BTreeMap<String, Node> = BTreeMap::new();
        let mut lower = usize::max_value();
        let mut pre_links: Vec<Entity> = Vec::new();

        for e in entities {
            let ref path = e.tree_id;

            if e.struct_type == StructType::Node {
                if path.level() < lower {
                    lower = path.level();
                }

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
        for (id, node) in node_map {
            push_node(&mut roots, lower, node);
        }

        roots
    }
}

fn push_node<'a>(roots: &mut Vec<Node>, base: usize, node: Node) {
    let mut search = base;
    let ref path = node.tree_id;

    if path.level() == base {
        roots.push(node.clone());
    } else {
        for root in roots {
            if let Some(parent) = path.key(base) {
                if parent == root.tree_id.id() {
                    push_node(&mut root.nodes, base + 1, node.clone());
                    break;
                }
            }
        }
    }
}
