extern crate bookmarkt;
extern crate env_logger;
extern crate rusqlite;

use bookmarkt::core::server;

fn main() {
    server::serve();
    println!("Server started on localhost:3000");
}
