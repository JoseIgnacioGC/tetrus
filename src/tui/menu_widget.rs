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
    #[cfg(debug_assertions)]
    EnterGameWithGrid(crate::board::Grid),
    Brake,
}

#[cfg(debug_assertions)]
#[derive(PartialEq, Eq, Default)]
pub enum MenuScreen {
    #[default]
    Main,
    DebugBoards,
}

pub struct MenuWidget<'a> {
    title: Line<'a>,
    option_index: usize,
    #[cfg(debug_assertions)]
    menu_options: [Span<'a>; 3],
    #[cfg(not(debug_assertions))]
    menu_options: [Span<'a>; 2],
    #[cfg(debug_assertions)]
    screen: MenuScreen,
    #[cfg(debug_assertions)]
    debug_board_index: usize,
    #[cfg(debug_assertions)]
    debug_options: [Span<'a>; 8],
}

impl<'a> MenuWidget<'a> {
    pub fn new(title: Line<'a>) -> Self {
        Self {
            title,
            option_index: 0,
            #[cfg(debug_assertions)]
            menu_options: ["endless".into(), "debug boards".into(), "quit".into()],
            #[cfg(not(debug_assertions))]
            menu_options: ["endless".into(), "quit".into()],
            #[cfg(debug_assertions)]
            screen: MenuScreen::Main,
            #[cfg(debug_assertions)]
            debug_board_index: 0,
            #[cfg(debug_assertions)]
            debug_options: [
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

    #[cfg(debug_assertions)]
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
                            self.screen = MenuScreen::DebugBoards;
                            self.debug_board_index = 0;
                            MenuState::Pass
                        }
                        2 => MenuState::Brake,
                        _ => unreachable!(),
                    },
                    KeyCode::Esc => MenuState::Brake,
                    _ => MenuState::Pass,
                }
            }
            MenuScreen::DebugBoards => {
                let options_len = self.debug_options.len();
                match event.code {
                    KeyCode::Up => {
                        self.debug_board_index =
                            (self.debug_board_index + options_len - 1) % options_len;
                        MenuState::Pass
                    }
                    KeyCode::Down => {
                        self.debug_board_index = (self.debug_board_index + 1) % options_len;
                        MenuState::Pass
                    }
                    KeyCode::Left | KeyCode::Esc => {
                        self.screen = MenuScreen::Main;
                        MenuState::Pass
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => match self.debug_board_index {
                        0 => MenuState::EnterGameWithGrid(crate::board::presets::t_spin_double()),
                        1 => MenuState::EnterGameWithGrid(crate::board::presets::t_spin_triple()),
                        2 => MenuState::EnterGameWithGrid(crate::board::presets::quad_clear()),
                        3 => MenuState::EnterGameWithGrid(crate::board::presets::l_spin()),
                        4 => MenuState::EnterGameWithGrid(crate::board::presets::j_spin()),
                        5 => MenuState::EnterGameWithGrid(crate::board::presets::s_spin()),
                        6 => MenuState::EnterGameWithGrid(crate::board::presets::z_spin()),
                        7 => {
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

    #[cfg(not(debug_assertions))]
    pub fn handle_key_event(&mut self, event: KeyEvent) -> MenuState {
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
                1 => MenuState::Brake,
                _ => unreachable!(),
            },
            KeyCode::Esc => MenuState::Brake,
            _ => MenuState::Pass,
        }
    }
}

impl<'a> Widget for &mut MenuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        #[cfg(debug_assertions)]
        if self.screen == MenuScreen::DebugBoards {
            let max_visible = (area.height.saturating_sub(6) as usize).max(4);
            let total_options = self.debug_options.len();
            let scroll_offset = if self.debug_board_index >= max_visible {
                self.debug_board_index - max_visible + 1
            } else {
                0
            };
            let end_offset = (scroll_offset + max_visible).min(total_options);

            let mut menu_text = Text::from(self.title.clone());
            menu_text.push_line(Line::raw(""));
            menu_text.push_line(Line::from("DEBUG BOARDS".bold()).centered());
            menu_text.push_line(Line::raw(""));

            if scroll_offset > 0 {
                menu_text.push_line(Line::from("▲".dim()).centered());
            }

            for i in scroll_offset..end_offset {
                let is_selected = i == self.debug_board_index;
                if i == 7 {
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
                    let option = &self.debug_options[i];
                    if is_selected {
                        menu_text.push_line(span!("- {} -", option).green().bold());
                    } else {
                        menu_text.push_line(option.to_span());
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
