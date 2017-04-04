use common::types::FnType;
use common::types::StructType;

use common::entity::Entity;
use common::TreeId;
use common::deep::tree_id::{ key, level };

use std::collections::BTreeMap;
use super::Link;
use super::Composite;

use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub path: TreeId,
    pub name: String,
    pub url: String,
    pub fn_type: FnType,
    pub links: Vec<Link>,
    pub nodes: Vec<Node>,
}

impl Node {
    fn push_composition(&mut self, e: &Entity) {
        if let Some(ref struct_type) = e.struct_type {
            match *struct_type {
                StructType::Link => {
                    self.links.push(Link::compose(&e));
                },
                StructType::Node => {
                    self.nodes.push(Node::compose(&e));
                }
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

impl Composite<Entity> for Node {
    fn is_empty(&self) -> bool {
        if self.id == Uuid::nil() {
            true
        } else {
            false
        }
    }

    fn compose(entity: &Entity) -> Self {
        let clone = entity.clone();
        let empty = || String::from("");

        Node {
            id: match clone.uuid {
                Some(s) => match Uuid::from_str(&s) {
                    Ok(u) => u,
                    Err(msg) => panic!(msg)
                },
                None => Uuid::nil()
            },
            path: clone.path.unwrap_or(TreeId::new(empty())),
            name: clone.name.unwrap_or(empty()),
            url: clone.url.unwrap_or(empty()),
            fn_type: clone.fn_type.unwrap_or(FnType::None),
            links: Vec::new(),
            nodes: Vec::new()
        }
    }

    fn decompose(&self) -> Vec<Entity> {
        let clone = self.clone();
        let uuid = match clone.id == Uuid::nil() {
            true => Uuid::new_v4(),
            false => clone.id
        };

        let mut entities = vec!(
            Entity {
                id: None,
                uuid: Some(uuid.hyphenated().to_string()),
                path: Some(clone.path),
                name: Some(clone.name),
                url: Some(clone.url),
                struct_type: Some(StructType::Node),
                fn_type: Some(clone.fn_type),
                rev_no: None
            }
        );

        for link in clone.links {
            entities.append(&mut link.decompose());
        }

        for node in clone.nodes {
            entities.append(&mut node.decompose());
        }

        entities
    }

    fn compose_vec(entities: Vec<Entity>) -> Vec<Self> {
        let mut node_map: BTreeMap<String, Node> = BTreeMap::new();
        let mut lower = usize::max_value();
        let mut pre_links: Vec<Entity> = Vec::new();

        for e in entities {
            let ref path = match e.path {
                Some(ref p) => p,
                None => continue
            };

            if e.struct_type == Some(StructType::Node) {
                if level(&path) < lower {
                    lower = level(&path);
                }

                node_map.insert(path.id(), Node::compose(&e));
            } else {
                pre_links.push(e.clone());
            }
        }

        for e in pre_links {
            let ref path = match e.path {
                Some(ref p) => p,
                None => continue
            };

            let parent = match key(&path, level(&path) - 1) {
                Some(p) => p,
                None => panic!("{:?}", e),
            };

            if let Some(node) = node_map.get_mut(&parent) {
                node.push_composition(&e.clone());
            }
        }

        let mut roots = Vec::new();
        for (_, node) in node_map {
            build_node_hierarchy(&mut roots, lower, node);
        }

        roots
    }
}

fn build_node_hierarchy(roots: &mut Vec<Node>, search_level: usize, node: Node) {
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
                build_node_hierarchy(&mut root.nodes, search_level + 1, node);
                break;
            }
        }
    }
}
