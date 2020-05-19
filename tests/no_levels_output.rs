use log::*;

#[test]
fn no_levels_output() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .show_level(false)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Trace, log::max_level())
}
