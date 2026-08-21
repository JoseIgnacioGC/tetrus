use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    macros::{constraint, span},
    style::Stylize,
    text::{Line, Span, Text, ToSpan},
    widgets::Widget,
};

#[derive(Default, PartialEq, Eq)]
pub enum MenuState {
    #[default]
    Pass,
    EnterGame,
    EnterGameWithPreset(crate::board::Grid, &'static [crate::blocks::Block], usize),
    Brake,
}

#[derive(PartialEq, Eq, Default)]
pub enum MenuScreen {
    #[default]
    Main,
    LearnMoves,
}

pub struct MenuWidget<'a> {
    title: Line<'a>,
    option_index: usize,
    menu_options: [Span<'a>; 3],
    screen: MenuScreen,
    learn_moves_index: usize,
    gravity: usize,
    learn_options: [Span<'a>; 8],
}

impl<'a> MenuWidget<'a> {
    pub fn new(title: Line<'a>) -> Self {
        Self {
            title,
            option_index: 0,
            menu_options: ["endless".into(), "learn moves".into(), "quit".into()],
            screen: MenuScreen::Main,
            learn_moves_index: 1,
            gravity: 1,
            learn_options: [
                "T-Spin Double Setup".into(),
                "T-Spin Triple Setup".into(),
                "Quad Clear Setup".into(),
                "L-Spin Setup".into(),
                "J-Spin Setup".into(),
                "S-Spin Setup".into(),
                "Z-Spin Setup".into(),
                "[←] back".into(),
            ],
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> MenuState {
        match self.screen {
            MenuScreen::Main => {
                let options_len = self.menu_options.len();
                match event.code {
                    KeyCode::Up => {
                        self.option_index = (self.option_index + options_len - 1) % options_len;
                        MenuState::Pass
                    }
                    KeyCode::Down => {
                        self.option_index = (self.option_index + 1) % options_len;
                        MenuState::Pass
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => match self.option_index {
                        0 => MenuState::EnterGame,
                        1 => {
                            self.screen = MenuScreen::LearnMoves;
                            self.learn_moves_index = 1;
                            MenuState::Pass
                        }
                        2 => MenuState::Brake,
                        _ => unreachable!(),
                    },
                    KeyCode::Esc => MenuState::Brake,
                    _ => MenuState::Pass,
                }
            }
            MenuScreen::LearnMoves => {
                let total_len = 1 + self.learn_options.len();
                match event.code {
                    KeyCode::Up => {
                        self.learn_moves_index =
                            (self.learn_moves_index + total_len - 1) % total_len;
                        MenuState::Pass
                    }
                    KeyCode::Down => {
                        self.learn_moves_index = (self.learn_moves_index + 1) % total_len;
                        MenuState::Pass
                    }
                    KeyCode::Left => {
                        if self.learn_moves_index == 0 {
                            if self.gravity > 0 {
                                self.gravity -= 1;
                            }
                            MenuState::Pass
                        } else {
                            self.screen = MenuScreen::Main;
                            MenuState::Pass
                        }
                    }
                    KeyCode::Right => {
                        if self.learn_moves_index == 0 && self.gravity < 20 {
                            self.gravity += 1;
                        }
                        MenuState::Pass
                    }
                    KeyCode::Esc => {
                        self.screen = MenuScreen::Main;
                        MenuState::Pass
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => match self.learn_moves_index {
                        0 => MenuState::Pass,
                        1 => MenuState::EnterGameWithPreset(
                            crate::board::presets::t_spin_double(),
                            &[crate::blocks::Block::T],
                            self.gravity,
                        ),
                        2 => MenuState::EnterGameWithPreset(
                            crate::board::presets::t_spin_triple(),
                            &[crate::blocks::Block::T],
                            self.gravity,
                        ),
                        3 => MenuState::EnterGameWithPreset(
                            crate::board::presets::quad_clear(),
                            &[crate::blocks::Block::Line],
                            self.gravity,
                        ),
                        4 => MenuState::EnterGameWithPreset(
                            crate::board::presets::l_spin(),
                            &[crate::blocks::Block::L],
                            self.gravity,
                        ),
                        5 => MenuState::EnterGameWithPreset(
                            crate::board::presets::j_spin(),
                            &[crate::blocks::Block::J],
                            self.gravity,
                        ),
                        6 => MenuState::EnterGameWithPreset(
                            crate::board::presets::s_spin(),
                            &[crate::blocks::Block::S],
                            self.gravity,
                        ),
                        7 => MenuState::EnterGameWithPreset(
                            crate::board::presets::z_spin(),
                            &[crate::blocks::Block::Z],
                            self.gravity,
                        ),
                        8 => {
                            self.screen = MenuScreen::Main;
                            MenuState::Pass
                        }
                        _ => unreachable!(),
                    },
                    _ => MenuState::Pass,
                }
            }
        }
    }
}

impl<'a> Widget for &mut MenuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.screen == MenuScreen::LearnMoves {
            let max_visible = (area.height.saturating_sub(6) as usize).max(4);
            let total_options = 1 + self.learn_options.len();
            let scroll_offset = if self.learn_moves_index >= max_visible {
                self.learn_moves_index - max_visible + 1
            } else {
                0
            };
            let end_offset = (scroll_offset + max_visible).min(total_options);

            let mut menu_text = Text::from(self.title.clone());
            menu_text.push_line(Line::raw(""));
            menu_text.push_line(Line::from("LEARN MOVES".bold()).centered());
            menu_text.push_line(Line::raw(""));

            if scroll_offset > 0 {
                menu_text.push_line(Line::from("▲".dim()).centered());
            }

            for i in scroll_offset..end_offset {
                let is_selected = i == self.learn_moves_index;
                if i == 0 {
                    let text = format!("gravity {}", self.gravity);
                    if is_selected {
                        menu_text.push_line(span!("- {} -", text).green().bold());
                    } else {
                        menu_text.push_line(Line::from(text));
                    }
                    menu_text.push_line(Line::raw(""));
                } else {
                    let option_idx = i - 1;
                    if option_idx == 7 {
                        if is_selected {
                            menu_text.push_line(
                                Line::from(vec![
                                    span!("- ").green().bold(),
                                    span!("[←]").cyan().bold(),
                                    span!(" back -").green().bold(),
                                ])
                                .centered(),
                            );
                        } else {
                            menu_text.push_line(
                                Line::from(vec![
                                    span!("[←]").cyan(),
                                    span!(" back"),
                                ])
                                .centered(),
                            );
                        }
                    } else {
                        let option = &self.learn_options[option_idx];
                        if is_selected {
                            menu_text.push_line(span!("- {} -", option).green().bold());
                        } else {
                            menu_text.push_line(option.to_span());
                        }
                    }
                }
            }

            if end_offset < total_options {
                menu_text.push_line(Line::from("▼".dim()).centered());
            }

            let lines_count = menu_text.lines.len() as u16;
            let centered_area = area.centered_vertically(constraint!(== lines_count));
            menu_text.centered().render(centered_area, buf);
            return;
        }

        let mut menu_text = Text::from(self.title.clone());
        menu_text.push_line(Line::raw(""));

        for (i, option) in self.menu_options.iter().enumerate() {
            if i == self.option_index {
                menu_text.push_line(span!("- {} -", option).green().bold());
            } else {
                menu_text.push_line(option.to_span());
            }
        }

        let lines_count = menu_text.lines.len() as u16;
        let centered_area = area.centered_vertically(constraint!(== lines_count));
        menu_text.centered().render(centered_area, buf);
    }
}
