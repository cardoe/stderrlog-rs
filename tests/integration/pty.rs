//! Logging to a terminal that goes away must not panic.
//!
//! This swaps out the process's stderr file descriptor, so it runs in an
//! isolated child process.

use std::fs::File;
use std::io::Error;
use std::os::unix::io::{AsRawFd, FromRawFd};

fn check_ret(ret: libc::c_int, what: &str) {
    if ret < 0 {
        panic!("{}: {}", what, Error::last_os_error());
    }
}

fn wrap_fd(fd: libc::c_int, what: &str) -> File {
    check_ret(fd, what);
    // SAFETY: `fd` was just returned to us by the OS and nothing else owns it.
    unsafe { File::from_raw_fd(fd) }
}

#[test]
fn log_after_pty_close() {
    let stderr = isolated!(log_after_pty_close, {
        let pt = wrap_fd(
            unsafe { libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY) },
            "posix_openpt",
        );
        check_ret(unsafe { libc::grantpt(pt.as_raw_fd()) }, "grantpt");
        check_ret(unsafe { libc::unlockpt(pt.as_raw_fd()) }, "unlockpt");
        let name = unsafe { libc::ptsname(pt.as_raw_fd()) };
        assert!(!name.is_null(), "ptsname: {}", Error::last_os_error());
        let client = wrap_fd(
            unsafe { libc::open(name, libc::O_RDWR | libc::O_NOCTTY) },
            "open pty client",
        );

        // Keep the real stderr so the parent still sees panics from here on.
        let saved_stderr = wrap_fd(unsafe { libc::dup(2) }, "dup stderr");
        check_ret(unsafe { libc::dup2(client.as_raw_fd(), 2) }, "dup2 pty");
        drop(client);

        // stderr is now a terminal, so `Auto` keeps colors enabled.
        stderrlog::new().verbosity(4).init().unwrap();
        log::info!("to the pty");

        // Closing the controlling side makes further writes fail with EIO.
        drop(pt);
        log::info!("after the pty closed");
        log::logger().flush();

        check_ret(
            unsafe { libc::dup2(saved_stderr.as_raw_fd(), 2) },
            "dup2 restore",
        );
        log::error!("restored");
    });
    // Only what was logged after restoring stderr reaches the parent. The
    // logger caches its stream per thread but writes through fd 2, so it
    // picks up the restored descriptor.
    assert!(stderr.ends_with("restored\n"), "{:?}", stderr);
}
