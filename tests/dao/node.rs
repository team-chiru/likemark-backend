extern crate bookmarkt;
use super::db_wrapper;

use bookmarkt::common::Link;
use bookmarkt::common::Node;
use bookmarkt::common::TreeId;
use bookmarkt::common::types::FnType;
use bookmarkt::common::Criteria;

use bookmarkt::dao::base::Dao;
use bookmarkt::dao::EntityDao;

fn get_unit() -> Node {
    Node {
        id: 2,
        path: TreeId::new(String::from("01")),
        name: String::from("test"),
        url: String::from("test"),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: 7,
                path: TreeId::new(String::from("0100")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!(
            Node {
                id: 8,
                path: TreeId::new(String::from("0101")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None,
                links: vec!(
                    Link {
                        id: 13,
                        path: TreeId::new(String::from("010100")),
                        name: String::from("test"),
                        url: String::from("test"),
                        fn_type: FnType::None
                    }
                ),
                nodes: Vec::new()
            }
        )
    }
}

#[test]
fn read() {
    let db = db_wrapper::init();
    let mut c = Criteria::new();

    let unit = get_unit();
    let test = unit.clone().path;
    println!("{:?}\n", unit);

    match EntityDao::read::<Node>(&db, &c.tree_id(&test)) {
        Ok(n) => {
            println!("{:?}\n", n);
            assert!(n == unit);
        },
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn insert() {
    let db = db_wrapper::init();
    let test = 17;
    let mut c = Criteria::new();
}

#[test]
#[should_panic]
fn delete() {
    let db = db_wrapper::init();
    let test = 3;
    let mut c = Criteria::new();
}

#[test]
fn update() {
    let db = db_wrapper::init();
    let test = 5;
    let mut c = Criteria::new();
}

#[test]
fn list() {
    let db = db_wrapper::init();
    let test = String::from("test");
    let mut c = Criteria::new();
}
