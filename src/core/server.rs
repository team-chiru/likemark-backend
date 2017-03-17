use iron::prelude::*;

use super::api::header;
use super::api::{ RequestRule, ResponseFormater };

type HeaderManager = header::Manager;

pub fn serve() {
    Iron::new(|req: &mut Request| {
        let mut resp = Response::with((
            HeaderManager::check(req),
            String::from("{ \"data\": \"Hello\" }\n")
        ));

        HeaderManager::format(&mut resp);
        Ok(resp)
    }).http("localhost:3000").unwrap();
}
