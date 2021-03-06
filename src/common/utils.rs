extern crate chrono;

use self::chrono::*;

pub fn load_file(path: &str) -> Result<String, String> {
    use std::path::PathBuf;
    let absolute_path = PathBuf::from(path);

    use std::fs::*;
    let mut file = match canonicalize(&absolute_path) {
        Ok(p) => {
            match File::open(p) {
                Ok(f) => f,
                Err(_) => {
                    return Err(format!("Unable to find file: {}", path));
                }
            }
        }
        Err(_) => {
            return Err(format!("Unable to find file: {}", path));
        }
    };

    let mut dump = String::new();

    use std::io::Read;
    match file.read_to_string(&mut dump) {
        Ok(_) => {
            info!("Dump the file: {} successfully", path);
            Ok(dump)
        }
        Err(_) => Err(format!("Unable to find file: {}", path)),
    }
}

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
