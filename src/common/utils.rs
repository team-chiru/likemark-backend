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
