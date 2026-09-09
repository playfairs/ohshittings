use std::error::Error;

use crate::{concurrency, diagnostics, formatting, logging, networking, scenarios};

pub fn run() -> Result<(), Box<dyn Error>> {
    let logger = logging::install();
    ohshit::info!(target: "application", operation = "startup"; "ohshittings initialized");

    scenarios::exercise_logging(&logger)?;
    diagnostics::exercise_diagnostics(&logger)?;
    formatting::exercise_formatting(&logger)?;
    concurrency::exercise_concurrency(&logger)?;
    networking::exercise_networking(&logger)?;
    scenarios::inspect_records(logging::memory_sink())?;

    ohshit::info!(target: "application", operation = "shutdown"; "ohshittings completed cleanly");
    Ok(())
}
