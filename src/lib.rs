// Copyright 2016-2018 Doug Goldstein
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/stderrlog/0.5.0")]

//! A simple logger to provide symantics similar to what is expected
//! of most UNIX utilities by logging to stderr and the higher the
//! verbosity the higher the log level. It supports the
//! ability to provide timestamps at different granularities. As
//! well as colorizing the different log levels.
//!
//! ## Simple Use Case
//!
//! ```rust
//! use log::*;
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
//! use log::*;
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
//! use log::*;
//! use docopt::Docopt;
//! use serde::Deserialize;
//!
//! const USAGE: &'static str = "
//! Usage: program [-q] [-v...]
//! ";
//!
//! #[derive(Debug, Deserialize)]
//! struct Args {
//!     flag_q: bool,
//!     flag_v: usize,
//! }
//!
//! fn main() {
//!     let args: Args = Docopt::new(USAGE)
//!                             .and_then(|d| d.deserialize())
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
//! use clap::{Arg, App, crate_version};
//! use log::*;
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
//! The 0.5.x versions of `stderrlog` aim to provide compatibility with
//! applications using `log` >= 0.4.11.
//!
//! ### Rust Compatibility
//!
//! `stderrlog` is serious about backwards compat. `stderrlog`
//! pins the minimum required version of Rust in the CI build.
//! Bumping the minimum version of Rust is a minor breaking
//! change and requires a minor version to be bumped.
//!
//! The minimum supported Rust version for this release is 1.31.0.
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
//! [large-example
//! crate](https://github.com/cardoe/stderrlog-rs/tree/master/examples/large-example)
//! under examples, you'll want to run the
//! following binaries to see all the examples:
//!
//! - `cargo run --bin large-example --`
//! - `cargo run --bin another --`
//! - `cargo run --bin yet --`

use atty::Stream;
use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub use termcolor::ColorChoice;
use thread_local::CachedThreadLocal;

/// State of the timestampping in the logger.
#[derive(Clone, Copy, Debug)]
pub enum Timestamp {
    /// Disable timestamping of log messages
    Off,
    /// Timestamp with second granularity
    Second,
    /// Timestamp with millisecond granularity
    Millisecond,
    /// Timestamp with microsecond granularity
    Microsecond,
    /// Timestamp with nanosecond granularity
    Nanosecond,
}

/// Provides a quick conversion of the following:
///
/// - "sec" -> `Timestamp::Second`
/// - "ms" -> `Timestamp::Millisecond`
/// - "us" -> `Timestamp::Microsecond`
/// - "ns" -> `Timestamp::Nanosecond`
/// - "none" | "off" -> `Timestamp::Off`
///
/// This is provided as a helper for argument parsers
impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" => Ok(Timestamp::Nanosecond),
            "ms" => Ok(Timestamp::Millisecond),
            "us" => Ok(Timestamp::Microsecond),
            "sec" => Ok(Timestamp::Second),
            "none" | "off" => Ok(Timestamp::Off),
            _ => Err("invalid value".into()),
        }
    }
}

/// Data specific to this logger
pub struct StdErrLog {
    verbosity: LevelFilter,
    quiet: bool,
    show_level: bool,
    timestamp: Timestamp,
    modules: Vec<String>,
    writer: CachedThreadLocal<RefCell<StandardStream>>,
    color_choice: ColorChoice,
    show_module_names: bool,
}

impl fmt::Debug for StdErrLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StdErrLog")
            .field("verbosity", &self.verbosity)
            .field("quiet", &self.quiet)
            .field("show_level", &self.show_level)
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
            ..*self
        }
    }
}

impl Log for StdErrLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level_filter() && self.includes_module(metadata.target())
    }

    fn log(&self, record: &Record) {
        // if logging isn't enabled for this level do a quick out
        if !self.enabled(record.metadata()) {
            return;
        }

        let writer = self
            .writer
            .get_or(|| RefCell::new(StandardStream::stderr(self.color_choice)));
        let writer = writer.borrow_mut();
        let mut writer = io::LineWriter::new(writer.lock());
        let color = match record.metadata().level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Magenta,
            Level::Info => Color::Yellow,
            Level::Debug => Color::Cyan,
            Level::Trace => Color::Blue,
        };
        {
            writer
                .get_mut()
                .set_color(ColorSpec::new().set_fg(Some(color)))
                .expect("failed to set color");
        }

        if self.show_module_names {
            let _ = write!(writer, "{}: ", record.metadata().target());
        }
        match self.timestamp {
            Timestamp::Second => {
                let fmt = "%Y-%m-%dT%H:%M:%S%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            }
            Timestamp::Millisecond => {
                let fmt = "%Y-%m-%dT%H:%M:%S%.3f%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            }
            Timestamp::Microsecond => {
                let fmt = "%Y-%m-%dT%H:%M:%S%.6f%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            }
            Timestamp::Nanosecond => {
                let fmt = "%Y-%m-%dT%H:%M:%S%.9f%:z";
                let _ = write!(writer, "{} - ", Local::now().format(fmt));
            }
            Timestamp::Off => {}
        }
        if self.show_level {
            let _ = write!(writer, "{} - ", record.level());
        }
        let _ = writeln!(writer, "{}", record.args());
        {
            writer.get_mut().reset().expect("failed to reset the color");
        }
    }

    fn flush(&self) {
        let writer = self
            .writer
            .get_or(|| RefCell::new(StandardStream::stderr(self.color_choice)));
        let mut writer = writer.borrow_mut();
        writer.flush().ok();
    }
}

impl StdErrLog {
    /// creates a new stderr logger
    pub fn new() -> StdErrLog {
        StdErrLog {
            verbosity: LevelFilter::Error,
            quiet: false,
            show_level: true,
            timestamp: Timestamp::Off,
            modules: Vec::new(),
            writer: CachedThreadLocal::new(),
            color_choice: ColorChoice::Auto,
            show_module_names: false,
        }
    }

    /// Sets the verbosity level of messages that will be displayed
    pub fn verbosity(&mut self, verbosity: usize) -> &mut StdErrLog {
        let log_lvl = match verbosity {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        self.verbosity = log_lvl;
        self
    }

    /// silence all output, no matter the value of verbosity
    pub fn quiet(&mut self, quiet: bool) -> &mut StdErrLog {
        self.quiet = quiet;
        self
    }

    /// Enables or disables the use of levels in log messages (default is true)
    pub fn show_level(&mut self, levels: bool) -> &mut StdErrLog {
        self.show_level = levels;
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

    /// Enables or disables the use of module names in log messages
    pub fn show_module_names(&mut self, show_module_names: bool) -> &mut StdErrLog {
        self.show_module_names = show_module_names;
        self
    }

    fn _module(&mut self, module: String) -> &mut StdErrLog {
        // If Ok, the module was already found
        if let Err(i) = self.modules.binary_search(&module) {
            // If a super-module of the current module already exists, don't insert this module
            if i == 0 || !is_submodule(&self.modules[i - 1], &module) {
                // Remove any submodules of the module we're inserting
                let submodule_count = self.modules[i..]
                    .iter()
                    .take_while(|possible_submodule| is_submodule(&module, possible_submodule))
                    .count();
                self.modules.drain(i..i + submodule_count);
                self.modules.insert(i, module);
            }
        }
        self
    }

    /// specifiy modules to allow to log to stderr
    pub fn modules<T: Into<String>, I: IntoIterator<Item = T>>(
        &mut self,
        modules: I,
    ) -> &mut StdErrLog {
        for module in modules {
            self.module(module);
        }
        self
    }

    fn log_level_filter(&self) -> LevelFilter {
        if self.quiet {
            LevelFilter::Off
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
        match self
            .modules
            .binary_search_by(|module| module.as_str().cmp(&module_path))
        {
            Ok(_) => {
                // Found exact module: return true
                true
            }
            Err(0) => {
                // if there's no item which would be located before module_path, no prefix is there
                false
            }
            Err(i) => is_submodule(&self.modules[i - 1], module_path),
        }
    }

    /// sets the the logger as active
    pub fn init(&mut self) -> Result<(), log::SetLoggerError> {
        /* if the user is using auto color choices then
         * detect if stderr is a tty, if it is continue
         * otherwise turn off colors by default
         */
        self.color_choice = match self.color_choice {
            ColorChoice::Auto => {
                if atty::is(Stream::Stderr) {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                }
            }
            other => other,
        };
        log::set_max_level(self.log_level_filter());
        log::set_boxed_logger(Box::new(self.clone()))
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
    parent.len() == possible_child.len()
        || possible_child.get(parent.len()..parent.len() + 2) == Some(b"::")
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
        super::new().module(module_path!()).init().unwrap();

        assert_eq!(log::Level::Error, log::max_level())
    }
}
