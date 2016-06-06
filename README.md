stderrlog
=========

Logger that logs to stderr based on verbosity specified

[![Build Status](https://travis-ci.org/cardoe/stderrlog-rs.svg?branch=master)](https://travis-ci.org/cardoe/stderrlog-rs)

### Documentation

[Module documentation with examples](https://cardoe.github.io/stderrlog-rs/)

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
stderrlog = "*"
```

and this to your crate root:

```rust
extern crate stderrlog;
```

and this to your main():

```rust
stderrlog::new().verbosity(args.flag_v).quiet(args.flag_q).init().unwrap();
```

where your getopt/Docopt args are defined as:

```rust
struct {
    flag_v: usize,
    flag_q: bool,
    ...
}
```

### Example

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
    info!("starting up");

    // ...
}
