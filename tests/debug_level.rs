#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn debug_level() {
    stderrlog::new().verbosity(3).init().unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LogLevel::Debug, log::max_log_level())
}
