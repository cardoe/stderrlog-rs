//! Module filtering, checked through `Log::enabled` directly so these tests
//! don't need the global logger.

use log::{Level, Log, Metadata};
use stderrlog::StdErrLog;

fn enabled(logger: &StdErrLog, target: &str) -> bool {
    logger.enabled(
        &Metadata::builder()
            .level(Level::Error)
            .target(target)
            .build(),
    )
}

#[test]
fn no_modules_allows_everything() {
    let logger = StdErrLog::new();
    assert!(enabled(&logger, "anything"));
    assert!(enabled(&logger, "anything::at::all"));
}

#[test]
fn exact_and_submodules() {
    let mut logger = StdErrLog::new();
    logger.module("good");
    assert!(enabled(&logger, "good"));
    assert!(enabled(&logger, "good::foo"));
    assert!(enabled(&logger, "good::foo::bar"));
    assert!(!enabled(&logger, "bad"));
}

#[test]
fn submodule_only() {
    let mut logger = StdErrLog::new();
    logger.module("good::foo");
    assert!(enabled(&logger, "good::foo"));
    assert!(!enabled(&logger, "good::bar"));
    assert!(!enabled(&logger, "good"));
}

#[test]
fn shared_prefix_is_not_a_submodule() {
    let mut logger = StdErrLog::new();
    logger.module("module_inclusion::included");
    assert!(!enabled(&logger, "module_inclusion::included_not"));
    assert!(!enabled(&logger, "module_inclusion::includedx::a"));
}

#[test]
fn super_then_submodule() {
    let mut logger = StdErrLog::new();
    logger.module("a::b");
    logger.module("a::b::c");
    assert!(enabled(&logger, "a::b"));
    assert!(enabled(&logger, "a::b::c"));
    assert!(enabled(&logger, "a::b::d"));
}

#[test]
fn sub_then_supermodule() {
    let mut logger = StdErrLog::new();
    logger.module("a::b::c");
    logger.module("a::b");
    assert!(enabled(&logger, "a::b"));
    assert!(enabled(&logger, "a::b::c"));
    assert!(enabled(&logger, "a::b::d"));
}

#[test]
fn modules_iterator() {
    let mut logger = StdErrLog::new();
    logger.modules(vec!["x", "y::z"]);
    assert!(enabled(&logger, "x::anything"));
    assert!(enabled(&logger, "y::z"));
    assert!(!enabled(&logger, "y"));
}

#[test]
fn level_and_module_both_required() {
    let mut logger = StdErrLog::new();
    logger.module("good").verbosity(1);
    let warn = Metadata::builder()
        .level(Level::Warn)
        .target("good")
        .build();
    let info = Metadata::builder()
        .level(Level::Info)
        .target("good")
        .build();
    let warn_bad = Metadata::builder().level(Level::Warn).target("bad").build();
    assert!(logger.enabled(&warn));
    assert!(!logger.enabled(&info));
    assert!(!logger.enabled(&warn_bad));
}

#[test]
fn quiet_disables_everything() {
    let mut logger = StdErrLog::new();
    logger.verbosity(4).quiet(true);
    assert!(!enabled(&logger, "anything"));
}

#[test]
fn clone_preserves_config() {
    let mut logger = StdErrLog::new();
    logger.module("good").verbosity(2);
    let cloned = logger.clone();
    let info = Metadata::builder()
        .level(Level::Info)
        .target("good")
        .build();
    assert!(cloned.enabled(&info));
    assert!(!enabled(&cloned, "bad"));
}
