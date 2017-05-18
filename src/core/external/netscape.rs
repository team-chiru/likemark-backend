extern crate regex;

use std::ops::IndexMut;

use common::model::{ Composite, Node };
use common::types::{ StructType };
use common::deep::{ Traversal, TreePath };
use common::entity::{ Entity, EntityBuilder };

use core::external::Converter;
use self::regex::Regex;

use super::types::{ Tag, Attribut };

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{ NodeData, RcDom, Handle };


struct EntityIncubator {
    entities: Vec<Entity>,
    current: Option<EntityBuilder>,
    is_phantom: bool,
}

impl EntityIncubator {
    fn incube(&mut self) {
        if !self.is_phantom {
            self.incube();
            self.current = Some(EntityBuilder::default());
        }

        self.is_phantom = false;
    }

    fn current(&mut self) -> &mut EntityBuilder {
        if self.current.is_none() {
            self.current = Some(EntityBuilder::default());
            self.is_phantom = true;
        }

        &mut self.current.unwrap()
    }

    fn lay(&mut self) {
        match self.current {
            Some(builder) =>
                match builder.build() {
                    Ok(e) => self.entities.push(e),
                    Err(_) => panic!("build error: unsatisfied builder "),
                },
            None => panic!("build error: empty builders")
        }
    }
}

struct NetscapeParser {
    tree_path: TreePath,
    saved_tags: Vec<Tag>
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
            NodeData::Text { ref contents } => {
                println!("{}", escape_default(&contents.borrow()));

                match self.saved_tags.pop() {
                    Some(Tag::A) | Some(Tag::H3) => {
                        let mut builder = self.builders.pop().unwrap();
                        builder.name(Some(escape_default(&contents.borrow())));
                        self.builders.push(builder);
                    },
                    Some(_) => {},
                    None => {}
                }
            },

            NodeData::Element { ref name, ref attrs, .. } => {
                println!("{}", escape_default(&name.local));

                match Tag::from(name) {
                    Tag::DT => {
                        self.create();

                        let path = self.tree_path.clone();
                        self.current().path(Some(path));
                        self.current().struct_type(Some(StructType::Link));

                        self.walk_nodes(handle.children.borrow().to_vec());
                    },
                    Tag::DD => {
                        // description
                        self.saved_tags.push(Tag::DD);
                        self.walk_nodes(handle.children.borrow().to_vec());
                    },
                    Tag::DL => {
                        self.create();

                        let path = self.tree_path.clone();
                        self.current().path(Some(path));
                        self.current().struct_type(Some(StructType::Node));

                        self.walk_nodes(handle.children.borrow().to_vec());
                        self.build();
                    },
                    Tag::H3 => {
                        // add_date
                        // last_modified
                        self.saved_tags.push(Tag::H3);
                        self.walk_nodes(handle.children.borrow().to_vec());
                    },
                    Tag::A => {
                        // add_date
                        // private

                        for ref attr in attrs.borrow().iter() {
                            match Attribut::from(&attr.name) {
                                Attribut::Href => {
                                    let mut builder = self.builders.pop().unwrap();
                                    builder.url(Some(format!("{}", attr.value)));
                                    self.builders.push(builder);
                                },
                                _ => {}
                            }
                        }

                        self.saved_tags.push(Tag::A);
                        self.walk_nodes(handle.children.borrow().to_vec());
                    },
                    Tag::TITLE => {},
                    Tag::H1 => {},
                    Tag::None => self.walk_nodes(handle.children.borrow().to_vec()),
                }
            },

            _ => self.walk_nodes(handle.children.borrow().to_vec())
        }
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

//struct NetscapeBuilder {}

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
            builders: vec![],
            tree_path: TreePath::new(String::from("00")),
            saved_tags: vec![],
            is_phantom: false
        };

        parser.walk_node(document);

        println!("{:?}", parser.entities);
        Node::compose_vec(parser.entities)
    }

    fn build(nodes: Vec<Node>) -> String {
        nodes.len();
        unimplemented!();
    }
}
