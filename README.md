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
