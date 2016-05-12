#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn info_level() {
    stderrlog::new().verbosity(2).init().unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LogLevel::Info, log::max_log_level())
}
