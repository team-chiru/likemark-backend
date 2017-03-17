use iron::prelude::*;

pub trait RequestRule<T> {
    fn check(&Request) -> T;
}

pub trait ResponseFormater {
    fn format(&mut Response);
}
