// Copyright 2016 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A simple logger to provide symantics similar to what is expected
//! of most UNIX utilities by logging to stderr and the higher the
//! verbosity the higher the log level
//!
//! ### Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate stderrlog;
//!
//! fn main() {
//!     stderrlog::new().init().unwrap();
//!
//!     info!("starting up");
//!
//!     // ...
//! }
//! ```

extern crate log;

use log::{LogLevelFilter, LogMetadata};
use std::io::Write;


#[derive(Clone, Copy, Debug)]
pub struct StdErrLog {
    verbosity: LogLevelFilter,
    quiet: bool,
}

impl log::Log for StdErrLog {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= log::max_log_level()
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            writeln!(&mut ::std::io::stderr(),
                     "{} - {}",
                     record.level(),
                     record.args())
                .unwrap();
        }
    }
}

impl StdErrLog {
    pub fn new() -> StdErrLog {
        StdErrLog {
            verbosity: LogLevelFilter::Error,
            quiet: false,
        }
    }

    /// Sets the verbosity level of messages that will be displayed
    pub fn verbosity(&mut self, verbosity: usize) -> &mut StdErrLog {
        let log_lvl = match verbosity {
            0 => LogLevelFilter::Error,
            1 => LogLevelFilter::Warn,
            2 => LogLevelFilter::Info,
            3 => LogLevelFilter::Debug,
            _ => LogLevelFilter::Trace,
        };

        self.verbosity = log_lvl;
        self
    }

    pub fn quiet(&mut self, quiet: bool) -> &mut StdErrLog {
        self.quiet = quiet;
        self
    }

    pub fn init(&self) -> Result<(), log::SetLoggerError> {

        log::set_logger(|max_log_level| {
            if self.quiet {
                max_log_level.set(LogLevelFilter::Off);
            } else {
                max_log_level.set(self.verbosity);
            }

            Box::new(self.clone())
        })
    }
}

pub fn new() -> StdErrLog {
    StdErrLog::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_default_level() {
        extern crate log;

        super::new().init().unwrap();

        assert_eq!(log::LogLevel::Error, log::max_log_level())
    }
}
