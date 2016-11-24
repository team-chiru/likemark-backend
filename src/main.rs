#![feature(field_init_shorthand)]

#[macro_use] extern crate bookmarkt;
#[macro_use] extern crate clap;

use clap::App;
use bookmarkt::common::utils;

fn main() {
    let yaml = load_yaml!("../res/yaml/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(submatches) = matches.subcommand_matches("import") {
        if let Some(src) = submatches.value_of("SRC") {
            match utils::dump_file(src) {
                Ok(f) => println!("{}", f),
                Err(msg) => println!("{}", msg)
            }
        }
    }

    if let Some(submatches) = matches.subcommand_matches("export"){
        if let Some(dest) = submatches.value_of("DEST"){
            println!("{}", utils::dump_file(dest) );
        }
    }
}
