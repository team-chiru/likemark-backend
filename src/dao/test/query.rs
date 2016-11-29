extern crate rusqlite;

use common::bookmark::*;
use common::utils;
use super::bookmark_dao::*;

use self::rusqlite::Connection;

fn init_test_db() -> LinkDao {
    let db = match Connection::open_in_memory() {
        Ok(c) => c,
        Err(err) => panic!("OPEN TEST DB FAILED: {}", err)
    };

    let init_query = utils::dump_file(dotenv!("INIT_SQL")).unwrap();
    let init_test_query = utils::dump_file(dotenv!("INIT_TEST_SQL")).unwrap();

    match db.execute(init_query.as_str(), &[]) {
        Ok(_) => {},
        Err(err) => panic!("CREATE TEST TABLE FAILED: {}", err),
    };

    match db.execute(init_test_query.as_str(), &[]) {
        Ok(_) => {},
        Err(err) => panic!("INIT TEST DB FAILED: {}", err),
    };

    LinkDao {
        connection: db,
        read_sql: utils::dump_file(dotenv!("READ_SQL")).unwrap(),
        delete_sql: utils::dump_file(dotenv!("DELETE_SQL")).unwrap(),
        insert_sql: utils::dump_file(dotenv!("INSERT_SQL")).unwrap(),
        update_sql: utils::dump_file(dotenv!("UPDATE_SQL")).unwrap(),
        list_sql: utils::dump_file(dotenv!("LIST_SQL")).unwrap()
    }
}

#[test]
fn read() {
    let dao = init_test_db();
    let link = Link {
        id: 1,
        name: String::from("test"),
        url: String::from("test_url"),
        rev_no: 0
    };

    let read_c = LinkCriteria::new();
    match dao.read(&read_c.id(1)) {
        Ok(l) => assert!(l == link),
        Err(err) => panic!("READ FAILED: {}", err)
    }
}

#[test]
fn insert() {
    let dao = init_test_db();
    let link = Link {
        id: 2,
        name: String::from("inserted"),
        url: String::from("url"),
        rev_no: 0
    };

    dao.insert(&link).unwrap();

    let read_c = LinkCriteria::new();

    match dao.read(&read_c.id(link.id)) {
        Ok(l) => assert!(l == link),
        Err(err) => panic!("INSERT FAILED: {}", err)
    }

}

#[test]
#[should_panic]
fn test_delete() {
    let dao = init_test_db();
    let link = Link {
        id: 1,
        name: String::from("test"),
        url: String::from("test_url"),
        rev_no: 0
    };

    let crit = LinkCriteria::new().id(link.id);

    dao.delete(&crit).unwrap();
    match dao.read(&crit) {
        Ok(_) => println!("{}", "IT NEVER PRINTS THIS"),
        Err(err) => panic!("{}", err)
    }
}

#[test]
fn test_update() {
    let dao = init_test_db();
    let link = Link {
        id: 1,
        name: String::from("updated"),
        url: String::from("test_url"),
        rev_no: 0
    };

    dao.update(link.clone()).unwrap();

    let read_c = LinkCriteria::new();
    match dao.read(&read_c.id(1)) {
        Ok(l) => assert!(link == l),
        Err(err) => panic!("UPDATE FAILED: {}", err)
    }
}


#[test]
fn test_list() {
    let dao = init_test_db();
    let link = Link {
        id: 1,
        name: String::from("inseted"),
        url: String::from("test_url"),
        rev_no: 0
    };

    dao.insert(&link).unwrap();
    dao.insert(&link).unwrap();

    let list_c = LinkCriteria::new();
    match dao.list(&list_c.url(String::from("test_url"))) {
        Ok(v) => assert!(v.len() == 3),
        Err(err) => panic!("LIST FAILED: {}", err)
    }
}
