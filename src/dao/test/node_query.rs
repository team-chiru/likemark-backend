extern crate rusqlite

use  common::Node;

use common::types::StructType;
use common::types::FnType;
use common::Criteria;

use common::utils::load_file;

use super::base::Dao;
use super::base::SqlConfig;
use super::node_dao::NodeDao;

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
    let dao = NodeDao { config: init_test_db() };
    let id_test = 1;
    let node = Node {
        id: id_test,
        parent_id: String::from("1"),
        name: String::from("test_read"),
        url: String::from("test_url"),
        struct_type: StructType::Node,
        fn_type: FnType::None,
        rev_no: 0,
    };

}
