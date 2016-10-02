extern crate bookmarkt;

use bookmarkt::dao::bookmark::hello_from_logic;
use bookmarkt::logic::bookmark::hello_from_dao;

fn main() {
    println!("{}", hello_from_dao());
    println!("{}", hello_from_logic());
}
