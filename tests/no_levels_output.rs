#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn no_levels_output() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .levels(false)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Trace, log::max_level())
}
