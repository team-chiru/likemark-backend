extern crate rusqlite;
extern crate bookmarkt;

use bookmarkt::common::utils::load_file;
use bookmarkt::dao::SqlConfig;
use self::rusqlite::Connection;

fn connect() -> SqlConfig {
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

pub fn init() -> SqlConfig {
    let config = connect();

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
