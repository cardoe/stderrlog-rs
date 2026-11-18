use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::panic;
use std::sync::Mutex;

/// Swapping fd 2 is process-wide, so only one caller may do it at a time.
/// Benchmarks run serially, but `cargo test --benches` runs them in parallel.
static STDERR_LOCK: Mutex<()> = Mutex::new(());

pub fn with_redirected_stderr<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
    F: panic::UnwindSafe,
{
    let _guard = STDERR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let file = File::create("/dev/null").unwrap();
    let fd = file.as_raw_fd();
    let orig_fd = unsafe {
        let orig_fd = libc::dup(2);
        assert!(orig_fd >= 0);
        let rc = libc::dup2(fd, 2);
        assert!(rc >= 0);
        orig_fd
    };
    drop(file);
    let result = panic::catch_unwind(f);
    unsafe {
        libc::dup2(orig_fd, 2);
        libc::close(orig_fd);
    }
    match result {
        Ok(t) => t,
        Err(e) => panic::resume_unwind(e),
    }
}
