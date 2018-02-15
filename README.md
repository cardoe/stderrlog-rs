[![Build status](https://travis-ci.org/cardoe/stderrlog-rs.svg?branch=master)](https://travis-ci.org/cardoe/stderrlog-rs)
[![Rust version]( https://img.shields.io/badge/rust-1.16+-blue.svg)]()
[![Documentation](https://docs.rs/stderrlog/badge.svg)](https://docs.rs/stderrlog)
[![Latest version](https://img.shields.io/crates/v/stderrlog.svg)](https://crates.io/crates/stderrlog)
[![All downloads](https://img.shields.io/crates/d/stderrlog.svg)](https://crates.io/crates/stderrlog)
[![Downloads of latest version](https://img.shields.io/crates/dv/stderrlog.svg)](https://crates.io/crates/stderrlog)

Logger that aims to provide a simple case of
[env_logger](https://crates.io/crates/env_logger) that just
logs to `stderr` based on verbosity.

## Documentation

[Module documentation with examples](https://docs.rs/stderrlog/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
stderrlog = "0.2"
```

and this to your crate root:

```rust
extern crate stderrlog;
```

and this to your main():

```rust
stderrlog::new().verbosity(args.flag_v).quiet(args.flag_q).init().unwrap();
```

where your args struct is defined as:

```rust
struct Args {
    flag_v: usize,
    flag_q: bool,
    ...
}
```

## Docopt Example

```rust
extern crate docopt;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate stderrlog;

use docopt::Docopt;

const USAGE: &'static str = "
Usage: program [-q] [-v...]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_q: bool,
    flag_v: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    stderrlog::new()
            .module(module_path!())
            .quiet(args.flag_q)
            .verbosity(args.flag_v)
            .init()
            .unwrap();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");

    // ...
}
```

## clap Example

```rust
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate stderrlog;

use clap::{Arg, App};

fn main() {
    let m = App::new("stderrlog example")
        .version(crate_version!())
        .arg(Arg::with_name("verbosity")
             .short("v")
             .multiple(true)
             .help("Increase message verbosity"))
        .arg(Arg::with_name("quiet")
             .short("q")
             .help("Silence all output"))
        .get_matches();

    let verbose = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .init()
        .unwrap();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
```
