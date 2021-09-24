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
