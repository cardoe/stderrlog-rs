#[cfg(unix)]
mod unix {
    use std::fs::File;
    use std::io::Error;
    use std::os::unix::io::{AsRawFd, FromRawFd};

    use log::info;

    fn check_ret(ret: libc::c_int, err_msg: &str) {
        if ret < 0 {
            panic!("{}: {}", err_msg, Error::last_os_error());
        }
    }

    fn wrap_fd(fd: libc::c_int, err_msg: &str) -> File {
        check_ret(fd, err_msg);
        // Use File as a owned fd.
        unsafe { File::from_raw_fd(fd) }
    }

    #[cfg(all(unix, target_os = "linux"))]
    #[test]
    fn log_after_pty_close() {
        let pt = wrap_fd(unsafe { libc::getpt() }, "getpt");
        check_ret(unsafe { libc::grantpt(pt.as_raw_fd()) }, "grantpt");
        check_ret(unsafe { libc::unlockpt(pt.as_raw_fd()) }, "unlockpt");

        let name = unsafe { libc::ptsname(pt.as_raw_fd()) };
        if name.is_null() {
            panic!("ptsname: {}", Error::last_os_error());
        }

        let client = wrap_fd(unsafe { libc::open(name, libc::O_RDWR) }, "open client pty");

        // Back up stderr
        let dup_stderr = wrap_fd(unsafe { libc::dup(2) }, "dup stderr");
        let dup_stderr_panic = wrap_fd(unsafe { libc::dup(2) }, "dup stderr");

        // Set up the panic handler to restore stderr.
        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            check_ret(
                unsafe { libc::dup2(dup_stderr_panic.as_raw_fd(), 2) },
                "dup2 restore stderr",
            );
            default_panic(info);
        }));

        // Replace stderr with pty
        check_ret(
            unsafe { libc::dup2(client.as_raw_fd(), 2) },
            "dup2 restore stderr",
        );

        stderrlog::new().verbosity(50).init().unwrap();
        info!("This should work.");

        println!("Closing PTY");
        drop(pt);

        println!("Sending log after PTY closed");
        info!("This should trigger an EIO.");

        // Restore stderr
        check_ret(
            unsafe { libc::dup2(dup_stderr.as_raw_fd(), 2) },
            "dup2 restore stderr",
        );
    }
}
