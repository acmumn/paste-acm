use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::NotImplemented))
}
