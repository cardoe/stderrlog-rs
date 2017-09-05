use std::sync;
use std::cell::RefCell;
use stderrlog::StdErrLog;
use log::{self, Log};

thread_local! {
    pub static LOGGER_INSTANCE: RefCell<Option<StdErrLog>> = RefCell::new(None);
}

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

struct DelegatingLogger;

impl Log for DelegatingLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        LOGGER_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            if let Some(ref instance) = *instance {
                instance.enabled(metadata)
            } else {
                false
            }
        })
    }

    fn log(&self, record: &log::LogRecord) {
        LOGGER_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            if let Some(ref instance) = *instance {
                instance.log(record);
            }
        });
    }
}

pub fn init() {
    INIT_LOGGER.call_once(|| {
        log::set_logger(|max_level| {
            max_level.set(log::LogLevelFilter::max());
            Box::new(DelegatingLogger)
        }).unwrap();
    });
}

pub fn set_logger(logger: StdErrLog) {
    LOGGER_INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();
        *instance = Some(logger);
    });
}