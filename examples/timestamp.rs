use clap::{crate_version, App, Arg};
use log::*;

fn main() {
    let m = App::new("stderrlog example")
        .version(crate_version!())
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Silence all output"),
        )
        .arg(
            Arg::with_name("timestamp")
                .short("t")
                .help("prepend log lines with a timestamp")
                .takes_value(true)
                .possible_values(&["none", "sec", "ms", "us", "ns"]),
        )
        .get_matches();

    let verbose = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");
    let ts = match m.value_of("timestamp") {
        Some("ns") => stderrlog::Timestamp::Nanosecond,
        Some("ms") => stderrlog::Timestamp::Millisecond,
        Some("us") => stderrlog::Timestamp::Microsecond,
        Some("sec") => stderrlog::Timestamp::Second,
        Some("none") | None => stderrlog::Timestamp::Off,
        Some(_) => clap::Error {
            message: "invalid value for 'timestamp'".into(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        }
        .exit(),
    };

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .timestamp(ts)
        .init()
        .unwrap();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
