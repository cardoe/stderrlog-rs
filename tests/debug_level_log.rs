use log::*;

#[test]
fn debug_level_log() {
    stderrlog::new()
        .module(module_path!())
        .timestamp(stderrlog::Timestamp::Nanosecond)
        .verbosity(log::Level::Debug)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Debug, log::max_level())
}
