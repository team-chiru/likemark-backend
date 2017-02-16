extern crate rusqlite;

use dao::base::Dao;
use dao::base::SqlConfig;
use dao::query_parser::*;

use common::Link;
use common::Criteria;
use common::types::FnType;
use common::types::StructType;

macro_rules! create_link {
    ($stmt:expr) => {{
        $stmt.query_map(&[], |row| {
            let struct_type: String = row.get(4);
            let fn_type: String = row.get(5);

            Entity {
                id: row.get(0),
                parent_id: row.get(1),
                name: row.get(2),
                url: row.get(3),
                struct_type: StructType::from(struct_type),
                fn_type: FnType::from(fn_type),
                rev_no: row.get(6),
            }
        })
    }};
}

pub struct EntityDao {
    pub config: SqlConfig,
}

impl EntityDao {
    fn read(&self, c: &Criteria) -> Result<Link, String> {
        let template = self.config.read_sql.clone();
        if c.id == None {
            return Err(String::from("Invalid criteria: id must be set!"));
        }

        let read_query = parse_query(&c.to_btree(), template);
        let mut stmt = match self.config.connection.prepare(read_query.as_str()) {
            Ok(read) => read,
            Err(err) => {
                return Err(format!("Read failed: {}", err));
            }
        };

        let link_iter = match create_link!(stmt) {
            Ok(l) => l,
            Err(err) => {
                return Err(format!("List failed: {}", err));
            }
        };

        match link_iter.last() {
            Some(l) => Ok(l.unwrap()),
            None => Err(format!("Nothing to read!")),
        }
    }

    fn insert(&self, e: &Link) -> Result<i32, String> {
        let btree = e.clone().to_btree();
        let template = self.config.insert_sql.clone();

        match self.config.connection.execute(parse_query(&btree, template).as_str(), &[]) {
            Ok(insert) => Ok(insert),
            Err(err) => Err(format!("Insert failed: {}", err)),
        }
    }

    fn delete(&self, c: &Criteria) -> Result<i32, String> {
        if c.id == None {
            return Err(String::from("Invalid criteria: id must be set!"));
        }

        let delete_query = parse_query(&c.to_btree(), self.config.delete_sql.clone());
        match self.config.connection.execute(delete_query.as_str(), &[]) {
            Ok(_) => Ok(c.id.unwrap()),
            Err(err) => Err(format!("Delete failed: {}", err)),
        }
    }

    fn update(&self, e: Link) -> Result<i32, String> {
        let update_id = e.id;
        let update_query = parse_query(&e.to_btree(), self.config.update_sql.clone());

        match self.config.connection.execute(update_query.as_str(), &[]) {
            Ok(_) => Ok(update_id),
            Err(err) => Err(format!("Update failed: {}", err)),
        }
    }

    fn list(&self, c: &Criteria) -> Result<Vec<Link>, String> {
        let mut list_link = Vec::<Link>::new();
        let list_query = parse_query(&c.to_btree(), self.config.list_sql.clone());

        let mut stmt = match self.config.connection.prepare(list_query.as_str()) {
            Ok(list) => list,
            Err(err) => {
                return Err(format!("List failed: {}", err));
            }
        };

        let link_iter = match create_link!(stmt) {
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
