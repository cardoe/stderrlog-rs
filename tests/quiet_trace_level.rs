#[macro_use]
extern crate log;
extern crate stderrlog;

#[test]
fn quiet_trace_level() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .quiet(true)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LevelFilter::Off, log::max_level())
}
