#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate stderrlog;

use clap::{Arg, App};

fn main() {
    let m = App::new("stderrlog example")
        .version(crate_version!())
        .arg(Arg::with_name("verbosity")
             .short("v")
             .multiple(true)
             .help("Increase message verbosity"))
        .arg(Arg::with_name("quiet")
             .short("q")
             .help("Silence all output"))
        .get_matches();

    let verbose = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
