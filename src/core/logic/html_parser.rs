extern crate treexml;
extern crate serde_json;

use common::utils::load_file;
use self::treexml::Document;
use std::str;



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

        //println!("{:?}", f );

        let doc = Document::parse(f.unwrap().as_bytes()).unwrap();
        let root = doc.root.unwrap();

        let link = root.find_child(|tag| tag.name == "DT").unwrap().clone();
        println!("{} [{:?}] = {}", link.name, link.attributes, link.contents.unwrap());

        //Chercher les liens et les mettres dans un tableau (btree etc...)
        //Les insert dans la base de donnees


    }

    /*
    pub fn export() -> Result<File,Err>{

    } */


}
