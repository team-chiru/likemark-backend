extern crate bookmarkt;
extern crate env_logger;

extern crate rusqlite;

use self::rusqlite::Connection;
use bookmarkt::dao::base::*;
use bookmarkt::dao::entity_dao::*;
use bookmarkt::common::link::*;
use bookmarkt::common::node::*;
use bookmarkt::common::tree_id::*;
use bookmarkt::common::criteria::*;
use bookmarkt::common::utils::load_file;
use bookmarkt::core::logic::html_parser::Parser;

fn main() {
    //let db = match Connection::open("res/bookmarkt.db") {
    let db = match Connection::open_in_memory() {
        Ok(c) => c,
        Err(err) => panic!("OPEN TEST DB FAILED: {}", err),
    };

    let read_sql = String::from("res/sql/entity/read.sql");
    let delete_sql = String::from("res/sql/entity/delete.sql");
    let insert_sql = String::from("res/sql/entity/insert.sql");
    let update_sql = String::from("res/sql/entity/update.sql");
    let list_sql = String::from("res/sql/entity/list.sql");

    let config = SqlConfig {
        connection: db,
        read_sql: load_file(&read_sql).unwrap(),
        delete_sql: load_file(&delete_sql).unwrap(),
        insert_sql: load_file(&insert_sql).unwrap(),
        update_sql: load_file(&update_sql).unwrap(),
        list_sql: load_file(&list_sql).unwrap(),
    };

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

    let mut c = Criteria::new();
    let id_test = TreeId::new("02".to_string());

    let node = EntityDao::read::<Node>(&config, &c.tree_id(&id_test));
    println!("{:?}\n", node);

    let link = EntityDao::read::<Link>(&config, &c.tree_id(&id_test));
    println!("{:?}\n", link);

    let bookmark_file_path = String::from("res/bookmark_file/bookmark_chrome.html");
    let mut parser = Parser::new(bookmark_file_path);
    parser.import();
}
