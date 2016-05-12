#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn trace_level() {
    stderrlog::new().verbosity(4).init().unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LogLevel::Trace, log::max_log_level())
}
