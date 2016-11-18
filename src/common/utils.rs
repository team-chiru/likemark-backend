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
