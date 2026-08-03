use std::time::Duration;

use ratatui::{
    layout::Rect,
    macros::{constraint, vertical},
    style::{Color, Stylize},
    text::{Line, Span},
    Frame,
};
use tachyonfx::{fx, EffectRenderer, Interpolation};

use crate::{
    board::Board,
    colors::GOLD,
    constants::{COMBO_NOTIFICATION_DURATION, COMBO_NOTIFICATION_FADE_DELAY},
};

pub struct MovementWidget {
    last_movement: Option<(&'static str, Duration)>,
    combo: Option<(usize, Duration)>,
}

impl MovementWidget {
    pub fn new() -> Self {
        Self {
            last_movement: None,
            combo: None,
        }
    }

    pub fn copy_metrics(&mut self, board: &Board) {
        self.last_movement = board.last_movement();
        self.combo = board.current_combo();
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let [movement_title_area, combo_area] =
            vertical![== 1, == 1].areas(area.centered_vertically(constraint!(== 2)));

        let fade_duration = COMBO_NOTIFICATION_DURATION
            .saturating_sub(COMBO_NOTIFICATION_FADE_DELAY)
            .as_millis() as u32;

        if let Some((movement, elapsed)) = self.last_movement {
            let movement_text = if let Some(rest) = movement.strip_prefix("B2B ") {
                let rest_span = if rest.contains("T-Spin") {
                    Span::from(rest.to_string()).magenta().bold()
                } else {
                    Span::from(rest.to_string()).white().bold()
                };
                Line::from(vec!["B2B ".fg(GOLD).bold(), rest_span]).right_aligned()
            } else if movement.contains("T-Spin") {
                Line::from(movement).magenta().bold().right_aligned()
            } else {
                Line::from(movement).white().bold().right_aligned()
            };
            frame.render_widget(movement_text, movement_title_area);

            if elapsed > COMBO_NOTIFICATION_FADE_DELAY {
                let effect_elapsed = elapsed - COMBO_NOTIFICATION_FADE_DELAY;
                let mut effect = fx::fade_to_fg(
                    Color::Rgb(50, 50, 50),
                    (fade_duration, Interpolation::CubicOut),
                );
                frame.render_effect(&mut effect, movement_title_area, effect_elapsed.into());
            }
        }

        if let Some((combo_count, elapsed)) = self.combo {
            let combo_text = Line::from(format!("{} Combo", combo_count))
                .white()
                .bold()
                .right_aligned();
            frame.render_widget(combo_text, combo_area);

            if elapsed > COMBO_NOTIFICATION_FADE_DELAY {
                let effect_elapsed = elapsed - COMBO_NOTIFICATION_FADE_DELAY;
                let mut effect = fx::fade_to_fg(
                    Color::Rgb(50, 50, 50),
                    (fade_duration, Interpolation::CubicOut),
                );
                frame.render_effect(&mut effect, combo_area, effect_elapsed.into());
            }
        }
    }
}
