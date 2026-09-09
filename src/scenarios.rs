use std::error::Error;
use std::sync::Arc;

use ohshit::{Filter, Level, Logger, LoggerConfig, MemorySink, Record};

pub fn exercise_logging(logger: &Logger) -> Result<(), Box<dyn Error>> {
    ohshit::trace!(target: "basic", operation = "trace"; "trace event");
    ohshit::debug!(target: "basic", operation = "debug"; "debug event");
    ohshit::info!(target: "basic", operation = "info"; "info event");
    ohshit::warn!(target: "basic", operation = "warn"; "warn event");
    ohshit::error!(target: "basic", operation = "error"; "error event");

    let filtered_sink = Arc::new(MemorySink::new());
    let filtered_config = LoggerConfig {
        level: Level::Error,
        filter: Filter::new(Level::Error),
        ..LoggerConfig::default()
    };
    let filtered = Logger::builder()
        .with_config(filtered_config)
        .build()
        .with_sink(filtered_sink.clone());
    filtered.log(Record::new(Level::Info, "filtered info"));
    filtered.log(Record::new(Level::Error, "retained error"));

    let records = filtered_sink.records();
    if records.len() != 1 || records[0].level() != Level::Error {
        return Err("minimum-level filtering did not remove the info record".into());
    }

    logger.log(
        Record::new(Level::Info, "request completed")
            .with_target("http")
            .with_context("request_id", "req-0001")
            .with_context("duration_ms", 14)
            .with_context("connection_state", "keep-alive"),
    );
    Ok(())
}

pub fn inspect_records(sink: Arc<MemorySink>) -> Result<(), Box<dyn Error>> {
    let records = sink.records();
    if records.len() < 6 {
        return Err(format!("expected at least 6 records, got {}", records.len()).into());
    }
    if records
        .iter()
        .any(|record| record.metadata().timestamp_seconds() == 0)
    {
        return Err("a record was created without a usable timestamp".into());
    }
    if !records.iter().any(|record| record.target() == "http") {
        return Err("structured target was not preserved".into());
    }
    if !records.iter().any(|record| {
        record
            .context()
            .entries()
            .iter()
            .any(|(key, _)| key == "request_id")
    }) {
        return Err("structured request_id field was not preserved".into());
    }
    let diagnostic_record = records
        .iter()
        .find(|record| record.level() == Level::OhShit)
        .ok_or("memory sink did not receive the diagnostic record")?;
    let diagnostic = diagnostic_record
        .metadata()
        .diagnostic()
        .ok_or("diagnostic record did not retain its structured diagnostic")?;
    if diagnostic.cause().is_none()
        || diagnostic.reason().is_none()
        || diagnostic.action().is_none()
        || diagnostic.location().is_none()
        || diagnostic_record.location().is_none()
    {
        return Err("diagnostic metadata was incomplete in the memory sink".into());
    }
    if records
        .iter()
        .filter(|record| record.target() == "concurrency")
        .count()
        < 80
    {
        return Err("concurrent logging records were not all dispatched".into());
    }
    Ok(())
}
