#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn error_level() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(0)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Error, log::max_level())
}
