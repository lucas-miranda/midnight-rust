use std::{
    fmt,
    time::{Duration, Instant},
};

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

    pub fn delta<'a>(&'a self, last_instant: &mut Instant) -> DeltaTime<'a> {
        let now = Self::now();
        let delta_time = DeltaTime::new(&self, now - *last_instant);
        *last_instant = now;
        delta_time
    }
}

pub struct DeltaTime<'a> {
    time: &'a Time,
    duration: Duration,
}

impl<'a> DeltaTime<'a> {
    pub(crate) fn new(time: &'a Time, duration: Duration) -> Self {
        Self {
            time,
            duration,
        }
    }

    pub fn base_time(&self) -> &Time {
        self.time
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }
}

impl<'a> fmt::Debug for DeltaTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.duration.fmt(f)
    }
}
