extern crate regex;

use std::collections::BTreeMap;
use common::utils::QueryValue;
use self::regex::Regex;

pub fn parse_query(btree: &BTreeMap<String, QueryValue>, query: String) -> String {
    let re = Regex::new(r"\{\{ +(\w+) +\}\}").unwrap();
    let mut replaced_query = String::from(query.clone());

    for capture in re.captures_iter(query.as_str()) {
        let query_tag = capture.at(0).unwrap_or("");
        let query_key = capture.at(1).unwrap_or("");

        let value = match btree.get(query_key) {
            Some(v) => v.clone(),
            None => panic!("parsing error"),
        };

        let dump: String = value.into();

        replaced_query = replaced_query.replace(query_tag, dump.as_str());
    }

    println!("{}", replaced_query);
    replaced_query
}
