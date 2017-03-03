extern crate bookmarkt;
use super::db_wrapper;

use bookmarkt::common::Link;
use bookmarkt::common::TreeId;
use bookmarkt::common::types::FnType;
use bookmarkt::common::Criteria;

use bookmarkt::dao::base::Dao;
use bookmarkt::dao::EntityDao;

#[test]
fn read() {
    let db = db_wrapper::init();
    let test = 5;
    let mut c = Criteria::new();

    let unit = Link {
        id: test,
        path: TreeId::new(String::from("00001")),
        name: String::from("test"),
        url: String::from("test"),
        fn_type: FnType::None
    };

    match EntityDao::read::<Link>(&db, &c.id(test)) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn insert() {
    let db = db_wrapper::init();
    let test = 17;
    let mut c = Criteria::new();

    let unit = Link {
        id: test,
        path: TreeId::new(String::from("03")),
        name: String::from("inserted"),
        url: String::from("test"),
        fn_type: FnType::None
    };

    EntityDao::insert::<Link>(&db, &unit).unwrap();
    match EntityDao::read::<Link>(&db, &c.id(test)) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
#[should_panic]
fn delete() {
    let db = db_wrapper::init();
    let test = 3;
    let mut c = Criteria::new();

    EntityDao::delete::<Link>(&db, &c.id(test)).unwrap();
    match EntityDao::read::<Link>(&db, &c.id(test)) {
        Ok(_) => println!("{}", "IT NEVER PRINTS THIS"),
        Err(err) => panic!("READ FAILED: {}", err),
    }
}

#[test]
fn update() {
    let db = db_wrapper::init();
    let test = 5;
    let mut c = Criteria::new();

    let read = EntityDao::read::<Link>(&db, &c.id(test)).unwrap();
    let unit = Link {
        id: read.id,
        path: read.path,
        name: String::from("updated"),
        url: read.url,
        fn_type: read.fn_type
    };

    EntityDao::update::<Link>(&db, &unit).unwrap();
    match EntityDao::read::<Link>(&db, &c.id(test)) {
        Ok(l) => assert!(l == unit),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn list() {
    let db = db_wrapper::init();
    let test = String::from("test");
    let mut c = Criteria::new();

    match EntityDao::list::<Link>(&db, &c.name(test)) {
        Ok(v) => assert!(v.len() == 8),
        Err(err) => panic!("LIST FAILED: {}", err)
    }
}
