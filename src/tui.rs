use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;

use crate::{blocks_manager::BlocksManager, board::Board};

const COLUMNS: usize = 10;
const ROWS: usize = 22;
const BLOCK_FALL_AWAIT_TIME: Duration = Duration::from_millis(500);

#[derive(Default)]
pub struct App;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut board = Board::new(COLUMNS, ROWS);
        let mut block_fall_start_time = Instant::now();
        let mut blocks_manager = BlocksManager::new();

        loop {
            if !board.is_block_falling {
                let block = blocks_manager.get_next_block();
                if board.try_insert_block(block).is_err() {
                    break;
                };
                block_fall_start_time = Instant::now();
            } else if block_fall_start_time.elapsed() < BLOCK_FALL_AWAIT_TIME
                    && /* FIX: touch any key triggers this */ event::poll(BLOCK_FALL_AWAIT_TIME)?
            {
                if let Some(event) = event::read().map_or(None, |e| e.as_key_press_event()) {
                    match event.code {
                        KeyCode::Left | KeyCode::Right => board.move_block_x_axis(event.code),
                        KeyCode::Down => {
                            let _ = board.try_move_block_down_or_set();
                        }
                        KeyCode::Char('z') | KeyCode::Char('x') => {
                            let _ = board.try_rotate_block(event.code);
                        }
                        KeyCode::Char(' ') => while board.try_move_block_down_or_set().is_ok() {},
                        KeyCode::Esc => break,
                        _ => continue,
                    }
                }
            } else {
                board.try_move_block_down_or_set().ok();
                block_fall_start_time = Instant::now();
            };

            terminal.draw(|frame| {
                frame.render_widget(&mut board, frame.area());
            })?;
        }

        ratatui::restore();
        Ok(())
    }
}
