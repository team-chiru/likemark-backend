use common::utils::QueryValue;
use std::collections::BTreeMap;

use common::tree_id::*;
use common::types::FnType;
use common::types::StructType;

#[derive(Debug, Clone)]
pub struct Criteria {
    id: Option<i32>,
    tree_id: Option<TreeId>,
    name: Option<String>,
    url: Option<String>,
    struct_type: Option<StructType>,
    fn_type: Option<FnType>,
    rev_no: Option<i32>
}

pub trait CriteriaBuilder {
    fn criteria(&self) -> Criteria;
}

impl Criteria {
    pub fn new() -> Criteria {
        Criteria {
            id: None,
            tree_id: None,
            name: None,
            url: None,
            struct_type: None,
            fn_type: None,
            rev_no: None,
        }
    }

    pub fn id(&mut self, id: i32) -> &mut Criteria {
        self.id = Some(id);
        self
    }

    pub fn tree_id(&mut self, tree_id: &TreeId) -> &mut Criteria {
        self.tree_id = Some(TreeId::new(tree_id.value()));
        self
    }

    pub fn name(&mut self, name: String) -> &mut Criteria {
        self.name = Some(name);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Criteria {
        self.url = Some(url);
        self
    }

    pub fn struct_type(&mut self, struct_type: StructType) -> &mut Criteria {
        self.struct_type = Some(struct_type);
        self
    }

    pub fn fn_type(&mut self, fn_type: FnType) -> &mut Criteria {
        self.fn_type = Some(fn_type);
        self
    }

    pub fn rev_no(&mut self, rev_no: i32) -> &mut Criteria {
        self.rev_no = Some(rev_no);
        self
    }

    pub fn build(&mut self) -> Criteria {
        self.clone()
    }

    pub fn map(&self) -> BTreeMap<String, QueryValue> {
        let mut btree: BTreeMap<String, QueryValue> = BTreeMap::new();

        match self.id {
            Some(id) => {
                btree.insert(String::from("id"), QueryValue::Integer(id));
            }
            None => {
                btree.insert(String::from("id"), QueryValue::Null);
            }
        }

        match self.tree_id.clone() {
            Some(tree_id) => {
                btree.insert(String::from("tree_id"), QueryValue::String(tree_id.value()));
            }
            None => {
                btree.insert(String::from("tree_id"), QueryValue::Null);
            }
        }

        match self.name.clone() {
            Some(name) => {
                btree.insert(String::from("name"), QueryValue::String(name));
            }
            None => {
                btree.insert(String::from("name"), QueryValue::Null);
            }
        }

        match self.url.clone() {
            Some(url) => {
                btree.insert(String::from("url"), QueryValue::String(url));
            }
            None => {
                btree.insert(String::from("url"), QueryValue::Null);
            }
        }

        match self.struct_type.clone() {
            Some(struct_type) => {
                let value = struct_type.into();
                btree.insert(String::from("struct_type"), QueryValue::String(value));
            }
            None => {
                btree.insert(String::from("struct_type"), QueryValue::Null);
            }
        }

        match self.fn_type.clone() {
            Some(fn_type) => {
                let value = fn_type.into();
                btree.insert(String::from("fn_type"), QueryValue::String(value));
            }
            None => {
                btree.insert(String::from("fn_type"), QueryValue::Null);
            }
        }

        match self.rev_no {
            Some(rev_no) => {
                btree.insert(String::from("rev_no"), QueryValue::Integer(rev_no));
            }
            None => {
                btree.insert(String::from("rev_no"), QueryValue::Null);
            }
        }

        btree
    }
}
