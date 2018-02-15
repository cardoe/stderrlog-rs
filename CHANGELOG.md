# ChangeLog

## 0.3.0

### Changed

- bump minimum Rust version to 1.16.0
- allow all log 0.3.x releases
- fix situations where including `a::b` also included `a::baz`

## 0.2.4

### Changed

- pinned log to 0.3.8 or older to retain Rust 1.13.0 support

## 0.2.3

### Added

- clap example
- support timestamps with microsecond and nanosecond granularity

### Changed

- improved performance (https://github.com/cardoe/stderrlog-rs/pull/2)
- improved docopt example in README
- migrated from time to chrono

### Thanks

- Thanks to Zachary Dremann <dremann@gmail.com> for the performance
  improvements and the nudge to improve the docs.
