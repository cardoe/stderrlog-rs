#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn error_level() {
    stderrlog::new().verbosity(0).init().unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LogLevel::Error, log::max_log_level())
}
