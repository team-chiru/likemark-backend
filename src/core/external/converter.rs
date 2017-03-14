use common::node::Node;

pub trait Converter {
    fn parse(String) -> Vec<Node>;
    fn build(Vec<Node>) -> String;
}
