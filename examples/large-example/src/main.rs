// Copyright 2018 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use large_example::bar;

use clap::{ArgAction, Parser};
use log::*;

/// A basic example
#[derive(Debug, Parser)]
struct Opt {
    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short = 'v', long = "verbose", action = ArgAction::Count)]
    verbose: u8,
    /// Allow module to log
    #[clap(short = 'm', long = "module")]
    modules: Vec<String>,
    /// Timestamp (sec, ms, us, ns, none)
    #[clap(short = 't', long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
}

fn main() {
    let opt = Opt::parse();

    stderrlog::new()
        .quiet(opt.quiet)
        .verbosity(opt.verbose as usize)
        .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .modules(opt.modules)
        .init()
        .unwrap();

    large_example::libfn();
    bar::barfn();

    println!("back to main()");
    trace!("{} - trace message", module_path!());
    debug!("{} - debug message", module_path!());
    info!("{} - info message", module_path!());
    warn!("{} - warn message", module_path!());
    error!("{} - error message", module_path!());

    println!("Some modules to try:");
    println!("  -m large_example");
    println!("  -m large_example::foo");
    println!("  -m large_example::bar");
}
