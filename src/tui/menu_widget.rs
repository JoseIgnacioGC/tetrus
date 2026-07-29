use std::{io, time::Duration};

use crossterm::event::{poll, read, KeyCode};
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

    pub fn run(&mut self) -> io::Result<MenuState> {
        let options_len = self.menu_options.len();

        while poll(Duration::ZERO)? {
            if let Some(event) = read().map_or(None, |e| e.as_key_press_event()) {
                match event.code {
                    KeyCode::Up => {
                        self.option_index = (self.option_index + options_len - 1) % options_len;
                    }
                    KeyCode::Down => {
                        self.option_index = (self.option_index + 1) % options_len;
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        return match self.option_index {
                            0 => Ok(MenuState::EnterGame),
                            1 => Ok(MenuState::Brake),
                            _ => unreachable!(),
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(MenuState::Brake);
                    }
                    _ => {}
                }
            }
        }

        Ok(MenuState::Pass)
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
