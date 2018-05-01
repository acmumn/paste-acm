use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

use snowflake::snowflake_b64;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::NotImplemented))
}
