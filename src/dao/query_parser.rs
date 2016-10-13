use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;

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
