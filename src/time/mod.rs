use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Time {
    begin: Instant,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            begin: Instant::now(),
        }
    }

    pub fn now() -> Instant {
        Instant::now()
    }

    pub fn elapsed(&self) -> Duration {
        self.begin.elapsed()
    }

    pub fn delta(&self, last_instant: &mut Instant) -> DeltaTime {
        let now = Self::now();
        let delta_time = DeltaTime::new(now - *last_instant);
        *last_instant = now;
        delta_time
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeltaTime {
    pub duration: Duration,
}

impl DeltaTime {
    pub(crate) fn new(duration: Duration) -> Self {
        Self {
            duration,
        }
    }
}
