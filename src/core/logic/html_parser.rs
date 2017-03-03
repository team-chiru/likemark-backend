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
        let filepath = &self.bookmark_file_path;
        let f = load_file(&filepath);



    }

    /*
    pub fn export() -> Result<File,Err>{

    } */


}
