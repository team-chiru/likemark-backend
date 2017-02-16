extern crate rusqlite;

use dao::base::Dao;
use dao::base::SqlConfig;
use dao::query_parser::*;

use common::criteria::*;
use common::types::FnType;
use common::types::StructType;

use common::entity::*;

macro_rules! create_entity {
    ($stmt:expr) => {{
        $stmt.query_map(&[], |row| {
            let struct_type: String = row.get(4);
            let fn_type: String = row.get(5);

            Entity {
                id: row.get(0),
                tree_id: row.get(1),
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
    fn read<T>(s: SqlConfig, c: &Criteria) -> Result<T, String>
        where T: FromEntity
    {
        let template = s.read_sql.clone();

        let read_query = parse_query(&c.map(), template);
        let mut stmt = match s.connection.prepare(read_query.as_str()) {
            Ok(read) => read,
            Err(err) => {
                return Err(format!("Read failed: {}", err));
            }
        };

        let mut entity_iter = match create_entity!(stmt) {
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
}
