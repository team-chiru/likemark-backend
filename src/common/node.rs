use common::bookmark::Link;
use std::collections::BTreeMap;
use common::utils::QueryValue;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub links: Vec<Link>,
    pub node_type: String,
    pub node: Option<Vec<Box<Node>>>
}

impl Node {
    pub fn hello(self) {
        println!("coucou je suis un node");
    }

    // NODE IMPLEMENTATION
}

impl Node {
    pub fn to_btree(self) -> BTreeMap<String, QueryValue> {
        let mut btree : BTreeMap<String, QueryValue> = BTreeMap::new();
        let mut links_ids = Vec::new();

        for link in self.links {
            links_ids.push(link.id);
        }

        match self.node {
            Some(nodes) => {
                // SHITS
            },
            None => {

            }
        };

        btree
    }
}

impl PartialEq for Node {
    fn eq(&self, n: &Node) -> bool {
        if self.links != n.links {
            return false;
        }

        if self.node != n.node {
            return false;
        }

        return true;
    }
}
