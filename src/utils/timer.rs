use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Timer {
    accumulated: Duration,
    started_at: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.started_at.take() {
            self.accumulated += start.elapsed();
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.started_at {
            Some(start) => self.accumulated + start.elapsed(),
            None => self.accumulated,
        }
    }

    pub fn reset(&mut self) {
        self.accumulated = Duration::ZERO;
        self.started_at = None;
    }
}
