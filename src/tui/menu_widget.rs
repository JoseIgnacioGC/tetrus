use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    macros::span,
    style::Stylize,
    text::{Line, Span, Text, ToSpan},
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
    title: Line<'a>,
    option_index: usize,
    menu_options: [Span<'a>; 2],
}

impl<'a> MenuWidget<'a> {
    pub fn new(title: Line<'a>) -> Self {
        Self {
            title,
            option_index: 0,
            menu_options: ["endless".into(), "quit".into()],
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
        let mut menu_text = Text::from(self.title.clone());
        menu_text.push_line(Line::raw(""));

        for (i, option) in self.menu_options.iter().enumerate() {
            if i == self.option_index {
                menu_text.push_line(span!("- {} -", option).green());
            } else {
                menu_text.push_line(option.to_span());
            }
        }

        menu_text.centered().render(area, buf);
    }
}
