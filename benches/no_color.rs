#![feature(test)]
extern crate test;
extern crate stderrlog;
extern crate libc;

#[macro_use]
extern crate log;
extern crate termcolor;

mod util;
mod common;

use std::sync;

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

fn init_logger() {
    INIT_LOGGER.call_once(|| {
        stderrlog::StdErrLog::new()
            .timestamp(stderrlog::Timestamp::Second)
            .verbosity(10)
            .color(termcolor::ColorChoice::Never)
            .module(module_path!())
            .init()
            .unwrap();
    });
}
