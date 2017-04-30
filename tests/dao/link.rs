extern crate bookmarkt;
extern crate uuid;

use super::db_wrapper;

use bookmarkt::common::entity::{ Entity, EntityBuilder };
use bookmarkt::common::model::{ Link };
use bookmarkt::common::TreePath;
use bookmarkt::common::types::FnType;

use bookmarkt::dao::{ Dao, EntityDao };

use self::uuid::Uuid;

#[test]
fn read() {
    let db = db_wrapper::init();

    let test_id = 5;
    let test_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000005").unwrap();

    let unit = Link {
        id: test_uuid,
        path: TreePath::new(String::from("0001")),
        name: String::from("test"),
        url: String::from("test"),
        fn_type: FnType::None
    };

    // test id
    let mut entity_id: Entity = EntityBuilder::default()
        .id(Some(test_id))
        .build().unwrap();

    match EntityDao::read::<Link>(&db, &mut entity_id) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }

    // test uuid
    let uuid_str = test_uuid.hyphenated().to_string();
    let mut entity_uuid: Entity = EntityBuilder::default()
        .uuid(Some(uuid_str))
        .build().unwrap();

    match EntityDao::read::<Link>(&db, &mut entity_uuid) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn insert() {
    let db = db_wrapper::init();

    let test_id = 17;
    let test_uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000017").unwrap();

    let unit = Link {
        id: test_uuid,
        path: TreePath::new(String::from("03")),
        name: String::from("inserted"),
        url: String::from("test"),
        fn_type: FnType::None
    };

    EntityDao::insert(&db, &unit).unwrap();

    // test id
    let mut entity_id: Entity = EntityBuilder::default()
        .id(Some(test_id))
        .build().unwrap();

    match EntityDao::read::<Link>(&db, &mut entity_id) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }

    // test uuid
    let uuid_str = test_uuid.hyphenated().to_string();
    let mut entity_uuid: Entity = EntityBuilder::default()
        .uuid(Some(uuid_str))
        .build().unwrap();

    match EntityDao::read::<Link>(&db, &mut entity_uuid) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
#[should_panic]
fn delete() {
    let db = db_wrapper::init();
    let test_id = 3;

    let mut entity_id: Entity = EntityBuilder::default()
        .id(Some(test_id))
        .build().unwrap();

    EntityDao::delete(&db, &mut entity_id).unwrap();
    match EntityDao::read::<Link>(&db, &mut entity_id) {
        Ok(_) => println!("{}", "IT NEVER PRINTS THIS"),
        Err(err) => panic!("READ FAILED: {}", err),
    }
}

#[test]
fn update() {
    let db = db_wrapper::init();
    let test_id = 5;

    let mut entity_id: Entity = EntityBuilder::default()
        .id(Some(test_id))
        .build().unwrap();

    let read = EntityDao::read::<Link>(&db, &mut entity_id).unwrap();
    let unit = Link {
        id: read.id,
        path: read.path,
        name: String::from("updated"),
        url: read.url,
        fn_type: read.fn_type
    };

    EntityDao::update::<Link>(&db, &unit).unwrap();
    match EntityDao::read::<Link>(&db, &mut entity_id) {
        Ok(l) => assert!(l == unit),
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

    match EntityDao::list::<Link>(&db, &mut entity_name) {
        Ok(v) => assert!(v.len() == 8),
        Err(err) => panic!("LIST FAILED: {}", err)
    }
}
