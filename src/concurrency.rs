use std::error::Error;
use std::thread;

use ohshit::Logger;

pub fn exercise_concurrency(_logger: &Logger) -> Result<(), Box<dyn Error>> {
    let workers = (0..4)
        .map(|worker| {
            thread::spawn(move || {
                for sequence in 0..20 {
                    ohshit::info!(target: "concurrency", worker = worker, sequence = sequence; "worker emitted a record");
                }
            })
        })
        .collect::<Vec<_>>();

    for worker in workers {
        worker
            .join()
            .map_err(|_| "a concurrent logging worker panicked")?;
    }
    Ok(())
}
