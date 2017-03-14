use common::node::Node;

pub trait Converter {
    fn parser(String) -> Vec<Node>;
    fn builder(Vec<Node>) -> String;
}
