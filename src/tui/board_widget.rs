use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    blocks_manager::BlocksManager,
    board::Board,
    tui::{COLUMNS, ROWS},
};

#[derive(Default, PartialEq, Eq)]
pub enum BoardState {
    #[default]
    Pass,
    Continue,
    Brake,
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
            board: Board::new(COLUMNS, ROWS),
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
                return Ok(BoardState::Brake);
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

impl Widget for &mut BoardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board.render(area, buf);
    }
}
