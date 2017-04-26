use dao::SqlConfig;
use dao::query::{ QueryParser, QueryMap };
use dao::dao::Dao;

use common::TreePath;
use common::types::FnType;
use common::types::StructType;

use common::model::Composite;
use common::entity::Entity;

macro_rules! create_entity {
    ($stmt:expr) => {{
        $stmt.query_map(&[], |row| {
            let struct_type: String = row.get(5);
            let fn_type: String = row.get(6);
            let path: String = row.get(2);

            Entity {
                id: Some(row.get(0)),
                uuid: Some(row.get(1)),
                path: Some(TreePath::new(path)),
                name: Some(row.get(3)),
                url: Some(row.get(4)),
                struct_type: Some(StructType::from(struct_type)),
                fn_type: Some(FnType::from(fn_type)),
                rev_no: Some(row.get(7)),
            }
        })
    }};
}

pub struct EntityDao {}

impl Dao<Entity, Entity> for EntityDao {
    fn read<T>(s: &SqlConfig, c: &mut Entity) -> Result<T, String>
        where T: Composite<Entity> {
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

        match T::compose_vec(matches).pop() {
            Some(e) => Ok(e),
            None => Err(String::from("Any entity was found"))
        }
    }

    fn insert<T>(s: &SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: Composite<Entity> {
        let template = s.insert_sql.clone();
        let mut results = Vec::new();

        let entities = e.decompose();
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

    fn delete(s: &SqlConfig, c: &mut Entity) -> Result<i32, String> {
        let template = s.delete_sql.clone();
        let map = c.map_query();
        let delete_query = map.fill_query(&template);

        match s.connection.execute(delete_query.as_str(), &[]) {
            Ok(delete) => Ok(delete),
            Err(err) => Err(format!("Delete failed: {}", err)),
        }
    }

    fn update<T>(s: &SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: Composite<Entity> {
        let template = s.update_sql.clone();
        let mut results = Vec::new();

        let entities = e.decompose();
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

    fn list<T>(s: &SqlConfig, c: &mut Entity) -> Result<Vec<T>, String>
        where T: Composite<Entity> {
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

        Ok(T::compose_vec(list_entity))
    }
}
