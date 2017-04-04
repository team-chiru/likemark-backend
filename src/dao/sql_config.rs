extern crate rusqlite;

pub struct SqlConfig {
    pub connection: rusqlite::Connection,
    pub read_sql: String,
    pub delete_sql: String,
    pub insert_sql: String,
    pub update_sql: String,
    pub list_sql: String,
}
