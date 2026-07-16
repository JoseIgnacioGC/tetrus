use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, KeyCode};
use ratatui::{
    macros::{constraint, horizontal, line, text, vertical},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

use crate::{blocks, blocks_manager::BlocksManager, board::Board};

const COLUMNS: usize = 10;
const ROWS: usize = 22;
const BLOCK_FALL_AWAIT_TIME: Duration = Duration::from_millis(500);

pub struct App {
    title: Line<'static>,
}

impl App {
    pub fn new() -> Self {
        let title = line![
            "T".red(),
            "E".fg(blocks::ORANGE),
            "T".yellow(),
            "R".green(),
            "U".cyan(),
            "S".magenta(),
        ]
        .centered();

        Self { title }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut board = Board::new(COLUMNS, ROWS);
        let mut block_fall_start_time = Instant::now();
        let mut blocks_manager = BlocksManager::new();

        loop {
            if !board.is_block_falling {
                let block = blocks_manager.get_next_block();
                if !board.insert_block(block) {
                    break;
                };
                block_fall_start_time = Instant::now();
            } else if block_fall_start_time.elapsed() < BLOCK_FALL_AWAIT_TIME
                    && /* TODO: touch any key triggers this */ event::poll(BLOCK_FALL_AWAIT_TIME)?
            {
                if let Some(event) = event::read().map_or(None, |e| e.as_key_press_event()) {
                    match event.code {
                        KeyCode::Left | KeyCode::Right => board.move_block_x_axis(event.code),
                        KeyCode::Down => {
                            let _ = board.move_block_down_or_set();
                        }
                        KeyCode::Char('z') | KeyCode::Char('x') => {
                            let _ = board.rotate_block(event.code);
                        }
                        KeyCode::Char(' ') => while board.move_block_down_or_set() {},
                        KeyCode::Esc => break,
                        _ => continue,
                    }
                }
            } else {
                let _ = board.move_block_down_or_set();
                block_fall_start_time = Instant::now();
            };

            terminal.draw(|frame| self.render(frame, &mut board))?;
        }

        ratatui::restore();
        Ok(())
    }

    fn render(&self, frame: &mut Frame, board: &mut Board) {
        let [title_area, game_area] = vertical![== 3,== ROWS as u16].areas(frame.area());
        let [left_area, board_area, next_blocks_area] =
            horizontal![*= 1, == (COLUMNS * 2 + 3) as u16, *= 1].areas(game_area);
        let [hold_area, metrics_area] = vertical![== 100%, == 8].areas(left_area);

        frame.render_widget(
            &self.title,
            title_area.centered_vertically(constraint!(== 1)),
        );
        frame.render_widget(board, board_area);
        frame.render_widget(
            Paragraph::new(text![
                "lv\n",
                "²\n",
                "score\n",
                "⁴ˑ²⁶⁶ˑ⁵⁶⁷\n",
                "lines\n",
                "²⁰⁄³⁰\n",
                "time\n",
                "1:01.02",
            ])
            .right_aligned(),
            metrics_area,
        );

        frame.render_widget(
            Paragraph::new("hold")
                .block(Block::default())
                .right_aligned(),
            hold_area,
        );
        frame.render_widget(
            Paragraph::new("next")
                .block(Block::default())
                .left_aligned(),
            next_blocks_area,
        );
    }
}
