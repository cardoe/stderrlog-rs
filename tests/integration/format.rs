//! The exact bytes written to stderr for the various formatting options.

use stderrlog::ColorChoice;

#[test]
fn no_level() {
    let stderr = isolated!(no_level, {
        stderrlog::new()
            .verbosity(1)
            .show_level(false)
            .init()
            .unwrap();
        log_all!();
    });
    assert_eq!(stderr, "error msg\nwarn msg\n");
}

#[test]
fn module_names() {
    let stderr = isolated!(module_names, {
        stderrlog::new().show_module_names(true).init().unwrap();
        log::error!("error msg");
        log::error!(target: "custom::target", "other msg");
    });
    assert_eq!(
        stderr,
        "integration::format: ERROR error msg\ncustom::target: ERROR other msg\n",
    );
}

#[test]
fn multiline_message() {
    let stderr = isolated!(multiline_message, {
        stderrlog::new().init().unwrap();
        log::error!("line one\nline two");
    });
    assert_eq!(stderr, "ERROR line one\nline two\n");
}

#[test]
fn auto_color_disabled_when_not_a_terminal() {
    // The child's stderr is a pipe, so `Auto` must not emit escape codes.
    let stderr = isolated!(auto_color_disabled_when_not_a_terminal, {
        stderrlog::new().color(ColorChoice::Auto).init().unwrap();
        log::error!("error msg");
    });
    assert_eq!(stderr, "ERROR error msg\n");
}

#[test]
fn color_never() {
    let stderr = isolated!(color_never, {
        stderrlog::new().color(ColorChoice::Never).init().unwrap();
        log::error!("error msg");
    });
    assert_eq!(stderr, "ERROR error msg\n");
}

#[test]
fn color_always_ansi() {
    // `AlwaysAnsi` rather than `Always`, since on Windows `Always` would try
    // the console API, which isn't available on a pipe.
    let stderr = isolated!(color_always_ansi, {
        stderrlog::new()
            .verbosity(4)
            .color(ColorChoice::AlwaysAnsi)
            .init()
            .unwrap();
        log_all!();
    });
    let expected = [
        ("31", "ERROR", "error msg"),
        ("33", "WARN", "warn msg"),
        ("34", "INFO", "info msg"),
        ("36", "DEBUG", "debug msg"),
        ("35", "TRACE", "trace msg"),
    ]
    .iter()
    .map(|(code, level, msg)| format!("\x1b[0m\x1b[{}m{} \x1b[0m{}\n", code, level, msg))
    .collect::<String>();
    assert_eq!(stderr, expected);
}

#[test]
fn module_filter() {
    let stderr = isolated!(module_filter, {
        stderrlog::new()
            .module(module_path!())
            .module("allowed")
            .init()
            .unwrap();
        log::error!("mine");
        log::error!(target: "allowed::child", "allowed child");
        log::error!(target: "allowedish", "sibling with shared prefix");
        log::error!(target: "other", "other");
    });
    assert_eq!(stderr, "ERROR mine\nERROR allowed child\n");
}
