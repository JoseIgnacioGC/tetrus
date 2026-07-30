use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    macros::span,
    style::Stylize,
    text::{Span, Text, ToSpan},
    widgets::Widget,
};

#[derive(Default, PartialEq, Eq)]
pub enum MenuState {
    #[default]
    Pass,
    EnterGame,
    Brake,
}

pub struct MenuWidget<'a> {
    option_index: usize,
    menu_options: [Span<'a>; 2],
}

impl<'a> MenuWidget<'a> {
    pub fn new() -> Self {
        Self {
            option_index: 0,
            menu_options: ["play".into(), "quit".into()],
        }
    }

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
        let menu_options: Text = self
            .menu_options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                if i == self.option_index {
                    return span!("- {} -", option).green();
                }

                option.to_span()
            })
            .collect();

        menu_options.centered().render(area, buf);
    }
}
