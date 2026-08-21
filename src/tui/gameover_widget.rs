use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    macros::{constraint, span},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Widget},
};

use crate::{
    colors::{BRONZE, GOLD, SILVER},
    scores::{HighScores, Initials, ScoreEntry},
};

#[derive(Default, PartialEq, Eq)]
pub enum GameoverState {
    #[default]
    Pass,
    EnterGame,
    Brake,
}

#[derive(PartialEq, Eq)]
pub enum GameoverStage {
    EnteringInitials,
    Menu,
}

pub struct GameoverWidget<'a> {
    option_index: usize,
    menu_options: [Span<'a>; 2],
    stage: GameoverStage,
    high_scores: HighScores,
    game_mode: String,
    current_score: usize,
    current_lines: usize,
    current_level: usize,
    qualified_rank: Option<usize>,
    highlighted_rank: Option<usize>,
    initials: Initials,
}

impl<'a> GameoverWidget<'a> {
    pub fn new() -> Self {
        Self {
            option_index: 0,
            menu_options: ["again?".into(), "quit".into()],
            stage: GameoverStage::Menu,
            high_scores: HighScores::default(),
            game_mode: "endless".to_string(),
            current_score: 0,
            current_lines: 0,
            current_level: 0,
            qualified_rank: None,
            highlighted_rank: None,
            initials: Initials::new(),
        }
    }

    pub fn setup_game_over(&mut self, mode: &str, score: usize, lines: usize, level: usize) {
        self.game_mode = mode.to_string();
        self.current_score = score;
        self.current_lines = lines;
        self.current_level = level;
        self.option_index = 0;
        self.initials = Initials::new();
        self.high_scores = HighScores::load();
        self.qualified_rank = self.high_scores.check_qualification(mode, score);

        if self.qualified_rank.is_some() {
            self.stage = GameoverStage::EnteringInitials;
            self.highlighted_rank = None;
        } else {
            self.stage = GameoverStage::Menu;
            self.highlighted_rank = None;
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> GameoverState {
        match self.stage {
            GameoverStage::EnteringInitials => match event.code {
                KeyCode::Char(c) => {
                    self.initials.push(c);
                    GameoverState::Pass
                }
                KeyCode::Backspace => {
                    self.initials.pop();
                    GameoverState::Pass
                }
                KeyCode::Enter => {
                    let final_initials = if self.initials.is_empty() {
                        Initials::from_str("PLAYER")
                    } else {
                        self.initials
                    };
                    let rank = self.high_scores.insert(
                        &self.game_mode,
                        ScoreEntry {
                            initials: final_initials,
                            score: self.current_score,
                            lines: self.current_lines,
                            level: self.current_level,
                        },
                    );
                    self.highlighted_rank = Some(rank);
                    self.stage = GameoverStage::Menu;
                    GameoverState::Pass
                }
                _ => GameoverState::Pass,
            },
            GameoverStage::Menu => {
                let options_len = self.menu_options.len();
                match event.code {
                    KeyCode::Up => {
                        self.option_index = (self.option_index + options_len - 1) % options_len;
                        GameoverState::Pass
                    }
                    KeyCode::Down => {
                        self.option_index = (self.option_index + 1) % options_len;
                        GameoverState::Pass
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => match self.option_index {
                        0 => GameoverState::EnterGame,
                        1 => GameoverState::Brake,
                        _ => unreachable!(),
                    },
                    _ => GameoverState::Pass,
                }
            }
        }
    }

    fn format_number(num: usize) -> String {
        let s = num.to_string();
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        for (i, &c) in chars.iter().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        result
    }
}

impl<'a> Widget for &mut GameoverWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block_width = 38;
        let block_height = if self.stage == GameoverStage::EnteringInitials {
            19
        } else {
            17
        };

        let block_area = area.centered(constraint!(== block_width), constraint!(== block_height));
        let mut lines = Vec::new();

        lines.push(Line::from("GAME OVER".bold()).centered());
        lines.push(Line::raw(""));

        match self.stage {
            GameoverStage::EnteringInitials => {
                let rank_num = self.qualified_rank.unwrap_or(1);
                let rank_color = match rank_num {
                    1 => GOLD,
                    2 => SILVER,
                    3 => BRONZE,
                    _ => Color::Rgb(160, 160, 160),
                };

                let rank_label = if rank_num <= 3 {
                    format!("#{}", rank_num)
                } else {
                    format!("{}", rank_num)
                };

                lines.push(
                    Line::from(vec![
                        span!("you are top ").white(),
                        span!("{}", rank_label).fg(rank_color).bold(),
                    ])
                    .centered(),
                );

                lines
                    .push(Line::from(GameoverWidget::format_number(self.current_score)).centered());
                lines.push(Line::raw(""));

                let mut slot_spans = Vec::new();
                for i in 0..Initials::MAX_LEN {
                    if i < self.initials.len() {
                        let c = self.initials.as_chars()[i];
                        slot_spans.push(span!("{} ", c).white().bold());
                    } else {
                        slot_spans.push(span!("_ ").white().bold());
                    }
                }
                lines.push(Line::from(slot_spans).centered());
                lines.push(Line::from("enter your initials".dim()).centered());
            }
            GameoverStage::Menu => {
                lines.push(Line::from(vec![span!("your score").white()]).centered());
                lines
                    .push(Line::from(GameoverWidget::format_number(self.current_score)).centered());
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::raw(""));

        let top_5 = self.high_scores.get_top_5(&self.game_mode);
        for i in 0..5 {
            let rank_idx = i + 1;
            let is_highlighted = self.highlighted_rank == Some(rank_idx);

            let rank_color = match rank_idx {
                1 => GOLD,
                2 => SILVER,
                3 => BRONZE,
                _ => Color::Rgb(160, 160, 160),
            };

            let rank_prefix = if is_highlighted {
                if rank_idx <= 3 {
                    format!("> #{}  ", rank_idx)
                } else {
                    format!(">  {}  ", rank_idx)
                }
            } else if rank_idx <= 3 {
                format!("  #{}  ", rank_idx)
            } else {
                format!("   {}  ", rank_idx)
            };

            if let Some(entry) = top_5.get(i) {
                let initials_str = format!("{:<6}", entry.initials.to_string());
                let score_str = format!("{:>10}", GameoverWidget::format_number(entry.score));

                if is_highlighted {
                    lines.push(
                        Line::from(vec![
                            span!("{}", rank_prefix).green().bold(),
                            span!("{}  ", initials_str).green().bold(),
                            span!("{} <", score_str).green().bold(),
                        ])
                        .centered(),
                    );
                } else {
                    lines.push(
                        Line::from(vec![
                            span!("{}", rank_prefix).fg(rank_color).bold(),
                            span!("{}  ", initials_str).white(),
                            span!("{}", score_str).yellow(),
                        ])
                        .centered(),
                    );
                }
            } else {
                lines.push(
                    Line::from(vec![
                        span!("{}", rank_prefix).fg(rank_color),
                        span!("------        ---").dark_gray(),
                    ])
                    .centered(),
                );
            }
        }

        if self.stage == GameoverStage::Menu {
            lines.push(Line::raw(""));

            for (i, option) in self.menu_options.iter().enumerate() {
                if i == self.option_index {
                    lines.push(Line::from(span!("- {} -", option).green().bold()).centered());
                } else {
                    lines.push(Line::from(option.clone()).centered());
                }
            }
        }

        let inner_area = block_area
            .inner(Margin::new(2, 1))
            .centered_vertically(constraint!(== lines.len() as u16));

        Clear.render(block_area, buf);
        Block::bordered()
            .border_style(Color::Rgb(60, 60, 60))
            .render(block_area, buf);

        Text::from(lines).render(inner_area, buf);
    }
}
