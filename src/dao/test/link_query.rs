extern crate rusqlite;

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
    match dao.read(read_c.id(id_test)) {
        Ok(l) => assert!(l == link),
        Err(err) => panic!("READ FAILED: {}", err),
    }
}

#[test]
fn insert() {
    let dao = LinkDao { config: init_test_db() };
    let id_test = 8;
    let link = Link {
        id: id_test,
        parent_id: String::from("3"),
        name: String::from("inserted"),
        url: String::from("test_url"),
        struct_type: StructType::Link,
        fn_type: FnType::None,
        rev_no: 0,
    };

    dao.insert(&link).unwrap();

    let mut read_c = Criteria::new();
    match dao.read(read_c.id(id_test)) {
        Ok(l) => assert!(l == link),
        Err(err) => panic!("INSERT FAILED: {}", err),
    }
}

#[test]
#[should_panic]
fn test_delete() {
    let dao = LinkDao { config: init_test_db() };
    let id_test = 3;

    let mut crit = Criteria::new();
    crit.id(id_test);

    dao.delete(&crit).unwrap();
    match dao.read(&crit) {
        Ok(_) => println!("{}", "IT NEVER PRINTS THIS"),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_update() {
    let dao = LinkDao { config: init_test_db() };
    let id_test = 4;

    let mut read_c = Criteria::new();
    let link = match dao.read(read_c.id(id_test)) {
        Ok(l) => l,
        Err(err) => panic!("UPDATE FAILED: {}", err),
    };

    let updated_link = Link {
        id: link.id,
        parent_id: link.parent_id.clone(),
        name: String::from("updated"),
        url: link.url.clone(),
        struct_type: link.struct_type.clone(),
        fn_type: link.fn_type.clone(),
        rev_no: link.rev_no,
    };

    dao.update(updated_link.clone()).unwrap();

    match dao.read(read_c.id(id_test)) {
        Ok(l) => assert!(updated_link == l),
        Err(err) => panic!("UPDATE FAILED: {}", err),
    }
}

#[test]
fn test_list() {
    let dao = LinkDao { config: init_test_db() };
    let inserted_name = String::from("inserted");

    let link1 = Link {
        id: -1,
        parent_id: String::from("3"),
        name: inserted_name.clone(),
        url: String::from("test_url1"),
        struct_type: StructType::Link,
        fn_type: FnType::None,
        rev_no: 0,
    };
    let link2 = Link {
        id: -1,
        parent_id: String::from("4"),
        name: inserted_name.clone(),
        url: String::from("test_url2"),
        struct_type: StructType::Node,
        fn_type: FnType::None,
        rev_no: 1,
    };

    dao.insert(&link1).unwrap();
    dao.insert(&link2).unwrap();

    let mut list_c = Criteria::new();
    match dao.list(list_c.name(inserted_name)) {
        Ok(v) => assert!(v.len() == 2),
        Err(err) => panic!("LIST FAILED: {}", err),
    }
}
