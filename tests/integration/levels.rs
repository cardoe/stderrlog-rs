//! Verbosity handling, as seen through the global logger.

use log::{Level, LevelFilter};

const ERROR: &str = "ERROR error msg\n";
const WARN: &str = "WARN warn msg\n";
const INFO: &str = "INFO info msg\n";
const DEBUG: &str = "DEBUG debug msg\n";
const TRACE: &str = "TRACE trace msg\n";

#[test]
fn default_is_error() {
    let stderr = isolated!(default_is_error, {
        stderrlog::new().init().unwrap();
        assert_eq!(log::max_level(), LevelFilter::Error);
        log_all!();
    });
    assert_eq!(stderr, ERROR);
}

/// Each case gets its own `#[test]` (and so its own child process), since
/// `init` can only succeed once per process.
macro_rules! verbosity_test {
    ($name:ident, $verbosity:expr, $max:expr, [$($line:expr),*]) => {
        #[test]
        fn $name() {
            let stderr = isolated!($name, {
                stderrlog::new().verbosity($verbosity).init().unwrap();
                assert_eq!(log::max_level(), $max);
                log_all!();
            });
            let expected: &[&str] = &[$($line),*];
            assert_eq!(stderr, expected.concat());
        }
    };
}

verbosity_test!(verbosity_0, 0, LevelFilter::Error, [ERROR]);
verbosity_test!(verbosity_1, 1, LevelFilter::Warn, [ERROR, WARN]);
verbosity_test!(verbosity_2, 2, LevelFilter::Info, [ERROR, WARN, INFO]);
verbosity_test!(
    verbosity_3,
    3,
    LevelFilter::Debug,
    [ERROR, WARN, INFO, DEBUG]
);
verbosity_test!(
    verbosity_4,
    4,
    LevelFilter::Trace,
    [ERROR, WARN, INFO, DEBUG, TRACE]
);
verbosity_test!(
    verbosity_saturates,
    50,
    LevelFilter::Trace,
    [ERROR, WARN, INFO, DEBUG, TRACE]
);

verbosity_test!(level_warn, Level::Warn, LevelFilter::Warn, [ERROR, WARN]);
verbosity_test!(
    level_debug,
    Level::Debug,
    LevelFilter::Debug,
    [ERROR, WARN, INFO, DEBUG]
);

verbosity_test!(level_filter_off, LevelFilter::Off, LevelFilter::Off, []);
verbosity_test!(
    level_filter_info,
    LevelFilter::Info,
    LevelFilter::Info,
    [ERROR, WARN, INFO]
);
verbosity_test!(
    level_filter_trace,
    LevelFilter::Trace,
    LevelFilter::Trace,
    [ERROR, WARN, INFO, DEBUG, TRACE]
);

#[test]
fn quiet_overrides_verbosity() {
    let stderr = isolated!(quiet_overrides_verbosity, {
        stderrlog::new().verbosity(4).quiet(true).init().unwrap();
        assert_eq!(log::max_level(), LevelFilter::Off);
        log_all!();
    });
    assert_eq!(stderr, "");
}

#[test]
fn quiet_false_is_noop() {
    let stderr = isolated!(quiet_false_is_noop, {
        stderrlog::new().verbosity(1).quiet(false).init().unwrap();
        log_all!();
    });
    assert_eq!(stderr, [ERROR, WARN].concat());
}

#[test]
fn init_twice_fails() {
    let stderr = isolated!(init_twice_fails, {
        stderrlog::new().init().unwrap();
        assert!(stderrlog::new().verbosity(4).init().is_err());
        // The first logger stays in charge.
        log_all!();
    });
    assert_eq!(stderr, ERROR);
}
