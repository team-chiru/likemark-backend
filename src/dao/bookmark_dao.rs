extern crate chrono;
extern crate rusqlite;

use common::bookmark::*;
use dao::query_parser::*;

#[derive(Debug)]
pub struct BookmarkDao<'a> {
    pub connection: &'a rusqlite::Connection,
    pub read_sql: String,
    pub delete_sql: String,
    pub insert_sql: String,
    pub update_sql: String,
    pub list_sql: String
}

impl<'a> BookmarkDao<'a> {
    pub fn insert(&self, b: Bookmark) {
        let insert_query = parse_query(&b.to_btree(), String::from(&*self.insert_sql));

        match self.connection.execute(insert_query.as_str(), &[] ) {
            Ok(insert) => println!("{} rows were inserted", insert),
            Err(err) => panic!("insert failed: {}", err),
        }
    }


    pub fn delete(&self, b: Bookmark) {
        let delete_query = parse_query(&b.to_btree(), String::from(&*self.delete_sql));

        self.connection.execute(delete_query.as_str(), &[] )
            .expect("delete failed");
    }

    pub fn read(&self, b: Bookmark) -> Result<Bookmark, rusqlite::Error> {
        let read_query = parse_query(&b.to_btree(), String::from(&*self.read_sql));

        let mut stmt = match self.connection.prepare(read_query.as_str()) {
            Ok(read) => read,
            Err(err) => panic!("read failed: {}", err),
        };

        let bookmark_iter = stmt.query_map(&[], |row| {
            Bookmark {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
                rev_no: row.get(5)
            }
        }).unwrap();

        bookmark_iter.last().expect("read failed !")
    }

    pub fn update(&self, b: Bookmark) {
        let update_query = parse_query(&b.to_btree(), String::from(&*self.update_sql));

        match self.connection.execute(update_query.as_str(), &[] ) {
            Ok(update) => update,
            Err(err) => panic!("listed failed: {}", err),
        };

    }

    pub fn list(&self, b: Bookmark) -> Vec<Bookmark> {
        let mut list_bookmark = Vec::<Bookmark>::new();
        let list_query = parse_query(&b.to_btree(), String::from(&*self.list_sql));

         let mut query_result = match self.connection.prepare(list_query.as_str()) {
            Ok(list) => list,
            Err(err) => panic!("listed failed: {}", err),
        };

        let bookmark_iter = query_result.query_map(&[], |row| {
            Bookmark {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
                rev_no: row.get(3)
            }
        }).unwrap();

        for result in bookmark_iter {
            match result {
                Ok(b) => list_bookmark.push(b),
                Err(_) => continue
            }
        }

        list_bookmark
    }
}



pub fn hello_from_dao() -> String {
    "Hello, I am bookmark dao!".to_string()
}
