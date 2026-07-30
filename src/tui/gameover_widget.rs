use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    macros::{constraint, span, text},
    style::Stylize,
    text::Span,
    widgets::{Block, Clear, Widget},
};

#[derive(Default, PartialEq, Eq)]
pub enum GameoverState {
    #[default]
    Pass,
    EnterGame,
    Brake,
}

pub struct GameoverWidget<'a> {
    option_index: usize,
    menu_options: [Span<'a>; 2],
}

impl<'a> GameoverWidget<'a> {
    pub fn new() -> Self {
        Self {
            option_index: 0,
            menu_options: ["again?".into(), "quit".into()],
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> GameoverState {
        let options_len = self.menu_options.iter().len();

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
            KeyCode::Esc => GameoverState::Brake,
            _ => GameoverState::Pass,
        }
    }
}

impl<'a> Widget for &mut GameoverWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let options_len = self.menu_options.iter().len() as u16 + 2;
        let block_area = area.centered(constraint!(== 30%), constraint!(== options_len + 6));
        let options_area = block_area
            .inner(Margin::new(1, 1))
            .centered_vertically(constraint!(== options_len));

        let mut gameover_text = text!["Game Over", ""].centered();
        let mut menu_options = self.menu_options.clone();

        menu_options[self.option_index] = span!("- {} -", menu_options[self.option_index]).green();

        gameover_text.extend(menu_options);

        Clear.render(block_area, buf);
        Block::bordered().render(block_area, buf);
        gameover_text.centered().render(options_area, buf);
    }
}
