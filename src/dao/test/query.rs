extern crate chrono;


use common::bookmark::Link;
use self::chrono::offset::local::Local;
use self::chrono::datetime::DateTime;

use super::query_parser::*;
use super::bookmark_dao::*;

fn init_link_res() -> Link {
    let now = Local::now();

    Link {
        id: 1,
        name: String::from("test"),
        url: String::from("test.com"),
        rev_no: 0
    }
}

#[test]
fn test_insert() {
    let dao = LinkDao {
        connection: &db,
        read_sql: dump_file!("res/sql/bookmark/read.sql"),
        delete_sql: dump_file!("res/sql/bookmark/delete.sql"),
        insert_sql: dump_file!("res/sql/bookmark/insert.sql"),
        update_sql: dump_file!("res/sql/bookmark/update.sql"),
        list_sql: dump_file!("res/sql/bookmark/list.sql")
    };

    let link = init_link_res();

    assert!(dao.insert(link));

}

/*
#[test]
fn test_delete() {
    let dao = BookmarkDao::new();

    let b = init_bookmart_res();
    let b2 = b.clone();

    dao.insert(b);
    assert!(dao.delete(b2));
}


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
