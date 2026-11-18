//! All integration tests live in this one binary (as in `jiff`), which keeps
//! link times down and lets helpers be shared without `#[allow(dead_code)]`.
//!
//! Tests that install the global logger run their bodies in a child process;
//! see `util::isolated`.

#[macro_use]
mod util;

mod format;
mod levels;
mod modules;
#[cfg(unix)]
mod pty;
#[cfg(feature = "timestamps")]
mod timestamps;
