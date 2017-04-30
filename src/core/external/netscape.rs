extern crate regex;

use common::model::Node;
use core::external::html_data::{ HtmlData };

use core::external::Converter;
use self::regex::Regex;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

pub struct Netscape {}

fn sanitize(bookmark_string: String) -> String {

    let mut sanitized = bookmark_string;
    sanitized = sanitized.replace("\t"," ");
    sanitized = sanitized.replace("\r"," ");

    let mut bookmark_str: String = String::from(&sanitized[..]);

    let set = vec![
        r"(?i)(<!DOCTYPE|<META|<TITLE|<H1|<P).*\s?",
        r"(?mis)<!--.*?-->\s?",
        r"(?mis)\s?<.?br>",
    ];

    let set_unique = vec![
        r"(?mis)>(\s*?)<",
        r"@\n<br>@mis",
        r"@\n<DD@i",
    ];

    for regex in set {
        let re = Regex::new(regex).unwrap();
        let result = re.replace_all(&bookmark_str, "").to_string();
        bookmark_str = result;
    }

    let mut re_unique: Regex;

    for i in 0..3 {

        re_unique = Regex::new(set_unique[i]).unwrap();

        if i == 0{
            re_unique.replace_all(&bookmark_str, ">\n<").to_string();
        }

        else if i == 1 {
            re_unique.replace_all(&bookmark_str, "<br>").to_string();
        }

        else if i == 2 {
            re_unique.replace_all(&bookmark_str, "<DD").to_string();
        }

    }

    sanitized = bookmark_str;
    sanitized.trim();
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

        let document = dom.document.borrow();
        //println!("{:?}",document);

        match document.node {
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

        Vec::new()
    }

    fn build(nodes: Vec<Node>) -> String {
        unimplemented!();
    }
}
