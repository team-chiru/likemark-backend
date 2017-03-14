use common::node::Node;
use core::external::converter::Converter;

pub struct Netscape {}

impl Converter for Netscape {

    fn parse(bookmark_string: String) -> Vec<Node> {
        let lines: Vec<&str> = bookmark_string.split("\n").collect();
        println!("{:?}",lines);
        vec![]
    }

    fn build(nodeVec: Vec<Node>) -> String {
        unimplemented!();
    }


}
