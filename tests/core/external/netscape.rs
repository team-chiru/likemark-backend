extern crate bookmarkt;

use bookmarkt::common::Link;
use bookmarkt::common::Node;
use bookmarkt::common::TreeId;
use bookmarkt::common::types::FnType;
use bookmarkt::common::types::StructType;

use bookmarkt::core::external::base::Converter;
use bookmarkt::core::external::Netscape;

use bookmarkt::common::utils::*;

fn test_import(input: String) -> Vec<Node> {
    let mut path = String::from("tests/core/import_imput/");
    path.push_str(&input);
    let file = load_file(&path).unwrap();

    Netscape::parse(file)
}

fn test_export(root: Vec<Node>) -> String {
    Netscape::build(root)
}

#[test]
fn netscape_basic() {
    let unit = Node {
        id: -1,
        path: TreeId::new(String::from("")),
        name: String::from(""),
        url: String::from(""),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: -1,
                path: TreeId::new(String::from("")),
                name: String::from("Secret stuff"),
                url: String::from("https://private.tld"),
                fn_type: FnType::None
            },
            Link {
                id: -1,
                path: TreeId::new(String::from("")),
                name: String::from("Public stuff"),
                url: String::from("http://public.tld"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!()
    };
}
