// Copyright 2017-2018 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{crate_version, App, Arg};
use log::*;
use std::str::FromStr;

fn main() {
    let m = App::new("stderrlog example")
        .version(crate_version!())
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Silence all output"),
        )
        .arg(
            Arg::with_name("timestamp")
                .short("t")
                .help("prepend log lines with a timestamp")
                .takes_value(true)
                .possible_values(&["none", "sec", "ms", "us", "ns"]),
        )
        .get_matches();

    let verbose = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");
    let ts = m
        .value_of("timestamp")
        .map(|v| {
            stderrlog::Timestamp::from_str(v).unwrap_or_else(|_| {
                clap::Error {
                    message: "invalid value for 'timestamp'".into(),
                    kind: clap::ErrorKind::InvalidValue,
                    info: None,
                }
                .exit()
            })
        })
        .unwrap_or(stderrlog::Timestamp::Off);

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .timestamp(ts)
        .init()
        .unwrap();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
