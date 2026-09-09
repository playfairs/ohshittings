use std::sync::{Arc, OnceLock};

use ohshit::{ConsoleSink, Filter, Level, Logger, LoggerConfig, MemorySink};

static MEMORY: OnceLock<Arc<MemorySink>> = OnceLock::new();

pub fn install() -> Logger {
    let _ = ohshit::init();
    let _ = ohshit::init();

    let config = LoggerConfig {
        level: Level::Trace,
        filter: Filter::new(Level::Trace),
        colors: false,
        show_location: true,
        show_timestamp: true,
        show_target: true,
    };
    let memory = Arc::new(MemorySink::new());
    let console = Arc::new(ConsoleSink::new().with_config(config.formatter_config()));
    let logger = Logger::builder()
        .with_config(config)
        .build()
        .with_sink(console)
        .with_sink(memory.clone());

    let _ = MEMORY.set(memory);
    ohshit::install_logger(logger.clone());
    logger
}

pub fn memory_sink() -> Arc<MemorySink> {
    MEMORY
        .get()
        .expect("logging must be installed before records are inspected")
        .clone()
}
