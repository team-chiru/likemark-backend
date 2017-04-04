extern crate regex;
extern crate chrono;

use self::chrono::*;
use std::collections::HashMap;
use common::Entity;

pub enum QueryValue {
    Null,
    Integer(i32),
    String(String),
    Date(DateTime<Local>),
}

impl<'a> Into<String> for &'a QueryValue {
    fn into(self) -> String {
        match self {
            &QueryValue::Null => String::from("NULL"),
            &QueryValue::Integer(i) => format!("{}", i),
            &QueryValue::String(ref s) => format!(r"'{}'", s),
            &QueryValue::Date(t) => t.to_rfc2822(),
        }
    }
}

pub trait QueryParser {
    fn fill_query(&self, query: &String) -> String;
}

impl QueryParser for HashMap<String, QueryValue> {
    fn fill_query(&self, query: &String) -> String {
        let re = regex::Regex::new(r"\{\{ +(\w+) +\}\}").unwrap();
        let mut replaced_query = String::from(query.clone());

        for capture in re.captures_iter(query.as_str()) {
            let query_tag = &capture[0];
            let query_key = &capture[1];

            let value = match self.get(query_key) {
                Some(v) => v.clone(),
                None => panic!("parsing error"),
            };

            let dump: String = value.into();

            replaced_query = replaced_query.replace(query_tag, dump.as_str());
        }

        replaced_query
    }
}

pub trait QueryMap {
    fn map_query(&self) -> HashMap<String, QueryValue>;
}

impl QueryMap for Entity {
    fn map_query(&self) -> HashMap<String, QueryValue> {
        let mut hash: HashMap<String, QueryValue> = HashMap::new();

        if let Some(id) = self.id {
            hash.insert(String::from("id"), QueryValue::Integer(id));
        }

        if let Some(ref path) = self.path {
            hash.insert(String::from("path"), QueryValue::String(path.id()));
        }

        if let Some(name) = self.name.clone() {
            hash.insert(String::from("name"), QueryValue::String(name));
        }

        if let Some(url) = self.url.clone() {
            hash.insert(String::from("url"), QueryValue::String(url));
        }

        if let Some(ref fn_type) = self.fn_type {
            let fn_type_str = fn_type.into();
            hash.insert(String::from("fn_type"), QueryValue::String(fn_type_str));
        }

        if let Some(ref struct_type) = self.struct_type {
            let struct_type_str = struct_type.into();
            hash.insert(String::from("struct_type"), QueryValue::String(struct_type_str));
        }

        hash
    }
}
