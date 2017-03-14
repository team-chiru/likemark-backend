extern crate regex;

use common::node::Node;
use core::external::converter::Converter;
use self::regex::Regex;

pub struct Netscape {}

impl Netscape {
    pub fn sanitize(bookmark_string: String) -> String {

        let mut sanitized = bookmark_string;
        sanitized = sanitized.replace("\t"," ");
        sanitized = sanitized.replace("\r"," ");
        /*let re1 = Regex::new(r"@<!--.*-->@mis").unwrap();
        sanitized = re1.replace_all(&sanitized, "");*/
        sanitized.trim();

        sanitized
    }
}

impl Converter for Netscape {

    fn parse(bookmark_string: String) -> Vec<Node> {
        let bookmark_string = Netscape::sanitize(bookmark_string);
        let lines: Vec<&str> = bookmark_string.split("\n").collect();
        println!("{:?}",lines);
        vec![]
    }

    fn build(nodeVec: Vec<Node>) -> String {
        unimplemented!();
    }


}
