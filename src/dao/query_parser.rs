extern crate regex;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::collections::BTreeMap;
//use common::bookmark::Bookmark;
//use common::bookmark::QueryValue;
use common::bookmark::*;
use self::regex::Regex;

pub fn load_sql_file(path: &str) -> String {
    let mut absolute_path = PathBuf::from(path);

    absolute_path = match fs::canonicalize(&absolute_path) {
        Ok(p) => p,
        Err(_) => panic!("Unable to find file!")
    };

    let mut file = File::open(absolute_path).unwrap();

    let mut sql = String::new();
    file.read_to_string(&mut sql)
        .expect("Unable to parse ressources!");

    sql
}


pub fn parse_query(btree: &BTreeMap<String, QueryValue>, query: String) -> String {
    let re = Regex::new(r"\{\{ +(\w+) +\}\}").unwrap();
    let mut replaced_query = String::from(query.clone());

    for capture in re.captures_iter(query.as_str()){

        let query_tag = capture.at(0).unwrap_or("");
        let query_key = capture.at(1).unwrap_or("");

        let value = match btree.get(query_key) {
            Some(v) => v.clone(),
            None => panic!("error")
        };

        let dump: String = value.into();
        replaced_query = replaced_query.replace(query_tag, dump.as_str());
    }

    replaced_query
}
