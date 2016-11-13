#[macro_export]
macro_rules! dump_file {
    ($path:expr) => {{
        let absolute_path = std::path::PathBuf::from($path);
        let mut file = match std::fs::canonicalize(&absolute_path) {
            Ok(p) => match std::fs::File::open(p) {
               Ok(f) => f,
               Err(_) => panic!("Unable to open file: {}", $path)
           },
            Err(_) => panic!("Unable to find file: {}", $path)
        };

        let mut dump = String::new();

        use std::io::Read;
        match file.read_to_string(&mut dump) {
            Ok(_) => println!("File {} has been read with success !", $path),
            Err(_) => panic!("Unable to parse ressources!")
        }

        dump
    }};
}
