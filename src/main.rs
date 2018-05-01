extern crate base64;
extern crate byteorder;
extern crate diesel;
extern crate failure;
extern crate futures;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
#[macro_use]
extern crate router;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate tokio_core;

mod handlers;
mod options;
mod snowflake;

use std::process::exit;

use failure::Error;
use iron::prelude::*;
use logger::Logger;
use structopt::StructOpt;

use options::Options;

fn main() {
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
    let mut chain = Chain::new(router! {
        index:    get  "/"    => handlers::index,
        upload:   post "/"    => handlers::upload,
        download: get  "/:id" => handlers::download,
    });
    chain.link(Logger::new(None));

    Iron::new(chain).http((options.addr, options.port))?;
    Ok(())
}
