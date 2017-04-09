use iron::prelude::*;
use iron::method::Method;
use iron::status::Status;

use serde_json;

use super::api::{ header, body };
use super::api::{ RequestRule, ResponseFormater };

fn get_body() -> String {
    let body = body::SuccessBody {
        data: None,
        meta: body::meta(),
        jsonapi: body::jsonapi(),
    };

    serde_json::to_string(&body).unwrap()
}

pub fn serve() {
    Iron::new(|req: &mut Request| {
        let mut resp = match req.method {
            Method::Get =>
                Response::with((
                    header::Matcher::check(req),
                    get_body()
                )),
            Method::Options => Response::with(Status::Ok),
            _ => Response::with(Status::MethodNotAllowed),
        };

        header::Matcher::format(&mut resp);
        Ok(resp)
    }).http("localhost:3000").unwrap();
}
