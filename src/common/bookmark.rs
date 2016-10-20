extern crate chrono;
extern crate yaml_rust;

use self::chrono::*;
use self::yaml_rust::Yaml;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Bookmark {
    pub id: i32,
    pub name: String,
    //time_created: Timespec,
    pub url: String,
    pub stamp: DateTime<Local>,
    pub rev_no: i32
}

enum QueryValue {
    Integer(i32),
    String(String),
    Date(DateTime<Local>)
}

impl<'a> Into<String> for QueryValue {
    fn into(self) -> String {
        match self {
            QueryValue::Integer(i) => format!("{}", i),
            QueryValue::String(s) => s,
            QueryValue::Date(t) => t.to_rfc2822()
        }
    }
}

impl Bookmark {
    pub fn to_yaml(self) -> String {
        let mut btree = BTreeMap::new();

        btree.insert("id", QueryValue::Integer(self.id));
        btree.insert("name", QueryValue::String(self.name));
        //btree.insert("time_created", QueryValue::Date(self.time_created));
        btree.insert("url", QueryValue::String(self.url));
        btree.insert("stamp", QueryValue::Date(self.stamp));
        btree.insert("rev_no", QueryValue::Integer(self.rev_no));

        let mut yaml = String::new();
        yaml.push_str("Bookmark: \n");

        for (k, v) in btree {
            yaml.push_str("\t");
            yaml.push_str(k);
            yaml.push_str(": ");

            let s: String = v.into();
            yaml.push_str(s.as_str());
            yaml.push_str("\n");
        }

        yaml
    }
}
