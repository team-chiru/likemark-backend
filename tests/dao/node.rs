extern crate bookmarkt;
use super::db_wrapper;

use bookmarkt::common::Link;
use bookmarkt::common::Node;
use bookmarkt::common::TreeId;
use bookmarkt::common::types::FnType;

use bookmarkt::dao::Criteria;
use bookmarkt::dao::Dao;
use bookmarkt::dao::EntityDao;

fn get_read_unit() -> Node {
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

fn get_insert_unit() -> Node {
    Node {
        id: 17,
        path: TreeId::new(String::from("03")),
        name: String::from("test"),
        url: String::from("test"),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: 18,
                path: TreeId::new(String::from("0300")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!(
            Node {
                id: 19,
                path: TreeId::new(String::from("0301")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None,
                links: vec!(
                    Link {
                        id: 20,
                        path: TreeId::new(String::from("030100")),
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

fn get_update_unit() -> Node {
    Node {
        id: 2,
        path: TreeId::new(String::from("01")),
        name: String::from("test_update"),
        url: String::from("test"),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: 7,
                path: TreeId::new(String::from("0100")),
                name: String::from("test_update"),
                url: String::from("test"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!(
            Node {
                id: 8,
                path: TreeId::new(String::from("0101")),
                name: String::from("test_update"),
                url: String::from("test"),
                fn_type: FnType::None,
                links: vec!(
                    Link {
                        id: 13,
                        path: TreeId::new(String::from("010100")),
                        name: String::from("test_update"),
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

    let unit = get_read_unit();
    let test = unit.clone().path;

    match EntityDao::read(&db, c.tree_id(&test)) {
        Ok(n) => assert!(n == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn insert() {
    let db = db_wrapper::init();
    let mut c = Criteria::new();

    let unit = get_insert_unit();
    let path = unit.clone().path;

    EntityDao::insert(&db, &unit).unwrap();
    match EntityDao::read(&db, c.tree_id(&path)) {
        Ok(n) => assert!(n == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
#[should_panic]
fn delete() {
    let db = db_wrapper::init();
    let test = TreeId::new(String::from("01"));
    let mut c = Criteria::new();

    EntityDao::delete(&db, c.tree_id(&test)).unwrap();
    match EntityDao::read(&db, c.tree_id(&test)) {
        Ok(_) => println!("{}", "IT NEVER PRINTS THIS"),
        Err(err) => panic!("READ FAILED: {}", err),
    }
}

fn change_name(str: &String, base: &mut Node) {
    base.name = str.clone();

    for link in base.links.as_mut_slice() {
        link.name = str.clone();
    }

    for node in base.nodes.as_mut_slice() {
        change_name(str, node);
    }
}

#[test]
fn update() {
    let db = db_wrapper::init();
    let mut c = Criteria::new();

    //TODO it assumes that the node is valid
    let unit = get_update_unit();
    let test_str = String::from("test_update");
    let test_read = &unit.path;

    let mut read = EntityDao::read(&db, c.tree_id(test_read)).unwrap();
    change_name(&test_str, &mut read);

    EntityDao::update(&db, &unit).unwrap();
    match EntityDao::read(&db, c.tree_id(&test_read)) {
        Ok(n) => assert!(n == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn list() {
    let db = db_wrapper::init();
    let test = String::from("test");
    let mut c = Criteria::new();

    match EntityDao::list(&db, c.name(test)) {
        Ok(v) => assert!(v.len() == 3),
        Err(err) => panic!("LIST FAILED: {}", err)
    }
}
