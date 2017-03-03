extern crate rusqlite;

use dao::base::Dao;
use dao::base::SqlConfig;
use dao::query_parser::*;

use common::criteria::*;
use common::tree_id::*;
use common::types::FnType;
use common::types::StructType;

use common::entity::*;

macro_rules! create_entity {
    ($stmt:expr) => {{
        $stmt.query_map(&[], |row| {
            let struct_type: String = row.get(4);
            let fn_type: String = row.get(5);
            let path: String = row.get(1);

            Entity {
                id: row.get(0),
                path: TreeId::new(path),
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

impl Dao for EntityDao {
    fn read<T>(s: &SqlConfig, c: &Criteria) -> Result<T, String>
        where T: EntityComposite {
        let template = s.read_sql.clone();
        let mapping = &c.map_query();
        let read_query = mapping.fill_query(&template);

        let mut stmt = match s.connection.prepare(read_query.as_str()) {
            Ok(read) => read,
            Err(err) => {
                return Err(format!("Read failed: {}", err));
            }
        };

        let entity_iter = match create_entity!(stmt) {
            Ok(l) => l,
            Err(err) => {
                return Err(format!("Read failed: {}", err));
            }

        };

        let mut matches = Vec::new();
        for row in entity_iter {
            match row {
                Ok(e) => {
                    matches.push(e);
                },
                Err(err) => {
                    return Err(format!("Read failed: {}", err));
                }
            }
        }

        match T::from_entities(matches).pop() {
            Some(e) => Ok(e),
            None => Err(String::from("Any entity was found"))
        }
    }

    fn insert<T>(s: &SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: EntityComposite {
        let template = s.insert_sql.clone();
        let mut results = Vec::new();

        let entities = e.into_entities();
        for entity in entities {
            let map = entity.map_query();
            let insert_query = map.fill_query(&template);

            match s.connection.execute(insert_query.as_str(), &[]) {
                Ok(insert) => results.push(insert),
                Err(err) => {
                    return Err(format!("Insert failed: {}", err));
                }
            }
        }

        Ok(results)
    }

    fn delete<T>(s: &SqlConfig, c: &Criteria) -> Result<i32, String>
        where T: EntityComposite {
        let template = s.delete_sql.clone();
        let map = c.map_query();
        let delete_query = map.fill_query(&template);

        match s.connection.execute(delete_query.as_str(), &[]) {
            Ok(delete) => Ok(delete),
            Err(err) => Err(format!("Delete failed: {}", err)),
        }
    }

    fn update<T>(s: &SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: EntityComposite {
        let template = s.update_sql.clone();
        let mut results = Vec::new();

        let entities = e.into_entities();
        for entity in entities {
            let map = entity.map_query();
            let update_query = map.fill_query(&template);

            match s.connection.execute(update_query.as_str(), &[]) {
                Ok(update) => results.push(update),
                Err(err) => return Err(format!("Update failed: {}", err))
            }
        }

        Ok(results)
    }

    fn list<T>(s: &SqlConfig, c: &Criteria) -> Result<Vec<T>, String>
        where T: EntityComposite {
        let template = s.read_sql.clone();
        let mapping = &c.map_query();
        let list_query = mapping.fill_query(&template);

        let mut stmt = match s.connection.prepare(list_query.as_str()) {
            Ok(list) => list,
            Err(err) => {
                return Err(format!("List failed: {}", err));
            }
        };

        let entity_iter = match create_entity!(stmt) {
            Ok(l) => l,
            Err(err) => {
                return Err(format!("List failed: {}", err));
            }
        };

        let mut list_entity = Vec::<Entity>::new();
        for result in entity_iter {
            match result {
                Ok(b) => list_entity.push(b),
                Err(_) => continue,
            }
        }

        Ok(T::from_entities(list_entity))
    }
}