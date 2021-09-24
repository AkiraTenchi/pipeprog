//! time management for the stats module

use std::time::{Duration, Instant};

pub struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    pub(crate) fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub(crate) fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }

    pub(crate) fn get_delta(&self) -> Duration {
        self.delta
    }

    pub(crate) fn get_ready(&self) -> bool {
        self.ready
    }

    pub(crate) fn unready(&mut self) {
        self.ready = false;
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    fn check_if_ready_initialised_as_true() {
        let timer = Timer::new();
        assert!(timer.get_ready());
    }

    #[test]
    fn check_if_unready_sets_ready_to_false() {
        let mut timer = Timer::new();
        timer.unready();
        assert_eq!(timer.get_ready(), false);
    }
}
