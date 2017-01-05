extern crate rusqlite;
use std::env;

use common::Link;
use common::types::StructType;
use common::types::FnType;
use common::Criteria;

use common::utils::load_file;

use super::base::Dao;
use super::base::SqlConfig;
use super::link_dao::LinkDao;

use self::rusqlite::Connection;

fn get_sql_config() -> SqlConfig {
    let db = match Connection::open_in_memory() {
        Ok(c) => c,
        Err(err) => panic!("OPEN TEST DB FAILED: {}", err),
    };

    let read_sql = String::from("res/sql/entity/read.sql");
    let delete_sql = String::from("res/sql/entity/delete.sql");
    let insert_sql = String::from("res/sql/entity/insert.sql");
    let update_sql = String::from("res/sql/entity/update.sql");
    let list_sql = String::from("res/sql/entity/list.sql");

    SqlConfig {
        connection: db,
        read_sql: load_file(&read_sql).unwrap(),
        delete_sql: load_file(&delete_sql).unwrap(),
        insert_sql: load_file(&insert_sql).unwrap(),
        update_sql: load_file(&update_sql).unwrap(),
        list_sql: load_file(&list_sql).unwrap(),
    }
}

fn init_test_db() -> SqlConfig {
    let config = get_sql_config();

    let init_sql = String::from("res/sql/entity/init.sql");
    let init_test_sql = String::from("res/sql/entity/init_test.sql");

    let init_query = load_file(&init_sql).unwrap();
    let init_test_query = load_file(&init_test_sql).unwrap();

    match config.connection.execute(init_query.as_str(), &[]) {
        Ok(_) => {}
        Err(err) => panic!("CREATE TEST TABLE FAILED: {}", err),
    };

    match config.connection.execute(init_test_query.as_str(), &[]) {
        Ok(_) => {}
        Err(err) => panic!("INIT TEST DB FAILED: {}", err),
    };

    config
}

#[test]
fn read() {
    let dao = LinkDao { config: init_test_db() };
    let id_test = 1;
    let link = Link {
        id: id_test,
        parent_id: String::from("1"),
        name: String::from("test_read"),
        url: String::from("test_url"),
        struct_type: StructType::Link,
        fn_type: FnType::None,
        rev_no: 0,
    };

    let mut read_c = Criteria::new();
    match dao.read(&read_c.id(id_test)) {
        Ok(l) => assert!(l == link),
        Err(err) => panic!("READ FAILED: {}", err),
    }
}

#[test]
fn insert() {
    let dao = LinkDao { config: init_test_db() };
    let link = Link {
        id: -1,
        parent_id: String::from("3"),
        name: String::from("inserted"),
        url: String::from("url"),
        struct_type: StructType::Link,
        fn_type: FnType::None,
        rev_no: 0,
    };

    dao.insert(&link).unwrap();
    let read_c = Criteria::new();
    match dao.read(&read_c.id(link.id)) {
        Ok(l) => assert!(l == link),
        Err(err) => panic!("INSERT FAILED: {}", err),
    }
}
// [test]
// [should_panic]
// fn test_delete() {
// let dao = init_test_db();
// let link = Link {
// id: 1,
// name: String::from("test"),
// url: String::from("test_url"),
// rev_no: 0,
// };
//
// let crit = LinkCriteria::new().id(link.id);
//
// dao.delete(&crit).unwrap();
// match dao.read(&crit) {
// Ok(_) => println!("{}", "IT NEVER PRINTS THIS"),
// Err(err) => panic!("{}", err),
// }
// }
//
// [test]
// fn test_update() {
// let dao = init_test_db();
// let link = Link {
// id: 1,
// name: String::from("updated"),
// url: String::from("test_url"),
// rev_no: 0,
// };
//
// dao.update(link.clone()).unwrap();
//
// let read_c = LinkCriteria::new();
// match dao.read(&read_c.id(1)) {
// Ok(l) => assert!(link == l),
// Err(err) => panic!("UPDATE FAILED: {}", err),
// }
// }
//
//
// [test]
// fn test_list() {
// let dao = init_test_db();
// let link = Link {
// id: 1,
// name: String::from("inseted"),
// url: String::from("test_url"),
// rev_no: 0,
// };
//
// dao.insert(&link).unwrap();
// dao.insert(&link).unwrap();
//
// let list_c = LinkCriteria::new();
// match dao.list(&list_c.url(String::from("test_url"))) {
// Ok(v) => assert!(v.len() == 3),
// Err(err) => panic!("LIST FAILED: {}", err),
// }
// }
//
//
