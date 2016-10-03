extern crate time;

use self::time::Timespec;


#[derive(Debug)]
struct bookmark {
    id: i32,
    name: String,
    time_created: Timespec,
    url: String,
    stamp: Timespec,
    rev_no: i32

}
