use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct HeldKey<K> {
    key: Option<K>,
    target_duration: Duration,
    release_timeout: Duration,
    press_start: Option<Instant>,
    last_event_time: Option<Instant>,
}

impl<K: PartialEq + Copy> HeldKey<K> {
    pub fn new(target_duration: Duration) -> Self {
        Self {
            key: None,
            target_duration,
            release_timeout: Duration::from_millis(50),
            press_start: None,
            last_event_time: None,
        }
    }

    pub fn with_release_timeout(mut self, timeout: Duration) -> Self {
        self.release_timeout = timeout;
        self
    }

    pub fn press(&mut self, key: K) {
        let now = Instant::now();
        if self.key != Some(key) {
            self.key = Some(key);
            self.press_start = Some(now);
        }
        self.last_event_time = Some(now);
    }

    pub fn release(&mut self) {
        self.key = None;
        self.press_start = None;
        self.last_event_time = None;
    }

    pub fn release_if_key(&mut self, key: K) {
        if self.key == Some(key) {
            self.release();
        }
    }

    pub fn update(&mut self) {
        if let Some(last) = self.last_event_time {
            if last.elapsed() > self.release_timeout {
                self.release();
            }
        }
    }

    pub fn key(&self) -> Option<K> {
        self.key
    }

    pub fn is_held(&self) -> bool {
        self.key.is_some()
    }

    pub fn elapsed(&self) -> Duration {
        match self.press_start {
            Some(start) => start.elapsed(),
            None => Duration::ZERO,
        }
    }

    pub fn is_held_for(&self, duration: Duration) -> bool {
        self.is_held() && self.elapsed() >= duration
    }

    pub fn is_target_reached(&self) -> bool {
        self.is_held_for(self.target_duration)
    }

    pub fn set_target_duration(&mut self, duration: Duration) {
        self.target_duration = duration;
    }
}
