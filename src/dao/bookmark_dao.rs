extern crate rusqlite;
use self::rusqlite::Connection;


pub fn create_connection() -> bool{

    Connection::open_in_memory().unwrap();
    return false;
}


pub fn list_bookmark() -> bool {

    return false;

}

pub fn hello_from_dao() -> String {
    "Hello, I am bookmark dao!".to_string()
}
