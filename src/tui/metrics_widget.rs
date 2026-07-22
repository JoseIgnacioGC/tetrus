use crate::{
    board::{Board, GOAL_MULTIPLIER},
    utils::integer_format::{to_superscript, to_superscript_with_separator},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    macros::{line, span, text},
    widgets::Widget,
};
use std::fmt::Write;
use std::time::Instant;

pub struct MetricsWidget {
    time: String,
    level: usize,
    cleaned_lines: usize,
    score: usize,
}
const MAX_INSTANT_STR_CAPACITY: usize = 9;

impl MetricsWidget {
    pub fn new() -> Self {
        Self {
            time: String::with_capacity(MAX_INSTANT_STR_CAPACITY),
            level: 1,
            cleaned_lines: 0,
            score: 0,
        }
    }

    pub fn format_instant(&mut self, instant: &Instant) {
        let total_ms = instant.elapsed().as_millis();

        let minutes = (total_ms / 60_000) % 60;
        let seconds = (total_ms / 1_000) % 60;
        let milliseconds = total_ms % 1_000;

        self.time.clear();

        let _ = write!(
            self.time,
            "{:02}:{:02}.{:03}",
            minutes, seconds, milliseconds
        );
    }

    pub fn copy_metrics(&mut self, board: &Board, instant: &Instant) {
        self.format_instant(instant);
        self.level = board.level;
        self.cleaned_lines = board.cleaned_lines;
        self.score = board.score;
    }
}

impl Widget for &MetricsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        text![
            "lv",
            to_superscript(self.level),
            "score",
            to_superscript_with_separator(self.score),
            "lines",
            line![span!(
                "{}⁄{}",
                to_superscript(self.cleaned_lines),
                to_superscript(self.level * GOAL_MULTIPLIER)
            )],
            "time",
            self.time.as_str(),
        ]
        .right_aligned()
        .render(area, buf);
    }
}
