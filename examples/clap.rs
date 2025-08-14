// Copyright 2017-2018 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{crate_version, Arg, ArgAction, Command};
use log::*;
use std::str::FromStr;

fn main() {
    let m = Command::new("stderrlog example")
        .version(crate_version!())
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
                .help("Silence all output"),
        )
        .arg(
            Arg::new("timestamp")
                .short('t')
                .help("prepend log lines with a timestamp")
                .num_args(1)
                .value_parser(["none", "sec", "ms", "us", "ns"]),
        )
        .get_matches();

    let verbose = m.get_count("verbosity") as usize;
    let quiet = m.get_flag("quiet");
    let ts = m
        .get_one::<String>("timestamp")
        .map(|v| {
            stderrlog::Timestamp::from_str(v).unwrap_or_else(|_| {
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidValue,
                    "invalid value for 'timestamp'",
                )
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
