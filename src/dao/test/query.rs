extern crate rusqlite;

use common::bookmark::Link;
use common::utils;
use super::bookmark_dao::*;

use self::rusqlite::Connection;
use std::path::Path;

fn init_link_res() -> Link {
    Link {
        id: 1,
        name: String::from("test"),
        url: String::from("test.com"),
        rev_no: 0
    }
}

#[test]
fn test_insert() {
    let db = Connection::open(Path::new("res/BOOKMARKT.db")).unwrap();

    let dao = LinkDao {
        connection: &db,
        read_sql: utils::dump_file("res/sql/bookmark/read.sql"),
        delete_sql: utils::dump_file("res/sql/bookmark/delete.sql"),
        insert_sql: utils::dump_file("res/sql/bookmark/insert.sql"),
        update_sql: utils::dump_file("res/sql/bookmark/update.sql"),
        list_sql: utils::dump_file("res/sql/bookmark/list.sql")
    };

    let link = init_link_res();

    dao.insert(link);

    let b = dao.read(init_link_res()).unwrap();
    assert!(b == init_link_res())
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
