extern crate regex;

use common::node::Node;

use core::external::base::Converter;
use self::regex::Regex;

pub struct Netscape {}

impl Netscape {
    pub fn sanitize(bookmark_string: String) -> String {

        let mut sanitized = bookmark_string;
        sanitized = sanitized.replace("\t"," ");
        sanitized = sanitized.replace("\r"," ");

        let mut bookmark_str: String = String::from(&sanitized[..]);

        let set = vec![
            r"(?i)(<!DOCTYPE|<META|<TITLE|<H1|<P).*\s?",
            r"(?mis)<!--.*?-->\s?",
            r"(?mis)\s?<.?br>",
        ];

        for regex in set {
            let re = Regex::new(regex).unwrap();
            let result = re.replace_all(&bookmark_str, "").to_string();
            bookmark_str = result;
        }

        let re = Regex::new(r"(?mis)>(\s*?)<").unwrap();
        bookmark_str = re.replace_all(&bookmark_str, ">\n<").to_string();

        sanitized = bookmark_str;
        sanitized.trim();
        sanitized
    }


}

impl Converter for Netscape {
    fn parse(bookmark_string: String) -> Vec<Node> {
        let bookmark_string = Netscape::sanitize(bookmark_string);
        println!("{}",bookmark_string);
        //bookmark_string.split("\n").collect();

        vec![]
    }

    fn build(nodes: Vec<Node>) -> String {
        format!("{:?}", nodes)
    }
}
