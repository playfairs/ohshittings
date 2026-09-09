mod app;
mod concurrency;
mod diagnostics;
mod formatting;
mod logging;
mod networking;
mod scenarios;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
