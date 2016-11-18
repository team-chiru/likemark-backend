extern crate chrono;
extern crate rusqlite;

use common::bookmark::*;
use dao::query_parser::*;

#[derive(Debug)]
pub struct LinkDao<'a> {
    pub connection: &'a rusqlite::Connection,
    pub read_sql: String,
    pub delete_sql: String,
    pub insert_sql: String,
    pub update_sql: String,
    pub list_sql: String
}

impl<'a> LinkDao<'a> {
    pub fn insert(&self, b: Link) {
        let insert_query = parse_query(&b.to_btree(), String::from(&*self.insert_sql));

        match self.connection.execute(insert_query.as_str(), &[] ) {
            Ok(insert) => println!("{} rows were inserted", insert),
            Err(err) => panic!("insert failed: {}", err),
        }
    }

    pub fn delete(&self, b: Link) {
        let delete_query = parse_query(&b.to_btree(), String::from(&*self.delete_sql));

        println!("{}", delete_query);
        self.connection.execute(delete_query.as_str(), &[] )
            .expect("delete failed");
    }

    pub fn read(&self, b: Link) -> Result<Link, rusqlite::Error> {
        let read_query = parse_query(&b.to_btree(), String::from(&*self.read_sql));

        let mut stmt = match self.connection.prepare(read_query.as_str()) {
            Ok(read) => read,
            Err(err) => panic!("read failed: {}", err),
        };

        let link_iter = stmt.query_map(&[], |row| {
            Link {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
                rev_no: row.get(3)
            }
        }).unwrap();

        link_iter.last().expect("read failed !")
    }

    pub fn update(&self, b: Link) {
        let update_query = parse_query(&b.to_btree(), String::from(&*self.update_sql));

        match self.connection.execute(update_query.as_str(), &[] ) {
            Ok(update) => update,
            Err(err) => panic!("listed failed: {}", err),
        };

    }

    pub fn list(&self, b: Link) -> Vec<Link> {
        let mut list_link = Vec::<Link>::new();
        let list_query = parse_query(&b.to_btree(), String::from(&*self.list_sql));

         let mut query_result = match self.connection.prepare(list_query.as_str()) {
            Ok(list) => list,
            Err(err) => panic!("listed failed: {}", err),
        };

        let link_iter = query_result.query_map(&[], |row| {
            Link {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
                rev_no: row.get(3)
            }
        }).unwrap();

        for result in link_iter {
            match result {
                Ok(b) => list_link.push(b),
                Err(_) => continue
            }
        }

        list_link
    }

    pub fn clear(&self){
        let clear_query = String::from("DELETE FROM BOOKMARK ;");
        let reset_increment = String::from("DELETE FROM sqlite_sequence WHERE name='bookmark';");
        let insert_query = String::from("INSERT INTO bookmark (name, url, rev_no ) VALUES( 'test', 'test_url', 0 );");

        println!("{}", clear_query);
        match self.connection.execute(clear_query.as_str(), &[]) {
            Ok(clear) => clear,
            Err(err) => panic!("clear failed: {}", err),
        };

        println!("{}", reset_increment);
        match self.connection.execute(reset_increment.as_str(), &[]) {
            Ok(reset) => reset,
            Err(err) => panic!("clear failed: {}", err),
        };

        println!("{}", insert_query);
        match self.connection.execute(insert_query.as_str(), &[]) {
            Ok(insert) => insert,
            Err(err) => panic!("insert after clear failed: {}", err),
        };

    }
}



pub fn hello_from_dao() -> String {
    "Hello, I am Link dao!".to_string()
}
