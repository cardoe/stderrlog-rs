#![feature(test)]

mod common;
mod util;

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
