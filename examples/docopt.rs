// Copyright 2016-2018 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use docopt::Docopt;
use log::*;
use serde::Deserialize;
use std::env;

const USAGE: &'static str = "
Usage:
  stderrtest [-q] [-v...]
";

#[derive(Deserialize)]
struct Args {
    flag_v: usize,
    flag_q: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args().into_iter()).deserialize())
        .unwrap_or_else(|e| e.exit());

    stderrlog::new()
        .verbosity(args.flag_v)
        .quiet(args.flag_q)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");
}
