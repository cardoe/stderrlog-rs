extern crate docopt;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate stderrlog;

use docopt::Docopt;
use std::env;

const USAGE: &'static str = "
Usage:
  stderrtest [-q] [-v...]
";

#[derive(RustcDecodable)]
struct Args {
    flag_v: usize,
    flag_q: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args().into_iter()).decode())
        .unwrap_or_else(|e| e.exit());

    stderrlog::new().verbosity(args.flag_v).quiet(args.flag_q).init().unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");
}
