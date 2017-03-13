use iron::prelude::*;
use iron::{ status };

use iron::headers::{ AccessControlAllowOrigin, AccessControlAllowHeaders, ContentType };
use iron::mime::{ Mime, TopLevel, SubLevel };
use unicase::UniCase;

pub fn serve() {

    Iron::new(|req: &mut Request| {
        println!("{:?}", req);

        let mut resp = Response::with((status::Ok, String::from("{ \"data\": \"Hello\" }\n")));

        resp.headers.set(AccessControlAllowOrigin::Any);
        resp.headers.set(
            AccessControlAllowHeaders(vec![
                UniCase("content-type".to_owned())
            ])
        );

        let sub_type = String::from("vnd.api+json");
        resp.headers.set(
            ContentType(
                Mime(TopLevel::Application, SubLevel::Ext(sub_type), vec![])
            )
        );

        println!("{:?}", resp);

        Ok(resp)
    }).http("localhost:3000").unwrap();
}
