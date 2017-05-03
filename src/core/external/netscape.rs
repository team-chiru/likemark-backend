extern crate regex;

use common::model::{ Composite, Node, Link };
use common::types::{ StructType };
use common::deep::{ Traversal, TreePath };
use common::entity::{ Entity, EntityBuilder };

use core::external::Converter;
use self::regex::Regex;

use super::types::Tag;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{ NodeData, RcDom, Handle };

struct NetscapeParser {
    entities: Vec<Entity>,
    unfinished: Vec<EntityBuilder>,
    tree_path: TreePath,
}

impl NetscapeParser {
    fn walk_nodes(&mut self, handles: Vec<Handle>) {
        let saved_path = self.tree_path.clone();

        if let Some(new_path) = self.tree_path.right() {
            self.tree_path = new_path;

            for child in handles.iter() {
                self.walk_node(child.clone());
            }
        }

        self.tree_path = saved_path;
    }

    fn walk_node(&mut self, handle: Handle) {
        match handle.data {
            NodeData::Text { ref contents } => {},

            NodeData::Element { ref name, ref attrs, .. } => {
                println!("{}", name.local);
                match Tag::from(name) {
                    Tag::DT => {},
                    Tag::DD => {},
                    Tag::DL => {
                        let mut builder = EntityBuilder::default();
                        builder.path(Some(self.tree_path.clone())).struct_type(Some(StructType::Node));

                        self.unfinished.push(builder);
                        self.walk_nodes(handle.children.borrow().to_vec());

                        builder = self.unfinished.pop().unwrap();
                        self.entities.push(builder.build().unwrap())
                    },
                    Tag::H3 => {},
                    Tag::A => {},
                    Tag::TITLE => {},
                    Tag::H1 => {},
                    _ => {

                    }
                }
            },

            _ => self.walk_nodes(handle.children.borrow().to_vec())
        }
    }
}

struct NetscapeBuilder {}

pub struct Netscape {}

fn sanitize(to_sanitized: String) -> String {
    let sanitizers = vec![ r"<\\?br>", r"[\r\n] *" ];
    let mut sanitized = to_sanitized;

    for raw in sanitizers {
        let re = Regex::new(raw).unwrap();
        sanitized = String::from(re.replace_all(&sanitized, ""));
    }

    sanitized
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

impl Converter for Netscape {
    fn parse(bookmark_str: String) -> Vec<Node> {
        let html_str = sanitize(bookmark_str);

        let parser = parse_document(
            RcDom::default(),
            Default::default()
        );

        let utf8_parser = parser.from_utf8();
        let result = utf8_parser.read_from(&mut html_str.as_bytes());

        let dom = match result {
            Ok(rcdom) => rcdom,
            Err(_) => panic!("Error")
        };

        let document = dom.document;

        let mut parser = NetscapeParser {
            entities: vec![],
            unfinished: vec![],
            tree_path: TreePath::new(String::from("00")),
        };

        parser.walk_node(document);

        println!("{:?}", parser.entities);
        Node::compose_vec(parser.entities)
    }

    fn build(nodes: Vec<Node>) -> String {
        unimplemented!();
    }
}
