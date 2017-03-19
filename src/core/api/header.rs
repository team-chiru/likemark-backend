use iron::prelude::*;
use iron::status::Status;
//use iron::{AfterMiddleware};
use iron::headers::{ Accept, AccessControlAllowOrigin, AccessControlAllowHeaders, ContentType };
use iron::mime::{ Mime, TopLevel, SubLevel };

use unicase::UniCase;

use super::RequestRule;
use super::ResponseFormater;

fn mime() -> Mime {
    Mime(
        TopLevel::Application,
        SubLevel::Ext(String::from("vnd.api+json")),
        vec![]
    )
}

fn content_type() -> ContentType {
    ContentType(
        mime()
    )
}

fn allow_headers() -> AccessControlAllowHeaders {
    AccessControlAllowHeaders(
        vec![
            UniCase("content-type".to_owned())
        ]
    )
}

fn allow_origin() -> AccessControlAllowOrigin {
    AccessControlAllowOrigin::Any
}

pub struct Matcher;

impl ResponseFormater for Matcher {
    fn format(res: &mut Response) {
        res.headers.set(allow_headers());
        res.headers.set(allow_origin());
        res.headers.set(content_type());
    }
}

impl RequestRule<Status> for Matcher {
    fn check(req: &Request) -> Status {
        if let Some(content) = req.headers.get::<ContentType>() {
            if *content != content_type() {
                return Status::UnsupportedMediaType;
            }
        }

        if let Some(accepts) = req.headers.get::<Accept>() {
            for quality in accepts.iter() {
                if quality.item == mime() {
                    return Status::Ok;
                }
            }
        }

        Status::NotAcceptable
    }
}
