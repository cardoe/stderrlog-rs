[![Build status](https://travis-ci.org/cardoe/stderrlog-rs.svg?branch=master)](https://travis-ci.org/cardoe/stderrlog-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/no8slwtoy5va0w4g/branch/master?svg=true)](https://ci.appveyor.com/project/cardoe/stderrlog-rs/branch/master)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/cardoe/stderrlog-rs.svg)](http://isitmaintained.com/project/cardoe/stderrlog-rs "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/cardoe/stderrlog-rs.svg)](http://isitmaintained.com/project/cardoe/stderrlog-rs "Percentage of issues still open")
[![Rust version]( https://img.shields.io/badge/rust-1.48.0+-blue.svg)]()
[![Documentation](https://docs.rs/stderrlog/badge.svg)](https://docs.rs/stderrlog)
[![Latest version](https://img.shields.io/crates/v/stderrlog.svg)](https://crates.io/crates/stderrlog)
[![All downloads](https://img.shields.io/crates/d/stderrlog.svg)](https://crates.io/crates/stderrlog)
[![Downloads of latest version](https://img.shields.io/crates/dv/stderrlog.svg)](https://crates.io/crates/stderrlog)

Logger that aims to provide a simple case of
[env_logger](https://crates.io/crates/env_logger) that just
logs to `stderr` based on verbosity.

## Documentation

For a working example for [clap](https://crates.io/crates/clap), please see the
[crate level documentation](https://docs.rs/stderrlog/*/stderrlog/).

For example binaries showing how
[module level logging](https://github.com/cardoe/stderrlog-rs/tree/master/examples/large-example) works, please see the `large-example` crate in `examples/`.

## Supported Versions

* `stderrlog` 0.6.x supports
  1) Rust 1.48.0 and newer
  2) `log` >= 0.4.11
* `stderrlog` 0.5.x supports
  1) Rust 1.36.0 and newer
  2) `log` >= 0.4.11
* `stderrlog` 0.4.x supports
  1) Rust 1.16.0 and newer
  2) `log` >= 0.4.1
* `stderrlog` 0.3.x supports
  1) Rust 1.16.0 and newer
  2) `log` 0.3.x
* `stderrlog` 0.2.x supports
  1) Rust 1.13.0 and newer
  2) `log` >= 0.3.0,  < 0.3.9

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
stderrlog = "0.4"
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
