use common::utils::load_file;
use common::entity;
use std::fs::File;

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

        println!("{:?}", f);

        //let mut html_buffer = String::new();
        //try!(f.read_to_string(&mut html_buffer));

        //Chercher les liens et les mettres dans un tableau (btree etc...)
        //Les insert dans la base de donnees


    }

    /*
    pub fn export() -> Result<File,Err>{

    } */


}
