extern crate chrono;
use self::chrono::*;

pub fn dump_file(path: &str) -> String {
    use std::path::PathBuf;
    let absolute_path = PathBuf::from(path);

    use std::fs::*;
    let mut file = match canonicalize(&absolute_path) {
        Ok(p) => match File::open(p) {
           Ok(f) => f,
           Err(_) => panic!("Unable to open file: {}", path)
       },
        Err(_) => panic!("Unable to find file: {}", path)
    };

    let mut dump = String::new();

    use std::io::Read;
    match file.read_to_string(&mut dump) {
        Ok(_) => println!("File {} has been read with success !", path),
        Err(_) => panic!("Unable to parse ressources!")
    }

    dump
}

pub enum QueryValue {
    Integer(i32),
    String(String),
    Date(DateTime<Local>),
    Null
}

impl<'a> Into<String> for &'a QueryValue {
    fn into(self) -> String {
        match self {
            &QueryValue::Integer(i) => format!("{}", i),
            &QueryValue::String(ref s) => format!("\"{}\"", s),
            &QueryValue::Date(t) => t.to_rfc2822(),
            &QueryValue::Null => String::from("NULL")
        }
    }
}
