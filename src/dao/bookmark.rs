extern crate rusqlite;
extern crate time;

use time::Timespec;
use rusqlite::Connection;


#[derive(Debug)]
struct bookmark {
    id: i32,
    name: String,
    time_created: Timespec,
    url: String,
    stamp: Timespec,
    revNo: i32
}
