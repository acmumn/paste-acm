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
    db.execute(
        "INSERT INTO 'paste-acm' (id, data) VALUES (?, ?)",
        &[&(id_num as i64), &body],
    )
    .map_err(|err| IronError::new(err, status::InternalServerError))?;
    drop(db);

    let mime: Mime = "text/plain".parse().unwrap();
    let body = format!("https://p.acm.umn.edu/{}\n", id_b64);
    Ok(Response::with((status::Ok, mime, body)))
}
