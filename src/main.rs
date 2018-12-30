extern crate base64;
extern crate byteorder;
extern crate failure;
extern crate futures;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate persistent;
#[macro_use]
extern crate router;
extern crate rusqlite;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate tokio_core;

mod handlers;
mod max_length;
mod options;
mod snowflake;

use std::env::set_var;
use std::process::exit;
use std::sync::Mutex;

use failure::Error;
use iron::prelude::*;
use iron::typemap::Key;
use logger::Logger;
use persistent::Read;
use rusqlite::Connection;
use structopt::StructOpt;

use options::Options;

fn main() {
    set_var("RUST_BACKTRACE", "1");

    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose + 2)
        .init()
        .expect("Failed to start logger");

    if let Err(err) = run(options) {
        error!("{}", err);
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> {
    let mut db = Connection::open(options.database)?;
    create_table(&mut db)?;

    let mut chain = Chain::new(router! {
        index:    get  "/"    => handlers::index,
        upload:   post "/"    => handlers::upload,
        download: get  "/:id" => handlers::download,
    });
    chain.link(Logger::new(None));
    chain.link(Read::<DB>::both(Mutex::new(db)));
    chain.link(Read::<MaxFileSize>::both(options.max_file_size));

    Iron::new(chain).http((options.addr, options.port))?;
    Ok(())
}

fn create_table(conn: &mut Connection) -> Result<(), Error> {
    let db = conn.transaction()?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS 'paste-acm' (
             id   INTEGER PRIMARY KEY,
             data TEXT NOT NULL
         )",
        &[],
    )?;

    db.commit()?;
    Ok(())
}

/// A key for the database connection.
enum DB {}

impl Key for DB {
    type Value = Mutex<Connection>;
}

/// A key for the maximum file size.
enum MaxFileSize {}

impl Key for MaxFileSize {
    type Value = usize;
}
