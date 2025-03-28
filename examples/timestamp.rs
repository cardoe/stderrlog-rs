use clap::{crate_version, Arg, ArgAction, Command};
use log::*;

fn main() {
    let m = Command::new("stderrlog example")
        .version(crate_version!())
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
                .help("Silence all output"),
        )
        .arg(
            Arg::new("timestamp")
                .short('t')
                .help("prepend log lines with a timestamp")
                .num_args(1)
                .value_parser(["none", "sec", "ms", "us", "ns"]),
        )
        .get_matches();

    let verbose = m.get_count("verbosity") as usize;
    let quiet = m.get_flag("quiet");
    let ts = match m.get_one::<String>("timestamp").map(|s| s.as_str()) {
        Some("ns") => stderrlog::Timestamp::Nanosecond,
        Some("ms") => stderrlog::Timestamp::Millisecond,
        Some("us") => stderrlog::Timestamp::Microsecond,
        Some("sec") => stderrlog::Timestamp::Second,
        Some("none") | None => stderrlog::Timestamp::Off,
        Some(_) => clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "invalid value for 'timestamp'",
        )
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
