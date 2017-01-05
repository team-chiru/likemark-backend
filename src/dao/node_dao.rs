use dao::dao::Dao;
use dao::dao::SqlConfig;
use dao::query_parser::*;

use common::Node;
use common::Criteria;
use common::types::FnType;
use common::types::StructType;

struct NodeDao;

impl Dao for NodeDao {
    fn read(c: &Criteria, config: &SqlConfig) -> Result<Link, String> {
        let template = config.get_read_sql();
        if c.id == None {
            return Err(String::from("Invalid criteria: id must be set!"));
        }

        let read_query = parse_query(&c.to_btree(), template);
        let mut stmt = match config.connection.prepare(read_query.as_str()) {
            Ok(read) => read,
            Err(err) => {
                return Err(format!("Read failed: {}", err));
            }
        };

        let link_iter = match stmt.query_map(&[], |row| {
            let struct_type: String = row.get(4);
            let fn_type: String = row.get(5);

            Link {
                id: row.get(0),
                parent_id: row.get(1),
                name: row.get(2),
                url: row.get(3),
                struct_type: StructType::from(struct_type),
                fn_type: FnType::from(fn_type),
                rev_no: row.get(6),
            }
        }) {
            Ok(l) => l,
            Err(err) => {
                return Err(format!("Read failed: {}", err));
            }
        };

        match link_iter.last() {
            Some(l) => Ok(l.unwrap()),
            None => Err(format!("Nothing to read!")),
        }
    }

    fn insert(e: &Link, config: &SqlConfig) -> Result<i32, String> {
        let btree = e.clone().to_btree();
        let template = config.get_insert_sql();

        match config.connection.execute(parse_query(&btree, template).as_str(), &[]) {
            Ok(insert) => Ok(insert),
            Err(err) => Err(format!("Insert failed: {}", err)),
        }
    }

    fn delete(c: &Criteria, config: &SqlConfig) -> Result<i32, String> {
        if c.id == None {
            return Err(String::from("Invalid criteria: id must be set!"));
        }

        let delete_query = parse_query(&c.to_btree(), config.get_delete_sql());
        match config.connection.execute(delete_query.as_str(), &[]) {
            Ok(_) => Ok(c.id.unwrap()),
            Err(err) => Err(format!("Delete failed: {}", err)),
        }
    }

    fn update(e: Link, config: &SqlConfig) -> Result<i32, String> {
        let update_id = e.id;
        let update_query = parse_query(&e.to_btree(), config.get_update_sql());

        match config.connection.execute(update_query.as_str(), &[]) {
            Ok(_) => Ok(update_id),
            Err(err) => Err(format!("Update failed: {}", err)),
        }
    }

    fn list(c: &Criteria, config: &SqlConfig) -> Result<Vec<Link>, String> {
        let mut list_link = Vec::<Link>::new();
        let list_query = parse_query(&c.to_btree(), config.get_list_sql());

        let mut query_result = match config.connection.prepare(list_query.as_str()) {
            Ok(list) => list,
            Err(err) => {
                return Err(format!("List failed: {}", err));
            }
        };

        let link_iter = match query_result.query_map(&[], |row| {
            let struct_type: String = row.get(4);
            let fn_type: String = row.get(5);

            Link {
                id: row.get(0),
                parent_id: row.get(1),
                name: row.get(2),
                url: row.get(3),
                struct_type: StructType::from(struct_type),
                fn_type: FnType::from(fn_type),
                rev_no: row.get(6),
            }
        }) {
            Ok(l) => l,
            Err(err) => {
                return Err(format!("List failed: {}", err));
            }
        };

        for result in link_iter {
            match result {
                Ok(b) => list_link.push(b),
                Err(_) => continue,
            }
        }

        Ok(list_link)
    }
}

// trait SqliteDao: Dao {
//    fn get_connection() -> rusqlite::Connection {
//        match Connection::open_in_memory() {
//            Ok(c) => c,
//            Err(err) => panic!("OPEN TEST DB FAILED: {}", err),
//        }
//    }
//
