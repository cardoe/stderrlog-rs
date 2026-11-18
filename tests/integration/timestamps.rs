//! Timestamp prefixes. Only built with the `timestamps` feature.

use std::str::FromStr;

use stderrlog::Timestamp;

/// Check that `stderr` is a single `<timestamp> - ERROR msg` line whose
/// timestamp has `digits` fractional digits and falls within the time the
/// child process ran.
fn check(stderr: &str, digits: usize, before: jiff::Timestamp, after: jiff::Timestamp) {
    let line = stderr.strip_suffix('\n').expect("trailing newline");
    assert!(!line.contains('\n'), "expected one line: {:?}", stderr);
    let (ts, rest) = line.split_once(" - ").expect("timestamp separator");
    assert_eq!(rest, "ERROR error msg");

    // RFC 3339 with an explicit `+HH:MM`/`-HH:MM` offset.
    let parsed: jiff::Timestamp = ts.parse().unwrap_or_else(|e| panic!("{:?}: {}", ts, e));
    let offset_start = ts.len() - "+00:00".len();
    assert!(
        ts[offset_start..].starts_with(['+', '-']),
        "missing offset: {:?}",
        ts
    );

    let fraction = ts[..offset_start].split_once('.').map(|(_, f)| f);
    match fraction {
        None => assert_eq!(digits, 0, "missing fraction: {:?}", ts),
        Some(f) => {
            assert_eq!(f.len(), digits, "fraction width: {:?}", ts);
            assert!(f.bytes().all(|b| b.is_ascii_digit()), "{:?}", ts);
        }
    }

    // Truncation to whole seconds can put the timestamp up to 1s before
    // `before`.
    let slack = jiff::SignedDuration::from_secs(1);
    assert!(parsed >= before - slack, "{} < {}", parsed, before);
    assert!(parsed <= after, "{} > {}", parsed, after);
}

macro_rules! timestamp_test {
    ($name:ident, $ts:expr, $digits:expr) => {
        #[test]
        fn $name() {
            let before = jiff::Timestamp::now();
            let stderr = isolated!($name, {
                stderrlog::new().timestamp($ts).init().unwrap();
                log::error!("error msg");
            });
            check(&stderr, $digits, before, jiff::Timestamp::now());
        }
    };
}

timestamp_test!(second, Timestamp::Second, 0);
timestamp_test!(millisecond, Timestamp::Millisecond, 3);
timestamp_test!(microsecond, Timestamp::Microsecond, 6);
timestamp_test!(nanosecond, Timestamp::Nanosecond, 9);

#[test]
fn off() {
    let stderr = isolated!(off, {
        stderrlog::new().timestamp(Timestamp::Off).init().unwrap();
        log::error!("error msg");
    });
    assert_eq!(stderr, "ERROR error msg\n");
}

#[test]
fn with_module_names_and_no_level() {
    let stderr = isolated!(with_module_names_and_no_level, {
        stderrlog::new()
            .timestamp(Timestamp::Second)
            .show_module_names(true)
            .show_level(false)
            .init()
            .unwrap();
        log::error!("error msg");
    });
    // Module name comes first, then the timestamp, then the message.
    let rest = stderr
        .strip_prefix("integration::timestamps: ")
        .unwrap_or_else(|| panic!("{:?}", stderr));
    let (_ts, msg) = rest.split_once(" - ").expect("timestamp separator");
    assert_eq!(msg, "error msg\n");
}

#[test]
fn from_str() {
    for (s, expected) in [
        ("ns", Timestamp::Nanosecond),
        ("ms", Timestamp::Millisecond),
        ("us", Timestamp::Microsecond),
        ("sec", Timestamp::Second),
        ("none", Timestamp::Off),
        ("off", Timestamp::Off),
    ] {
        // `Timestamp` doesn't implement `PartialEq`, so compare via `Debug`.
        let parsed = Timestamp::from_str(s).unwrap();
        assert_eq!(
            format!("{:?}", parsed),
            format!("{:?}", expected),
            "{:?}",
            s
        );
    }
    assert!(Timestamp::from_str("bogus").is_err());
}
