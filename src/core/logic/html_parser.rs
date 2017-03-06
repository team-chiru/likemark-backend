extern crate serde_json;
use self::serde_json::*;
use common::utils::load_file;
use std::str;
use std::fs::File;
use std::io::BufReader;





#[derive(Debug, Clone)]
pub struct Parser{
    bookmark_file_path: String
}

impl Parser {
    pub fn new(bookmark_file_path: String) -> Parser{
        Parser {
            bookmark_file_path: bookmark_file_path
        }
    }

    pub fn import(&mut self){

        let file_path = &self.bookmark_file_path;
        let f = load_file(&file_path);
        let s_temp = f.unwrap();
        let file_content: &str = &s_temp[..];

        //print!("{:?}", file_content);

        let json: Value = serde_json::from_str(file_content).unwrap_or_else(|e| {
            panic!("Failed to parse json; error is {}", e);
        });

        let links = json.as_object()
            .and_then(|object| object.get("_links"))
            .and_then(|links| links.as_object())
            .unwrap_or_else(|| {
                panic!("Failed to get '_links' value from json");
        });

        //println!("{:?}", links);

        //let mut html_buffer = String::new();
        //try!(f.read_to_string(&mut html_buffer));

        //Chercher les liens et les mettres dans un tableau (btree etc...)
        //Les insert dans la base de donnees


    }

    /*
    pub fn export() -> Result<File,Err>{

    } */


}
