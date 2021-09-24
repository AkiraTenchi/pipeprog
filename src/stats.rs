use crate::timer::Timer;
use crossbeam::channel::Receiver;
use crossterm::style::Stylize;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{stderr, Result, Stderr, Write};
use std::time::Instant;

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

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes).with(Color::Rgb {
        r: 255,
        g: 204,
        b: 255,
    }));
    let elapsed = style::style(elapsed.with(Color::Rgb {
        r: 102,
        g: 204,
        b: 255,
    }));
    let rate = style::style(format!(" [{:.0}b/s]", rate).with(Color::Rgb {
        r: 204,
        g: 255,
        b: 153,
    }));
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
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
