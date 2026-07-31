use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    macros::{constraint, text},
    widgets::{Block, Clear, Widget},
};

use crate::{blocks_manager::BlocksManager, board::Board};

#[derive(Default, PartialEq, Eq)]
pub enum BoardState {
    #[default]
    Pass,
    Brake,
    GameOver,
    Paused,
}

pub struct BoardWidget {
    pub board: Board,
    pub blocks_manager: BlocksManager,

    tick_interval: Duration,
    last_tick: Instant,
    acc_time: Duration,
}

impl BoardWidget {
    pub fn new() -> Self {
        let tick_60fps_interval: Duration = Duration::from_secs_f32(1.0 / 60.0);

        Self {
            tick_interval: tick_60fps_interval,
            board: Board::new(),
            blocks_manager: BlocksManager::new(),
            last_tick: Instant::now(),
            acc_time: Duration::ZERO,
        }
    }

    pub fn new_game(&mut self) {
        self.board.new_game();
        self.last_tick = Instant::now();
        self.acc_time = Duration::ZERO;
        self.blocks_manager = BlocksManager::new();
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> BoardState {
        if self.board.is_paused {
            return match event.code {
                KeyCode::Enter | KeyCode::Char('p') | KeyCode::Char('P') => {
                    self.board.pause();
                    BoardState::Paused
                }
                KeyCode::Esc => BoardState::Brake,
                _ => BoardState::Paused,
            };
        }

        match event.code {
            KeyCode::Left | KeyCode::Right => {
                self.board.move_block_x_axis(event.code);
                BoardState::Pass
            }
            KeyCode::Down => {
                let _ = self.board.move_block_down_or_set();
                BoardState::Pass
            }
            KeyCode::Up
            | KeyCode::Char('z')
            | KeyCode::Char('Z')
            | KeyCode::Char('x')
            | KeyCode::Char('X')
            | KeyCode::Char('a')
            | KeyCode::Char('A') => {
                let _ = self.board.rotate_block(event.code);
                BoardState::Pass
            }
            KeyCode::Char(' ') => {
                while self.board.move_block_down_or_set() {
                    self.acc_time = Duration::ZERO;
                }
                BoardState::Pass
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                let _ = self.board.hold_block(&mut self.blocks_manager);
                BoardState::Pass
            }
            KeyCode::Enter | KeyCode::Char('p') | KeyCode::Char('P') => {
                self.board.pause();
                BoardState::Paused
            }
            KeyCode::Esc => BoardState::Brake,
            _ => BoardState::Pass,
        }
    }

    pub fn update(&mut self) -> BoardState {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_tick);
        self.last_tick = current_time;

        if self.board.is_paused {
            let elapsed = current_time.elapsed();
            if elapsed < self.tick_interval {
                std::thread::sleep(self.tick_interval - elapsed);
            };

            return BoardState::Paused;
        }

        self.acc_time += delta_time;

        if !self.board.is_block_falling {
            let block = self.blocks_manager.get_next_block();
            if !self.board.spawn_next_block(&block) {
                self.board.timer.pause();
                return BoardState::GameOver;
            };
        }

        while self.acc_time >= self.board.fall_speed {
            self.acc_time -= self.board.fall_speed;
            let _ = self.board.move_block_down_or_set();
        }

        let elapsed = current_time.elapsed();
        if elapsed < self.tick_interval {
            std::thread::sleep(self.tick_interval - elapsed);
        };

        BoardState::Pass
    }
}

impl Widget for &BoardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board.render(area, buf);

        if self.board.is_paused {
            let block_area = area.centered(constraint!(== 50%), constraint!(== 5));
            let text_area = block_area
                .inner(Margin::new(1, 1))
                .centered_vertically(constraint!(== 1));

            let pause_text = text!["pause"].centered();

            Clear.render(block_area, buf);
            Block::bordered().render(block_area, buf);
            pause_text.render(text_area, buf);
        }
    }
}
