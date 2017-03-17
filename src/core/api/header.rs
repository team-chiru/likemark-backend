use iron::prelude::*;
use iron::status::Status;
//use iron::{AfterMiddleware};
use iron::headers::{ Accept, AccessControlAllowOrigin, AccessControlAllowHeaders, ContentType };
use iron::mime::{ Mime, TopLevel, SubLevel };

use unicase::UniCase;

use super::RequestRule;
use super::ResponseFormater;

pub struct Manager;
impl Manager {
    fn content_type() -> ContentType {
        ContentType(
            Mime(
                TopLevel::Application,
                SubLevel::Ext(String::from("vnd.api+json")),
                vec![]
            )
        )
    }

    fn allow_headers() -> AccessControlAllowHeaders {
        AccessControlAllowHeaders(vec![
            UniCase("content-type".to_owned())
        ])
    }

    fn allow_origin() -> AccessControlAllowOrigin {
        AccessControlAllowOrigin::Any
    }
}

impl ResponseFormater for Manager {
    fn format(res: &mut Response) {
        res.headers.set(Manager::allow_headers());
        res.headers.set(Manager::allow_origin());
        res.headers.set(Manager::content_type());
    }
}

impl RequestRule<Status> for Manager {
    fn check(req: &Request) -> Status {
        if let Some(content) = req.headers.get::<ContentType>() {
            if *content != Manager::content_type() {
                Status::Ok
            } else {
                Status::UnsupportedMediaType
            }
        } else {
            if let Some(accept) = req.headers.get::<Accept>() {
                for quality in accept.iter() {
                    println!("{:?}", quality.item);
                }

                Status::Ok
            } else {
                Status::Ok
            }
        }
    }
}
