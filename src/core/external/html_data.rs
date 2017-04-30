use core::external::types::{ Tag, Attribut };

use std::default::Default;
use std::iter::repeat;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

pub struct HtmlData {
    pub dom: RcDom
}

impl HtmlData {
    pub fn new(input: String) -> Self {
        let parser = parse_document(
            RcDom::default(),
            Default::default()
        );

        let utf8_parser = parser.from_utf8();
        let result = utf8_parser.read_from(&mut input.as_bytes());

        match result {
            Ok(rcdom) => HtmlData { dom: rcdom },
            Err(_) => panic!("Error")
        }
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn walk(indent: usize, handle: Handle) {
    let node = handle.borrow();
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.node {
        Document
            => println!("#Document"),

        Doctype(ref name, ref public, ref system)
            => println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system),

        Text(ref text)
            => println!("#text: {:?}", escape_default(text)),

        Comment(ref text)
            => println!("<!-- {:?} -->", escape_default(text)),

        Element(ref name, _, ref attrs) => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
    }

    for child in node.children.iter() {
        walk(indent+4, child.clone());
    }
}
