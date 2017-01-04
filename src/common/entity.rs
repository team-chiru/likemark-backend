use std::collections::BTreeMap;
use common::utils::QueryValue;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub url: String,
    pub struct_type: String,
    pub fn_type: String,
    pub rev_no: i32,
}

impl Entity {
    pub fn to_yaml(self) -> String {
        let btree = self.to_btree();

        let mut yaml = String::new();
        yaml.push_str("Link: \n");

        for (k, v) in btree {
            yaml.push_str("\t");
            yaml.push_str(&k);
            yaml.push_str(": ");

            let s: String = (&v).into();
            yaml.push_str(s.as_str());
            yaml.push_str("\n");
        }

        yaml
    }

    pub fn to_btree(self) -> BTreeMap<String, QueryValue> {
        let mut btree: BTreeMap<String, QueryValue> = BTreeMap::new();

        btree.insert(String::from("id"), QueryValue::Integer(self.id));
        btree.insert(String::from("parent_id"), QueryValue::Integer(self.id));
        btree.insert(String::from("name"), QueryValue::String(self.name));
        btree.insert(String::from("url"), QueryValue::String(self.url));
        btree.insert(String::from("struct_type"),
                     QueryValue::String(self.struct_type));
        btree.insert(String::from("fn_type"), QueryValue::String(self.fn_type));
        btree.insert(String::from("rev_no"), QueryValue::Integer(self.rev_no));

        btree
    }
}

impl PartialEq for Entity {
    fn eq(&self, e: &Entity) -> bool {
        self.id == e.id && self.parent_id == e.parent_id && self.name == e.name &&
        self.url == e.url && self.struct_type == e.struct_type &&
        self.fn_type == e.fn_type && self.rev_no == e.rev_no
    }
}
