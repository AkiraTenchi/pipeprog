//! The stats module contains the loop keeping track and timing the output of the transfer statistics

pub mod output_formatter;
mod timer;

use crossbeam::channel::Receiver;
use output_formatter::{output_progress, TimeOutput};
use std::io::{stderr, Result};
use std::time::Instant;
use timer::Timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.get_delta().as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.get_ready() {
            timer.unready();
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second,
            );
        }
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}
