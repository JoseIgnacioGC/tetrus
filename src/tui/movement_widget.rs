use std::time::Duration;

use ratatui::{
    layout::Rect,
    macros::{constraint, vertical},
    style::{Color, Stylize},
    text::Line,
    Frame,
};
use tachyonfx::{fx, EffectRenderer, Interpolation};

use crate::{
    board::Board,
    colors::GOLD,
    constants::{COMBO_NOTIFICATION_DURATION, COMBO_NOTIFICATION_FADE_DELAY},
};

pub struct MovementWidget {
    last_movement: Option<(&'static str, usize, Duration)>,
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
        let [tspin_area, clear_area, b2b_area, combo_area] =
            vertical![== 1, == 1, == 1, == 1].areas(area.centered_vertically(constraint!(== 4)));

        let fade_duration = COMBO_NOTIFICATION_DURATION
            .saturating_sub(COMBO_NOTIFICATION_FADE_DELAY)
            .as_millis() as u32;

        if let Some((movement, b2b_count, elapsed)) = self.last_movement {
            let has_b2b = movement.starts_with("B2B ");
            let rest = movement.strip_prefix("B2B ").unwrap_or(movement);
            let has_tspin = rest.contains("T-Spin");

            if has_tspin {
                let text = Line::from("T-SPIN").magenta().bold().right_aligned();
                frame.render_widget(text, tspin_area);
            }

            let clear_text = if rest.contains("Single") || rest.contains("single") {
                Some("SINGLE")
            } else if rest.contains("Double") || rest.contains("double") {
                Some("DOUBLE")
            } else if rest.contains("Triple") || rest.contains("triple") {
                Some("TRIPLE")
            } else if rest.contains("Quad") || rest.contains("quad") {
                Some("QUAD")
            } else if rest.contains("Perfect Clear") {
                Some("PERFECT CLEAR!")
            } else {
                None
            };

            if let Some(clear_str) = clear_text {
                let text = Line::from(clear_str).white().bold().right_aligned();
                frame.render_widget(text, clear_area);
            }

            if has_b2b {
                let text = Line::from(format!("B2B  x{}", b2b_count.max(1)))
                    .fg(GOLD)
                    .bold()
                    .right_aligned();
                frame.render_widget(text, b2b_area);
            }

            if elapsed > COMBO_NOTIFICATION_FADE_DELAY {
                let effect_elapsed = elapsed - COMBO_NOTIFICATION_FADE_DELAY;
                let mut effect = fx::fade_to_fg(
                    Color::Rgb(50, 50, 50),
                    (fade_duration, Interpolation::CubicOut),
                );

                if has_tspin {
                    frame.render_effect(&mut effect, tspin_area, effect_elapsed.into());
                }
                if clear_text.is_some() {
                    frame.render_effect(&mut effect, clear_area, effect_elapsed.into());
                }
                if has_b2b {
                    frame.render_effect(&mut effect, b2b_area, effect_elapsed.into());
                }
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
