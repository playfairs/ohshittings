use std::error::Error;
use std::sync::Arc;

use ohshit::{
    Diagnostic, Formatter, FormatterConfig, Level, Logger, LoggerConfig, Record, TerminalFormatter,
};

#[derive(Debug)]
struct PrefixFormatter {
    terminal: TerminalFormatter,
}

impl Formatter for PrefixFormatter {
    fn format_record(&self, record: &Record, config: &FormatterConfig) -> String {
        format!(
            "consumer {}",
            self.terminal.format_record_with_config(record, config)
        )
    }

    fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        self.terminal.format_diagnostic(diagnostic)
    }
}

pub fn exercise_formatting(logger: &Logger) -> Result<(), Box<dyn Error>> {
    let cases = [
        (Level::Trace, "normal message"),
        (Level::Debug, ""),
        (Level::Info, "unicode: café 日本語"),
        (Level::Warn, "line one\nline two"),
        (
            Level::Error,
            "a deliberately long message that exercises terminal formatting without relying on a fixed width",
        ),
    ];
    for (level, message) in cases {
        logger.log(
            Record::new(level, message)
                .with_target("formatting")
                .with_context("case", level.to_string()),
        );
    }

    let diagnostic = Diagnostic::new("Formatting diagnostic")
        .with_cause("formatting test cause")
        .with_reason("the formatter must preserve structured diagnostics")
        .with_action("inspect the rendered output")
        .with_location(ohshit::Location::new(file!(), line!()));
    let terminal = TerminalFormatter::default();
    let rendered = terminal.format_diagnostic(&diagnostic);
    if !rendered.contains("Formatting diagnostic") || !rendered.contains("Cause:") {
        return Err("diagnostic formatter dropped structured content".into());
    }

    let sink = Arc::new(ohshit::ConsoleSink::with_formatter(Arc::new(
        PrefixFormatter {
            terminal: TerminalFormatter::default(),
        },
    )));
    let custom = Logger::builder()
        .with_config(LoggerConfig::default())
        .build()
        .with_sink(sink.clone());
    custom.info("custom formatter record");
    if !sink
        .entries()
        .iter()
        .any(|entry| entry.starts_with("consumer "))
    {
        return Err("custom formatter was not used by the console sink".into());
    }
    Ok(())
}
