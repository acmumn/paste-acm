use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(raw(global_setting = "::structopt::clap::AppSettings::ColoredHelp"))]
pub struct Options {
    /// The address to serve on.
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0")]
    pub addr: IpAddr,

    /// The database file.
    #[structopt(short = "d", long = "db", default_value = "paste-acm.db",
                parse(from_os_str))]
    pub database: PathBuf,

    /// The maximum size of a file, in bytes.
    #[structopt(short = "m", long = "max-size", default_value = "1000000")]
    pub max_file_size: usize,

    /// The port to serve on.
    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,
}
