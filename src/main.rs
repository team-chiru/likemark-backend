extern crate bookmarkt;

use bookmarkt::dao::bookmark_dao::*;
use bookmarkt::logic::services::*;


fn main() {
    println!("{}", hello_from_dao());
    println!("{}", hello_from_logic());
}
