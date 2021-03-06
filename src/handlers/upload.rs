use std::io::Read as IoRead;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use persistent::Read;

use max_length::MaxLength;
use snowflake::snowflake_b64;
use {MaxFileSize, DB};

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let max_length = req.get::<Read<MaxFileSize>>().unwrap();
    let mut body = String::new();
    MaxLength::new(&mut req.body, *max_length)
        .read_to_string(&mut body)
        .map_err(|err| IronError::new(err, status::BadRequest))?;
    let (id_num, id_b64) = snowflake_b64();

    let mutex = req.get::<Read<DB>>().unwrap();
    let db = mutex.lock().unwrap();

    db.execute_named(
        "INSERT INTO 'paste-acm' (id, data) VALUES (:id, :data)",
        &[(":id", &(id_num as i64)), (":data", &body)],
    )
    .map_err(|err| IronError::new(err, status::InternalServerError))?;
    drop(db);

    let mime: Mime = "text/plain".parse().unwrap();
    let url = &req.url;
    let port = match (url.scheme(), url.port()) {
        ("http", 80) | ("https", 443) => String::new(),
        (_, port) => format!(":{}", port),
    };
    let body = format!("{}://{}{}/{}\n", url.scheme(), url.host(), port, id_b64);
    Ok(Response::with((status::Ok, mime, body)))
}
