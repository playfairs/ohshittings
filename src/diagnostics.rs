use std::error::Error;
use std::fs::File;

use ohshit::{Diagnostic, Location, Logger};

pub fn exercise_diagnostics(logger: &Logger) -> Result<(), Box<dyn Error>> {
    let path = "/definitely/missing/ohshittings-input";
    match File::open(path) {
        Ok(_) => return Err("the deliberately missing path unexpectedly opened".into()),
        Err(error) => {
            let diagnostic = Diagnostic::new(format!("Could not open {path}"))
                .with_cause(error.to_string())
                .with_reason("The operating system rejected the file-open operation.")
                .with_action("provide a readable input path")
                .with_location(Location::new(file!(), line!()));
            ohshit::ohshit!(target: "filesystem", diagnostic);
        }
    }

    let invalid = "";
    if invalid.parse::<u16>().is_err() {
        logger.warn("invalid port input was rejected");
    }
    Ok(())
}
