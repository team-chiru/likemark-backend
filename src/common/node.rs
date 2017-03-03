use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::EntityComposite;
use common::entity::Entity;

use std::collections::BTreeMap;
use common::tree_id::*;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub path: TreeId,
    pub name: String,
    pub url: String,
    pub fn_type: FnType,
    pub links: Vec<Link>,
    pub nodes: Vec<Node>,
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

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        let mut link_eq = true;
        let ref links = self.links;
        let ref other_links = other.links;

        if links.len() == other_links.len() {
            for i in 0..links.len() {
                if links[i] != other_links[i] {
                    link_eq = false;
                    break;
                }
            }
        } else {
            link_eq = false;
        }

        let mut node_eq = true;
        let ref nodes = self.nodes;
        let ref other_nodes = other.nodes;

        if nodes.len() == other_nodes.len() {
            for i in 0..nodes.len() {
                if nodes[i] != other_nodes[i] {
                    node_eq = false;
                    break;
                }
            }
        } else {
            node_eq = false;
        }

        self.id == other.id &&
        self.name == other.name &&
        self.path == other.path &&
        self.url == other.url &&
        self.fn_type == other.fn_type &&
        link_eq && node_eq
    }
}

impl EntityComposite for Node {
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

    fn into_entities(&self) -> Vec<Entity> {
        let clone = self.clone();
        let mut entities = vec!(
            Entity {
                id: clone.id,
                path: clone.path,
                name: clone.name,
                url: clone.url,
                struct_type: StructType::Node,
                fn_type: clone.fn_type,
                rev_no: 0
            }
        );

        for link in clone.links {
            entities.append(&mut link.into_entities());
        }

        for node in clone.nodes {
            entities.append(&mut node.into_entities());
        }

        entities
    }

    fn from_entities(entities: Vec<Entity>) -> Vec<Self> {
        let mut node_map: BTreeMap<String, Node> = BTreeMap::new();
        let mut lower = usize::max_value();
        let mut pre_links: Vec<Entity> = Vec::new();

        for e in entities {
            let ref path = e.path;
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
