use iron::prelude::*;
<<<<<<< HEAD
use iron::{ status };

use iron::headers::{ Accept, qitem, AccessControlAllowOrigin, AccessControlAllowHeaders, ContentType };
use iron::mime::{ Mime, TopLevel, SubLevel };
use unicase::UniCase;

fn media_type() -> Mime {
    Mime(
        TopLevel::Application,
        SubLevel::Ext(String::from("vnd.api+json")),
        vec![]
    )
=======
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
>>>>>>> 4-optional-tree_id
}

pub fn serve() {
    Iron::new(|req: &mut Request| {
<<<<<<< HEAD
        println!("{:?}", req.headers.get::<ContentType>());

        let mut resp = Response::with((status::Ok, String::from("{ \"data\": \"Hello\" }\n")));

        let content_type = ContentType(media_type());

        if req.headers.get::<ContentType>() != Some(&content_type) {
            resp = Response::with(status::MethodNotAllowed);
            resp.headers.set(
                Accept(vec![
                    qitem(media_type())
                ])
            );
        }

        resp.headers.set(AccessControlAllowOrigin::Any);
        resp.headers.set(
            AccessControlAllowHeaders(vec![
                UniCase("content-type".to_owned())
            ])
        );
        resp.headers.set(ContentType(media_type()));

        println!("{:?}", resp);

=======
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
>>>>>>> 4-optional-tree_id
        Ok(resp)
    }).http("localhost:3000").unwrap();
}
