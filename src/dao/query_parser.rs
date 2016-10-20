use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::collections::BTreeMap;
use common::bookmark::Bookmark;
use common::bookmark::QueryValue;

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


pub fn parse_query(btree: BTreeMap<String, QueryValue>, query: String) -> String {

    let mut replaced_query = query;

    for (k, v) in btree {
        let pattern = format!("{{ {:} }}", k);
        let dump: String = v.into();
        replaced_query.replace(pattern, dump.as_str());
    }

    replaced_query
}
