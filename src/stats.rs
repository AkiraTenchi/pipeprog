use crate::timer::Timer;
use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::Instant;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.get_delta().as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.get_ready() {
            timer.unready();
            eprint!(
                "\rtotal_bytes: {} {} [{:.0}b/s]",
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

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}
