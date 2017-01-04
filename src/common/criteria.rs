use common::utils::QueryValue;
use std::collections::BTreeMap;

pub struct Criteria {
    pub id: Option<i32>,
    pub parent_id: Option<i32>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub struct_type: Option<String>,
    pub fn_type: Option<String>,
    pub rev_no: Option<i32>,
}

impl Criteria {
    pub fn new() -> Criteria {
        Criteria {
            id: None,
            parent_id: None,
            name: None,
            url: None,
            struct_type: None,
            fn_type: None,
            rev_no: None,
        }
    }

    pub fn id(mut self, id: i32) -> Criteria {
        self.id = Some(id);
        self
    }

    pub fn parent_id(mut self, parent_id: i32) -> Criteria {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn name(mut self, name: String) -> Criteria {
        self.name = Some(name);
        self
    }

    pub fn url(mut self, url: String) -> Criteria {
        self.url = Some(url);
        self
    }

    pub fn struct_type(mut self, struct_type: String) -> Criteria {
        self.struct_type = Some(struct_type);
        self
    }

    pub fn fn_type(mut self, fn_type: String) -> Criteria {
        self.fn_type = Some(fn_type);
        self
    }

    pub fn rev_no(mut self, rev_no: i32) -> Criteria {
        self.rev_no = Some(rev_no);
        self
    }

    pub fn to_btree(&self) -> BTreeMap<String, QueryValue> {
        let mut btree: BTreeMap<String, QueryValue> = BTreeMap::new();

        match self.id {
            Some(id) => {
                btree.insert(String::from("id"), QueryValue::Integer(id));
            }
            None => {}
        }

        match self.parent_id {
            Some(parent_id) => {
                btree.insert(String::from("parent_id"), QueryValue::Integer(parent_id));
            }
            None => {}
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
                btree.insert(String::from("struct_type"), QueryValue::String(struct_type));
            }
            None => {
                btree.insert(String::from("struct_type"), QueryValue::Null);
            }
        }

        match self.fn_type.clone() {
            Some(fn_type) => {
                btree.insert(String::from("fn_type"), QueryValue::String(fn_type));
            }
            None => {
                btree.insert(String::from("fn_type"), QueryValue::Null);
            }
        }

        match self.rev_no {
            Some(rev_no) => {
                btree.insert(String::from("rev_no"), QueryValue::Integer(rev_no));
            }
            None => {}
        }

        btree
    }
}
