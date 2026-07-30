use std::{
    io,
    time::{Duration, Instant},
};

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

    tick_interval: Duration,
    blocks_manager: BlocksManager,
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

    pub fn run(&mut self) -> io::Result<BoardState> {
        use crossterm::event::{poll, read, KeyCode};

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_tick);
        self.last_tick = current_time;

        if self.board.is_paused {
            while poll(Duration::ZERO)? {
                if let Some(event) = read().map_or(None, |e| e.as_key_press_event()) {
                    match event.code {
                        KeyCode::Enter | KeyCode::Char('p') => {
                            self.board.pause();
                        }
                        KeyCode::Esc => {
                            return Ok(BoardState::Brake);
                        }
                        _ => (),
                    }
                }
            }

            let elapsed = current_time.elapsed();
            if elapsed < self.tick_interval {
                std::thread::sleep(self.tick_interval - elapsed);
            };

            return Ok(BoardState::Paused);
        }

        self.acc_time += delta_time;

        while poll(Duration::ZERO)? {
            if let Some(event) = read().map_or(None, |e| e.as_key_press_event()) {
                match event.code {
                    KeyCode::Left | KeyCode::Right => self.board.move_block_x_axis(event.code),
                    KeyCode::Down => {
                        let _ = self.board.move_block_down_or_set();
                    }
                    KeyCode::Char('z') | KeyCode::Char('x') => {
                        let _ = self.board.rotate_block(event.code);
                    }
                    KeyCode::Char(' ') => while self.board.move_block_down_or_set() {},
                    KeyCode::Enter | KeyCode::Char('p') => {
                        self.board.pause();
                        let elapsed = current_time.elapsed();
                        if elapsed < self.tick_interval {
                            std::thread::sleep(self.tick_interval - elapsed);
                        };
                        return Ok(BoardState::Paused);
                    }
                    KeyCode::Esc => {
                        return Ok(BoardState::Brake);
                    }
                    _ => (),
                }
            }
        }

        if !self.board.is_block_falling {
            let block = self.blocks_manager.get_next_block();
            if !self.board.spawn_next_block(block) {
                self.board.timer.pause();
                return Ok(BoardState::GameOver);
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

        Ok(BoardState::Pass)
    }
}

impl Widget for &BoardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board.render(area, buf);

        if self.board.is_paused {
            let block_area = area.centered(constraint!(== 50%), constraint!(== 5));
            let options_area = block_area
                .inner(Margin::new(1, 1))
                .centered_vertically(constraint!(== 1));

            let pause_text = text!["pause"].centered();

            Clear.render(block_area, buf);
            Block::bordered().render(block_area, buf);
            pause_text.render(options_area, buf);
        }
    }
}
