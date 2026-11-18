//! Helpers for running tests that install the global logger.
//!
//! `log` only allows a logger to be installed once per process, and
//! `log::max_level()` is process-global. To test `StdErrLog::init()` and the
//! bytes it actually writes to stderr, each such test re-executes the test
//! binary, asking libtest to run only that one test. In the child, the test
//! body installs the logger and logs; in the parent, we capture the child's
//! stderr and make assertions on it.

use std::env;
use std::process::{Command, Stdio};

/// The environment variable used to tell a re-executed test binary which test
/// should run its body (instead of spawning yet another child).
const CHILD_ENV: &str = "STDERRLOG_TEST_CHILD";

/// Run `body` in a fresh child process and return everything it wrote to
/// stderr. Returns `None` when called from inside the child, in which case the
/// caller should return immediately.
///
/// Prefer the [`isolated!`] macro, which fills in `module_path` and `name`.
pub fn isolated(module_path: &str, name: &str, body: impl FnOnce()) -> Option<String> {
    // libtest names tests by module path without the crate name.
    let test_name = match module_path.split_once("::") {
        Some((_crate, rest)) => format!("{}::{}", rest, name),
        None => name.to_string(),
    };

    if env::var(CHILD_ENV).ok().as_deref() == Some(test_name.as_str()) {
        body();
        return None;
    }

    let exe = env::current_exe().expect("current test executable");
    let output = Command::new(exe)
        .args([
            test_name.as_str(),
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD_ENV, &test_name)
        .stdin(Stdio::null())
        .output()
        .expect("failed to spawn child test process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8(output.stderr).expect("stderr is valid UTF-8");
    assert!(
        output.status.success(),
        "child test {} failed: {}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        test_name,
        output.status,
        stdout,
        stderr,
    );
    // Guard against the filter silently matching nothing, which would make
    // every assertion on an empty stderr vacuous.
    assert!(
        stdout.contains("1 passed"),
        "child did not run exactly one test for {}\n--- stdout ---\n{}",
        test_name,
        stdout,
    );
    Some(stderr)
}

/// Run a block in an isolated child process, evaluating to its stderr.
///
/// The first argument must be the name of the enclosing `#[test]` function.
macro_rules! isolated {
    ($name:ident, $body:block) => {
        match crate::util::isolated(module_path!(), stringify!($name), || $body) {
            Some(stderr) => stderr,
            None => return,
        }
    };
}

/// Emit one message at every level from the caller's module.
macro_rules! log_all {
    () => {{
        log::error!("error msg");
        log::warn!("warn msg");
        log::info!("info msg");
        log::debug!("debug msg");
        log::trace!("trace msg");
    }};
}
