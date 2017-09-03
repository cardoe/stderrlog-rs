#![feature(test)]
extern crate test;
extern crate stderrlog;
extern crate libc;

#[macro_use]
extern crate log;

mod util;
mod common;

use std::sync;

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

fn init_logger() {
    INIT_LOGGER.call_once(|| {
        stderrlog::StdErrLog::new()
            .timestamp(stderrlog::Timestamp::Second)
            .verbosity(10)
            .quiet(true)
            .module(module_path!())
            .init().unwrap();
    });
}