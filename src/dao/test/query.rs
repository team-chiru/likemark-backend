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
    let read_sql = load_sql_file("res/sql/bookmark/read.sql");
    let now = Local::now();
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    let b2 = b.clone();

    assert!(b2 == dao.read(b).unwrap());
}
