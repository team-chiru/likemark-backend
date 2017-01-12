use std::collections::BTreeMap;
use common::utils::QueryValue;

use common::types::StructType;
use common::types::FnType;

pub trait Entity {
    fn id(&self) -> i32 {
        -1
    }

    fn parent_id(&self) -> String {
        String::from("0")
    }

    fn name(&self) -> String {
        String::from("")
    }

    fn url(&self) -> String {
        String::from("")
    }

    fn struct_type(&self) -> StructType {
        StructType::Link
    }

    fn fn_type(&self) -> FnType {
        FnType::None
    }

    fn rev_no(&self) -> i32 {
        0
    }

    fn to_btree(&self) -> BTreeMap<String, QueryValue> {
        let mut btree: BTreeMap<String, QueryValue> = BTreeMap::new();

        btree.insert(String::from("id"), QueryValue::Integer(self.id()));
        btree.insert(String::from("parent_id"),
                     QueryValue::String(self.parent_id()));
        btree.insert(String::from("name"), QueryValue::String(self.name()));
        btree.insert(String::from("url"), QueryValue::String(self.url()));
        btree.insert(String::from("struct_type"),
                     QueryValue::String(self.struct_type().into()));
        btree.insert(String::from("fn_type"),
                     QueryValue::String(self.fn_type().into()));
        btree.insert(String::from("rev_no"), QueryValue::Integer(self.rev_no()));

        btree
    }

    fn to_yaml(&self) -> String {
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
}

impl PartialEq for Entity {
    fn eq(&self, e: &Self) -> bool {
        self.id() == e.id() ||
        (self.parent_id() == e.parent_id() && self.name() == e.name() && self.url() == e.url() &&
         self.struct_type() == e.struct_type() &&
         self.fn_type() == e.fn_type() && self.rev_no() == e.rev_no())
    }
}
