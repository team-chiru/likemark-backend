use common::utils::QueryValue;
use std::collections::BTreeMap;

pub struct LinkCriteria {
    pub id: Option<i32>,
    name: Option<String>,
    url: Option<String>,
    rev_no: Option<i32>
}

impl<'a> LinkCriteria {
    pub fn new() -> LinkCriteria {
        LinkCriteria {
            id: None,
            name: None,
            url: None,
            rev_no: None
        }
    }

    pub fn id(mut self, id: i32) -> LinkCriteria {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> LinkCriteria {
        self.name = Some(name);
        self
    }

    pub fn url(mut self, url: String) -> LinkCriteria {
        self.url = Some(url);
        self
    }

    pub fn rev_no(mut self, rev_no: i32) -> LinkCriteria {
        self.rev_no = Some(rev_no);
        self
    }

    pub fn to_btree(&self) -> BTreeMap<String, QueryValue> {
        let mut btree : BTreeMap<String, QueryValue> = BTreeMap::new();

        match self.id {
            Some(id) => {
                btree.insert(String::from("id"), QueryValue::Integer(id));
            },
            None => {}
        };

        match self.name.clone() {
            Some(name) => {
                btree.insert(String::from("name"), QueryValue::String(name));
            },
            None => {
                btree.insert(String::from("name"), QueryValue::Null);
            }
        };

        match self.url.clone() {
            Some(url) => {
                btree.insert(String::from("url"), QueryValue::String(url));
            },
            None => {
                btree.insert(String::from("url"), QueryValue::Null);
            }
        };

        match self.rev_no {
            Some(rev_no) => {
                btree.insert(String::from("rev_no"), QueryValue::Integer(rev_no));
            },
            None => {}
        };

        btree
    }
}
