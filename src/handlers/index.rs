use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

pub fn handler(_req: &mut Request) -> IronResult<Response> {
    const BODY: &str = include_str!("../index.txt");
    let mime: Mime = "text/plain".parse().unwrap();
    Ok(Response::with((status::Ok, mime, BODY)))
}
