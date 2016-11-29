extern crate yaml_rust;

use std::collections::BTreeMap;
use common::utils::QueryValue;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: i32,
    //pub node_id: i32,
    pub name: String,
    pub url: String,
    pub rev_no: i32
}

impl Link {
    pub fn to_yaml(self) -> String {
        let mut btree = BTreeMap::new();

        btree.insert("id", QueryValue::Integer(self.id));
        btree.insert("name", QueryValue::String(self.name));
        btree.insert("url", QueryValue::String(self.url));
        btree.insert("rev_no", QueryValue::Integer(self.rev_no));

        let mut yaml = String::new();
        yaml.push_str("Link: \n");

        for (k, v) in btree {
            yaml.push_str("\t");
            yaml.push_str(k);
            yaml.push_str(": ");

            let s: String = (&v).into();
            yaml.push_str(s.as_str());
            yaml.push_str("\n");
        }

        yaml
    }

    pub fn to_btree(self) -> BTreeMap<String, QueryValue> {
        let mut btree : BTreeMap<String, QueryValue> = BTreeMap::new();

        btree.insert(String::from("id"), QueryValue::Integer(self.id));
        btree.insert(String::from("name"), QueryValue::String(self.name));
        btree.insert(String::from("url"), QueryValue::String(self.url));
        btree.insert(String::from("rev_no"), QueryValue::Integer(self.rev_no));

        btree
    }
}

impl PartialEq for Link {
    fn eq(&self, b: &Link) -> bool {
        if self.id != b.id {
            return false;
        }

        if self.name != b.name {
            return false;
        }

        if self.url != b.url {
            return false;
        }

        if self.rev_no != b.rev_no {
            return false;
        }

        return true;
    }
}
