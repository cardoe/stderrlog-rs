extern crate stderrlog;
#[macro_use]
extern crate log;

mod utils;

mod included_not {
    use utils;
    use log;
    use stderrlog::StdErrLog;
    #[test]
    fn including_module_with_substring_name() {
        utils::init();
        let mut logger = StdErrLog::new();
        logger.module("module_inclusion::included");
        logger.verbosity(10);
        utils::set_logger(logger);
        assert!(!log_enabled!(log::LogLevel::Error));
    }
}

mod included {
    mod b {
        use utils;
        use log;
        use stderrlog::StdErrLog;
        #[test]
        fn super_and_submodule_included() {
            utils::init();
            let mut logger = StdErrLog::new();
            logger.module("module_inclusion::included");
            logger.module("module_inclusion::included::a");
            logger.verbosity(10);
            utils::set_logger(logger);
            assert!(log_enabled!(log::LogLevel::Error));
        }
    }
}