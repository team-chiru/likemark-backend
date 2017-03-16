extern crate regex;

use common::node::Node;

use core::external::base::Converter;
use self::regex::Regex;
use self::regex::RegexSet;


pub struct Netscape {}

impl Netscape {
    pub fn sanitize(bookmark_string: String) -> String {

        let mut sanitized = bookmark_string;

        sanitized.trim();
        sanitized = sanitized.replace("\t"," ");
        sanitized = sanitized.replace("\r"," ");

        let mut bookmark_str: String = String::from(&sanitized[..]);

        let set = vec![
            r"@<!--.*-->@mis",
            r"@>(\s*?)<@mis",
            r"@(<!DOCTYPE|<META|<TITLE|<H1|<P).*\n@i",
            r"@\n<br>@mis",
            r"@\n<DD@i",
            r"barfoo",
            r"foobar",
        ];


        for regex in set {
            let re = Regex::new(regex).unwrap();
            let result = re.replace_all(&bookmark_str, "").to_string();
            bookmark_str = result;
        }

        sanitized = bookmark_str;

        sanitized.to_string()
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
