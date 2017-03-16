use iron::prelude::*;
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
}

pub fn serve() {
    Iron::new(|req: &mut Request| {
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

        Ok(resp)
    }).http("localhost:3000").unwrap();
}
