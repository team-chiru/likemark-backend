#![feature(field_init_shorthand)]

#[macro_use] extern crate bookmarkt;
#[macro_use] extern crate clap;

use clap::App;

fn main() {
    App::new("fake").version("v1.0-beta").get_matches();
}
