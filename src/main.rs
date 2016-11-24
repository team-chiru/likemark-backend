#![feature(field_init_shorthand)]

#[macro_use] extern crate bookmarkt;
#[macro_use] extern crate clap;

use clap::App;
use bookmarkt::core::services::bookmark_services;

fn main() {
    let yaml = load_yaml!("../res/yaml/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(submatches) = matches.subcommand_matches("import") {
        if let Some(src) = submatches.value_of("SRC") {
            match bookmark_services::BookmarkServices::import(String::from(src)) {
                Ok(f) => println!("{}", f),
                Err(msg) => println!("{}", msg)
            }
        }
    }

    if let Some(submatches) = matches.subcommand_matches("export"){
        if let Some(dest) = submatches.value_of("DEST"){

        }
    }
}
