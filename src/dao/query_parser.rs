extern crate regex;

use std::collections::BTreeMap;
use std::collections::HashMap;
use common::utils::QueryValue;

pub trait QueryParser {
    fn fill_query(&self, query: &String) -> String;
}

impl QueryParser for BTreeMap<String, QueryValue> {
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
