use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;

pub fn handler(_req: &mut Request) -> IronResult<Response> {
    const BODY: &str = include_str!("../index.txt");
    let ctype = ContentType::plaintext();
    Ok(Response::with((status::Ok, Header(ctype), BODY)))
}
