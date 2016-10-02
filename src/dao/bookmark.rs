extern crate rusqlite;
extern crate time;

use self::time::Timespec;
use self::rusqlite::Connection;


#[derive(Debug)]
struct bookmark {
    id: i32,
    name: String,
    time_created: Timespec,
    url: String,
    stamp: Timespec,
    revNo: i32

}

pub fn hello_from_logic() -> String {
    "Hello, I am bookmark dao!".to_string()
}
