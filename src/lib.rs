// Copyright 2016 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A simple logger to provide symantics similar to what is expected
//! of most UNIX utilities by logging to stderr and the higher the
//! verbosity the higher the log level. It supports the
//! ability to provide timestamps at different granularities. As
//! well as colorizing the different log levels.
//!
//! ## Simple Use Case
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate stderrlog;
//!
//! fn main() {
//!     stderrlog::new().module(module_path!()).init().unwrap();
//!
//!     error!("some failure");
//!
//!     // ...
//! }
//! ```
//!
//! # StructOpt Example
//!
//! ```
//! #[macro_use]
//! extern crate log;
//! extern crate stderrlog;
//! #[macro_use]
//! extern crate structopt;
//!
//! use structopt::StructOpt;
//!
//! /// A StructOpt example
//! #[derive(StructOpt, Debug)]
//! #[structopt()]
//! struct Opt {
//!     /// Silence all output
//!     #[structopt(short = "q", long = "quiet")]
//!     quiet: bool,
//!     /// Verbose mode (-v, -vv, -vvv, etc)
//!     #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
//!     verbose: usize,
//!     /// Timestamp (sec, ms, ns, none)
//!     #[structopt(short = "t", long = "timestamp")]
//!     ts: Option<stderrlog::Timestamp>,
//! }
//!
//! fn main() {
//!     let opt = Opt::from_args();
//!
//!     stderrlog::new()
//!         .module(module_path!())
//!         .quiet(opt.quiet)
//!         .verbosity(opt.verbose)
//!         .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
//!         .init()
//!         .unwrap();
//!     trace!("trace message");
//!     debug!("debug message");
//!     info!("info message");
//!     warn!("warn message");
//!     error!("error message");
//! }
//! ```
//!
//! ## docopt Example
//!
//! ```rust
//! extern crate docopt;
//! #[macro_use]
//! extern crate log;
//! extern crate rustc_serialize;
//! extern crate stderrlog;
//!
//! use docopt::Docopt;
//!
//! const USAGE: &'static str = "
//! Usage: program [-q] [-v...]
//! ";
//!
//! #[derive(Debug, RustcDecodable)]
//! struct Args {
//!     flag_q: bool,
//!     flag_v: usize,
//! }
//!
//! fn main() {
//!     let args: Args = Docopt::new(USAGE)
//!                             .and_then(|d| d.decode())
//!                             .unwrap_or_else(|e| e.exit());
//!
//!     stderrlog::new()
//!             .module(module_path!())
//!             .quiet(args.flag_q)
//!             .timestamp(stderrlog::Timestamp::Second)
//!             .verbosity(args.flag_v)
//!             .init()
//!             .unwrap();
//!     trace!("trace message");
//!     debug!("debug message");
//!     info!("info message");
//!     warn!("warn message");
//!     error!("error message");
//!
//!     // ...
//! }
//! ```
//!
//! # clap Example
//!
//! ```
//! #[macro_use]
//! extern crate clap;
//! #[macro_use]
//! extern crate log;
//! extern crate stderrlog;
//!
//! use clap::{Arg, App};
//! use std::str::FromStr;
//!
//! fn main() {
//!     let m = App::new("stderrlog example")
//!         .version(crate_version!())
//!         .arg(Arg::with_name("verbosity")
//!              .short("v")
//!              .multiple(true)
//!              .help("Increase message verbosity"))
//!         .arg(Arg::with_name("quiet")
//!              .short("q")
//!              .help("Silence all output"))
//!         .arg(Arg::with_name("timestamp")
//!              .short("t")
//!              .help("prepend log lines with a timestamp")
//!              .takes_value(true)
//!              .possible_values(&["none", "sec", "ms", "ns"]))
//!         .get_matches();
//!
//!     let verbose = m.occurrences_of("verbosity") as usize;
//!     let quiet = m.is_present("quiet");
//!     let ts = m.value_of("timestamp").map(|v| {
//!         stderrlog::Timestamp::from_str(v).unwrap_or_else(|_| {
//!             clap::Error {
//!                 message: "invalid value for 'timestamp'".into(),
//!                 kind: clap::ErrorKind::InvalidValue,
//!                 info: None,
//!             }.exit()
//!         })
//!     }).unwrap_or(stderrlog::Timestamp::Off);
//!
//!     stderrlog::new()
//!         .module(module_path!())
//!         .quiet(quiet)
//!         .verbosity(verbose)
//!         .timestamp(ts)
//!         .init()
//!         .unwrap();
//!     trace!("trace message");
//!     debug!("debug message");
//!     info!("info message");
//!     warn!("warn message");
//!     error!("error message");
//! }
//! ```
//!
//! ### `log` Compatibility
//!
//! The 0.3.x versions of `stderrlog` aim to provide compatibility with
//! applications using `log` 0.3.x
//!
//! ### Rust Compatibility
//!
//! `stderrlog` is serious about backwards compat. `stderrlog`
//! pins the minimum required version of Rust in the CI build.
//! Bumping the minimum version of Rust is a minor breaking
//! change and requires a minor version to be bumped.
//!
//! The minimum supported Rust version for this release is 1.16.0.
//!
//! ### Module Level Logging
//!
//! `stderrlog` has the ability to limit the components which can log.
//! Many crates use [log](https://docs.rs/log/*/log/) but you may not
//! want their output in your application. For example
//! [hyper](https://docs.rs/hyper/*/hyper/) makes heavy use of log but
//! when your application receives `-vvvvv` to enable the `trace!()`
//! messages you don't want the output of `hyper`'s `trace!()` level.
//!
//! To support this `stderrlog` includes a `module()` method allowing
//! you to specify the modules that are allowed to log. The examples
//! above use the `module_path!()` macro to enable logging only for
//! the binary itself but none of its dependencies. To enable logging
//! from extra crates just add another call to `module()` with the
//! name of the crate. To enable logging for only a module within
//! that crate specifiy `crate::module` to `module()`. crates and
//! modules will be named the same way would would include them in
//! source code with `use` (e.g. `some-crate` would be `some_crate`).
//!
//! For a good example of how the module level logging works see the
//! large-example project under examples, you'll want to run the
//! following binaries to see all the examples:
//! - cargo run --bin large-example --
//! - cargo run --bin another --
//! - cargo run --bin yet --

extern crate chrono;
extern crate log;
extern crate termcolor;
extern crate thread_local;

use chrono::Local;
use log::{LogLevel, LogLevelFilter, LogMetadata};
use std::cell::RefCell;
use std::io::{self, Write};
use std::fmt;
use std::str::FromStr;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use thread_local::CachedThreadLocal;

pub use termcolor::ColorChoice;

/// State of the timestampping in the logger.
#[derive(Clone, Copy, Debug)]
pub enum Timestamp {
    /// Disable timestamping of log messages
    Off,
    /// Timestamp with second granularity
    Second,
    /// Timestamp with microsecond granularity
    Microsecond,
    /// Timestamp with nanosecond granularity
    Nanosecond,
}

/// Provides a quick conversion of the following:
///
/// - "sec" -> `Timestamp::Second`
/// - "ms" -> `Timestamp::Microsecond`
/// - "ns" -> `Timestamp::Nanosecond`
/// - "none" | "off" -> `Timestamp::Off`
///
/// This is provided as a helper for argument parsers
impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" => Ok(Timestamp::Nanosecond),
            "ms" => Ok(Timestamp::Microsecond),
            "sec" => Ok(Timestamp::Second),
            "none" | "off" => Ok(Timestamp::Off),
            _ => Err("invalid value".into()),
        }
    }
}

/// Data specific to this logger
pub struct StdErrLog {
    verbosity: LogLevelFilter,
    quiet: bool,
    timestamp: Timestamp,
    modules: Vec<String>,
    writer: CachedThreadLocal<RefCell<io::LineWriter<StandardStream>>>,
    color_choice: ColorChoice,
}

impl fmt::Debug for StdErrLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StdErrLog")
            .field("verbosity", &self.verbosity)
            .field("quiet", &self.quiet)
            .field("timestamp", &self.timestamp)
            .field("modules", &self.modules)
            .field("writer", &"stderr")
            .field("color_choice", &self.color_choice)
            .finish()
    }
}

impl Clone for StdErrLog {
    fn clone(&self) -> StdErrLog {
        StdErrLog {
            modules: self.modules.clone(),
            writer: CachedThreadLocal::new(),
            .. *self
        }
    }
}

impl log::Log for StdErrLog {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.log_level_filter() && self.includes_module(metadata.target())
    }

    fn log(&self, record: &log::LogRecord) {

        // if logging isn't enabled for this level do a quick out
        if !self.enabled(record.metadata()) {
            return;
        }

        let writer =
            self.writer.get_or(|| Box::new(RefCell::new(io::LineWriter::new(StandardStream::stderr(self.color_choice)))));
        let mut writer = writer.borrow_mut();
        let color = match record.metadata().level() {
            LogLevel::Error => Color::Red,
            LogLevel::Warn => Color::Magenta,
            LogLevel::Info => Color::Yellow,
            LogLevel::Debug => Color::Cyan,
            LogLevel::Trace => Color::Blue,
        };
        {
            writer.get_mut().set_color(ColorSpec::new().set_fg(Some(color))).expect("failed to set color");
        }
        match self.timestamp {
            Timestamp::Second => {
                let fmt = "%Y-%m-%dT%H:%M:%S%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            },
            Timestamp::Microsecond => {
                let fmt = "%Y-%m-%dT%H:%M:%S%.6f%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            },
            Timestamp::Nanosecond => {
                let fmt = "%Y-%m-%dT%H:%M:%S%.9f%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            },
            Timestamp::Off => {},
        }
        let _ = writeln!(writer, "{} - {}", record.level(), record.args());
        {
            writer.get_mut().reset().expect("failed to reset the color");
        }
    }
}

impl StdErrLog {
    /// creates a new stderr logger
    pub fn new() -> StdErrLog {
        StdErrLog {
            verbosity: LogLevelFilter::Error,
            quiet: false,
            timestamp: Timestamp::Off,
            modules: Vec::new(),
            writer: CachedThreadLocal::new(),
            color_choice: ColorChoice::Auto,
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

    /// silence all output, no matter the value of verbosity
    pub fn quiet(&mut self, quiet: bool) -> &mut StdErrLog {
        self.quiet = quiet;
        self
    }

    /// Enables or disables the use of timestamps in log messages
    pub fn timestamp(&mut self, timestamp: Timestamp) -> &mut StdErrLog {
        self.timestamp = timestamp;
        self
    }

    /// Enables or disables the use of color in log messages
    pub fn color(&mut self, choice: ColorChoice) -> &mut StdErrLog {
        self.color_choice = choice;
        self
    }

    /// specify a module to allow to log to stderr
    pub fn module<T: Into<String>>(&mut self, module: T) -> &mut StdErrLog {
        self._module(module.into())
    }

    fn _module(&mut self, module: String) -> &mut StdErrLog {
        // If Ok, the module was already found
        if let Err(i) = self.modules.binary_search(&module) {
            // If a super-module of the current module already exists, don't insert this module
            if i == 0 || !is_submodule(&self.modules[i - 1], &module) {
                // Remove any submodules of the module we're inserting
                let submodule_count = self.modules[i..]
                    .iter()
                    .take_while(|possible_submodule|
                        is_submodule(&module, possible_submodule)
                    )
                    .count();
                self.modules.drain(i..i+submodule_count);
                self.modules.insert(i, module);
            }
        }
        self
    }

    /// specifiy modules to allow to log to stderr
    pub fn modules<T: Into<String>, I: IntoIterator<Item = T>>(&mut self,
                                                               modules: I)
                                                               -> &mut StdErrLog {
        for module in modules {
            self.module(module);
        }
        self
    }

    fn log_level_filter(&self) -> LogLevelFilter {
        if self.quiet {
            LogLevelFilter::Off
        } else {
            self.verbosity
        }
    }

    fn includes_module(&self, module_path: &str) -> bool {
        // If modules is empty, include all module paths
        if self.modules.is_empty() {
            return true;
        }
        // if a prefix of module_path is in `self.modules`, it must
        // be located at the first location before
        // where module_path would be.
        match self.modules.binary_search_by(|module| module.as_str().cmp(&module_path)) {
            Ok(_) => {
                // Found exact module: return true
                true
            }
            Err(0) => {
                // if there's no item which would be located before module_path, no prefix is there
                false
            }
            Err(i) => {
                is_submodule(&self.modules[i - 1], module_path)
            }
        }
    }

    /// sets the the logger as active
    pub fn init(&self) -> Result<(), log::SetLoggerError> {
        log::set_logger(|max_log_level| {
                            max_log_level.set(self.log_level_filter());

                            Box::new(self.clone())
                        })
    }
}

impl Default for StdErrLog {
    fn default() -> Self {
        StdErrLog::new()
    }
}

/// creates a new stderr logger
pub fn new() -> StdErrLog {
    StdErrLog::new()
}

fn is_submodule(parent: &str, possible_child: &str) -> bool {
    // Treat as bytes, because we'll be doing slicing, and we only care about ':' chars
    let parent = parent.as_bytes();
    let possible_child = possible_child.as_bytes();

    // a longer module path cannot be a parent of a shorter module path
    if parent.len() > possible_child.len() {
        return false;
    }

    // If the path up to the parent isn't the same as the child,
    if parent != &possible_child[..parent.len()] {
        return false;
    }

    // Either the path is exactly the same, or the sub module should have a "::" after
    // the length of the parent path. This prevents things like 'a::bad' being considered
    // a submodule of 'a::b'
    parent.len() == possible_child.len() ||
        possible_child.get(parent.len()..parent.len() + 2) == Some(b"::")
}

#[cfg(test)]
mod tests {
    use super::is_submodule;

    #[test]
    fn submodule() {
        assert!(is_submodule("a", "a::b::c::d"));
        assert!(is_submodule("a::b::c", "a::b::c::d"));
        assert!(is_submodule("a::b::c", "a::b::c"));
        assert!(!is_submodule("a::b::c", "a::bad::c"));
        assert!(!is_submodule("a::b::c", "a::b::cab"));
        assert!(!is_submodule("a::b::c", "a::b::cab::d"));
        assert!(!is_submodule("a::b::c", "a::b"));
        assert!(!is_submodule("a::b::c", "a::bad"));
    }

    #[test]
    fn test_default_level() {
        extern crate log;

        super::new().module(module_path!()).init().unwrap();

        assert_eq!(log::LogLevel::Error, log::max_log_level())
    }
}
