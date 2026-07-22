use ratatui::{
    buffer::Buffer,
    layout::{Offset, Rect},
    macros::{span, text},
    widgets::Widget,
};
use std::time::{Duration, Instant};

use crate::board::Board;

#[cfg(debug_assertions)]
pub struct DebugWidget {
    acc_time: Duration,
    last_tick: Instant,
    last_fps_count: Instant,
    fps_counter: usize,
    fps: usize,
    fall_speed: f32,
}

impl DebugWidget {
    pub fn new() -> Self {
        Self {
            acc_time: Duration::ZERO,
            last_tick: Instant::now(),
            last_fps_count: Instant::now(),
            fps_counter: 0,
            fps: 60,
            fall_speed: 0.0,
        }
    }

    pub fn copy_metrics(&mut self, board: &Board) {
        self.fall_speed = board.fall_speed.as_secs_f32();
    }
}

impl Widget for &mut DebugWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_tick);

        self.last_tick = current_time;
        self.acc_time += delta_time;
        self.fps_counter += 1;

        if current_time.duration_since(self.last_fps_count) >= Duration::from_secs(1) {
            self.fps = self.fps_counter;
            self.fps_counter = 0;
            self.last_fps_count = current_time;
        };

        text![
            "[debug]",
            span!("fps: {}", self.fps),
            span!("fall_speed: {}", self.fall_speed),
        ]
        .left_aligned()
        .render(area.offset(Offset::new(3, 0)), buf);
    }
}
