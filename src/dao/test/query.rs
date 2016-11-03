extern crate chrono;

use common::bookmark::Bookmark;
use self::chrono::offset::local::Local;
use self::chrono::datetime::DateTime;

use super::query_parser::*;
use super::bookmark_dao::*;

fn init_bookmart_res() -> Bookmark {
    let now = Local::now();
    Bookmark {
        id: 1,
        name: String::from("test"),
        url: String::from("test.com"),
        time_created: now,
        stamp: now,
        rev_no: 0
    }
}

#[test]
fn test_insert() {
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    let b2 = b.clone();

    assert!(b2 == dao.read(b).unwrap());
}


#[test]
fn test_delete() {
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    let b2 = b.clone();

    dao.insert(b);
    assert!(dao.delete(b2));
}

/*
#[test]
fn test_read() {
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    assert!(dao.read(b));
}

#[test]
fn test_update() {
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    assert!(dao.update(b))
}

#[test]
fn test_list() {
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    assert!(dao.list());
}
*/
