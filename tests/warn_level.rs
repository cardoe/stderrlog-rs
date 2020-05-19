use log::*;

#[test]
fn warn_level() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(1)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Warn, log::max_level())
}
