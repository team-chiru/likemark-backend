extern crate bookmarkt;
extern crate uuid;

use super::db_wrapper;

use bookmarkt::common::entity::{ Entity, EntityBuilder };
use bookmarkt::common::model::{ Link, Node };
use bookmarkt::common::TreeId;
use bookmarkt::common::types::FnType;

use bookmarkt::dao::{ Dao, EntityDao };

use self::uuid::Uuid;

fn get_read_unit() -> Node {
    Node {
        id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        path: TreeId::new(String::from("01")),
        name: String::from("test"),
        url: String::from("test"),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000007").unwrap(),
                path: TreeId::new(String::from("0100")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!(
            Node {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000008").unwrap(),
                path: TreeId::new(String::from("0101")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None,
                links: vec!(
                    Link {
                        id: Uuid::parse_str("00000000-0000-0000-0000-000000000013").unwrap(),
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
        id: Uuid::parse_str("00000000-0000-0000-0000-000000000017").unwrap(),
        path: TreeId::new(String::from("03")),
        name: String::from("test"),
        url: String::from("test"),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000018").unwrap(),
                path: TreeId::new(String::from("0300")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!(
            Node {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000019").unwrap(),
                path: TreeId::new(String::from("0301")),
                name: String::from("test"),
                url: String::from("test"),
                fn_type: FnType::None,
                links: vec!(
                    Link {
                        id: Uuid::parse_str("00000000-0000-0000-0000-000000000020").unwrap(),
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
        id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        path: TreeId::new(String::from("01")),
        name: String::from("test_update"),
        url: String::from("test"),
        fn_type: FnType::None,
        links: vec!(
            Link {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000007").unwrap(),
                path: TreeId::new(String::from("0100")),
                name: String::from("test_update"),
                url: String::from("test"),
                fn_type: FnType::None
            }
        ),
        nodes: vec!(
            Node {
                id: Uuid::parse_str("00000000-0000-0000-0000-000000000008").unwrap(),
                path: TreeId::new(String::from("0101")),
                name: String::from("test_update"),
                url: String::from("test"),
                fn_type: FnType::None,
                links: vec!(
                    Link {
                        id: Uuid::parse_str("00000000-0000-0000-0000-000000000013").unwrap(),
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

    let unit = get_read_unit();
    let test_path = unit.clone().path;

    let mut entity_path: Entity = EntityBuilder::default()
        .path(Some(test_path))
        .build().unwrap();

    match EntityDao::read::<Node>(&db, &mut entity_path) {
        Ok(n) => assert!(n == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn insert() {
    let db = db_wrapper::init();
    let unit = get_insert_unit();
    let test_path = unit.clone().path;

    let mut entity_path: Entity = EntityBuilder::default()
        .path(Some(test_path))
        .build().unwrap();

    EntityDao::insert(&db, &unit).unwrap();
    match EntityDao::read::<Node>(&db, &mut entity_path) {
        Ok(n) => assert!(n == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
#[should_panic]
fn delete() {
    let db = db_wrapper::init();
    let test_path = TreeId::new(String::from("01"));

    let mut entity_path: Entity = EntityBuilder::default()
        .path(Some(test_path))
        .build().unwrap();

    EntityDao::delete(&db, &mut entity_path).unwrap();
    match EntityDao::read::<Node>(&db, &mut entity_path) {
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

    //TODO it assumes that the node is valid
    let unit = get_update_unit();
    let test_str = String::from("test_update");
    let test_read = unit.clone().path;

    let mut entity_path: Entity = EntityBuilder::default()
        .path(Some(test_read))
        .build().unwrap();

    let mut read = EntityDao::read::<Node>(&db, &mut entity_path).unwrap();
    change_name(&test_str, &mut read);

    EntityDao::update::<Node>(&db, &unit).unwrap();
    match EntityDao::read::<Node>(&db, &mut entity_path) {
        Ok(n) => assert!(n == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn list() {
    let db = db_wrapper::init();
    let test_name = String::from("test");

    let mut entity_name: Entity = EntityBuilder::default()
        .name(Some(test_name))
        .build().unwrap();

    match EntityDao::list::<Node>(&db, &mut entity_name) {
        Ok(v) => assert!(v.len() == 3),
        Err(err) => panic!("LIST FAILED: {}", err)
    }
}
