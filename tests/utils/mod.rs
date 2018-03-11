use log::{self, Log};
use std::cell::RefCell;
use std::sync;
use stderrlog::StdErrLog;

thread_local! {
    pub static LOGGER_INSTANCE: RefCell<Option<StdErrLog>> = RefCell::new(None);
}

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

struct DelegatingLogger;

impl Log for DelegatingLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        LOGGER_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            if let Some(ref instance) = *instance {
                instance.enabled(metadata)
            } else {
                false
            }
        })
    }

    fn log(&self, record: &log::Record) {
        LOGGER_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            if let Some(ref instance) = *instance {
                instance.log(record);
            }
        });
    }

    fn flush(&self) {
        LOGGER_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            if let Some(ref instance) = *instance {
                instance.flush();
            }
        });
    }
}

pub fn init() {
    INIT_LOGGER.call_once(|| {
        log::set_max_level(log::LevelFilter::max());
        log::set_boxed_logger(Box::new(DelegatingLogger)).unwrap();
    });
}

pub fn set_logger(logger: StdErrLog) {
    LOGGER_INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();
        *instance = Some(logger);
    });
}
