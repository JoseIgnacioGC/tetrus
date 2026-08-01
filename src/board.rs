use crate::{
    blocks::{Block, Rotation},
    blocks_manager::BlocksManager,
    tui::{COLUMNS, ROWS},
    utils::timer::Timer,
};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use std::time::{Duration, Instant};

pub type Coords = (u16, u16, Color);

pub const GOAL_MULTIPLIER: usize = 5;
pub const MOVEMENT_SETS: [(&str, usize); 5] = [
    ("", 0),
    ("single", 100),
    ("double", 300),
    ("triple", 500),
    ("quad", 800),
];

const MAX_FALL_SPEED_LEVEL: usize = 20;

#[derive(Default)]
pub struct Board {
    pub is_block_falling: bool,
    pub is_paused: bool,
    pub cleaned_lines: usize,
    pub score: usize,
    pub level: usize,
    pub fall_speed: Duration,
    pub timer: Timer,
    pub current_rotation: Rotation,
    pub held_block: Option<Block>,
    pub can_hold: bool,
    pub last_movement: &'static str,
    pub last_movement_timer: Option<Instant>,

    board: [[Option<Color>; COLUMNS as usize]; ROWS as usize],
    current_block: Option<Block>,
    current_square_coord: (isize, isize),
}

impl Board {
    pub fn new() -> Self {
        Self {
            level: 1,
            can_hold: true,
            ..Default::default()
        }
    }

    pub fn new_game(&mut self) {
        self.is_block_falling = false;
        self.is_paused = false;
        self.cleaned_lines = 0;
        self.score = 0;
        self.level = 1;
        self.fall_speed = Duration::ZERO;

        self.board.iter_mut().for_each(|row| row.fill(None));
        self.current_block = None;
        self.current_rotation = Rotation::Deg0;
        self.current_square_coord = (0, 0);
        self.held_block = None;
        self.can_hold = true;
        self.last_movement = "";
        self.last_movement_timer = None;

        self.timer.reset();
        self.timer.start();
    }

    pub fn pause(&mut self) {
        if self.is_paused {
            self.is_paused = false;
            self.timer.start();
        } else {
            self.is_paused = true;
            self.timer.pause();
        }
    }

    pub fn get_ghost_coord(&self) -> Option<(isize, isize)> {
        let block = self.current_block?;
        let (x, mut ghost_y) = self.current_square_coord;

        while self.can_place(block, (x, ghost_y + 1), self.current_rotation) {
            ghost_y += 1;
        }

        Some((x, ghost_y))
    }

    pub fn hold_block(&mut self, blocks_manager: &mut BlocksManager) -> bool {
        if !self.can_hold || self.is_paused {
            return false;
        }

        let Some(current) = self.current_block else {
            return false;
        };

        self.can_hold = false;

        if let Some(prev_held) = self.held_block {
            self.held_block = Some(current);
            let pos_x = (COLUMNS as isize - prev_held.side_len() as isize) / 2;
            let pos_y = 0;
            self.current_block = Some(prev_held);
            self.current_rotation = Rotation::Deg0;
            self.current_square_coord = (pos_x, pos_y);
        } else {
            self.held_block = Some(current);
            let next_block = blocks_manager.get_next_block();
            let pos_x = (COLUMNS as isize - next_block.side_len() as isize) / 2;
            let pos_y = 0;
            self.current_block = Some(next_block);
            self.current_rotation = Rotation::Deg0;
            self.current_square_coord = (pos_x, pos_y);
        }

        true
    }

    pub fn rotate_block(&mut self, key: KeyCode) -> bool {
        let Some(block) = self.current_block else {
            return false;
        };

        let next_rotation = match key {
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                self.current_rotation.rotate_counter_clockwise()
            }
            KeyCode::Char('a') | KeyCode::Char('A') => self.current_rotation.rotate_180(),
            _ => self.current_rotation.rotate_clockwise(),
        };

        let (x, y) = self.current_square_coord;
        const KICK_OFFSETS: [(isize, isize); 10] = [
            (0, 0),
            (1, 0),
            (-1, 0),
            (2, 0),
            (-2, 0),
            (0, -1),
            (1, -1),
            (-1, -1),
            (0, -2),
            (0, 1),
        ];

        for (dx, dy) in KICK_OFFSETS {
            let test_coord = (x + dx, y + dy);
            if self.can_place(block, test_coord, next_rotation) {
                self.current_square_coord = test_coord;
                self.current_rotation = next_rotation;
                return true;
            }
        }

        false
    }

    pub fn spawn_next_block(&mut self, block: &Block) -> bool {
        let pos_x = (COLUMNS as isize - block.side_len() as isize) / 2;
        let pos_y = 0;

        if !self.can_place(*block, (pos_x, pos_y), Rotation::Deg0) {
            return false;
        }

        self.current_block = Some(*block);
        self.current_rotation = Rotation::Deg0;
        self.current_square_coord = (pos_x, pos_y);
        self.is_block_falling = true;
        self.can_hold = true;

        self.update_metrics();

        true
    }

    pub fn move_block_x_axis(&mut self, key: KeyCode) {
        let Some(block) = self.current_block else {
            return;
        };

        let (x, y) = self.current_square_coord;
        let next_x = match key {
            KeyCode::Left => x - 1,
            KeyCode::Right => x + 1,
            _ => return,
        };

        if self.can_place(block, (next_x, y), self.current_rotation) {
            self.current_square_coord = (next_x, y);
        }
    }

    pub fn move_block_down_or_set(&mut self) -> bool {
        let Some(block) = self.current_block else {
            return false;
        };

        let (x, y) = self.current_square_coord;
        let next_pos = (x, y + 1);

        if self.can_place(block, next_pos, self.current_rotation) {
            self.current_square_coord = next_pos;
            true
        } else {
            for (block_x, block_y, color) in block.get_coordinates(self.current_rotation) {
                let board_x = x + block_x as isize;
                let board_y = y + block_y as isize;
                if board_x >= 0
                    && board_x < COLUMNS as isize
                    && board_y >= 0
                    && board_y < ROWS as isize
                {
                    self.board[board_y as usize][board_x as usize] = Some(color);
                }
            }
            self.current_block = None;
            self.is_block_falling = false;
            self.clear_lines();
            false
        }
    }

    fn can_place(&self, block: Block, coord: (isize, isize), rotation: Rotation) -> bool {
        let (square_x, square_y) = coord;
        block
            .get_coordinates(rotation)
            .into_iter()
            .all(|(block_x, block_y, _)| {
                let board_x = square_x + block_x as isize;
                let board_y = square_y + block_y as isize;

                board_x >= 0
                    && board_x < COLUMNS as isize
                    && board_y >= 0
                    && board_y < ROWS as isize
                    && self.board[board_y as usize][board_x as usize].is_none()
            })
    }

    fn clear_lines(&mut self) {
        let mut cleared = 0;

        for y in (0..ROWS as usize).rev() {
            if self.board[y].iter().all(Option::is_some) {
                cleared += 1;
            } else if cleared > 0 {
                self.board[y + cleared] = self.board[y];
            }
        }

        for y in 0..cleared {
            self.board[y] = [None; COLUMNS as usize];
        }

        if cleared > 0 {
            self.last_movement = MOVEMENT_SETS[cleared.min(4)].0;
            self.last_movement_timer = Some(Instant::now());
        }

        self.score += MOVEMENT_SETS[cleared].1 * self.level;
        self.cleaned_lines += cleared;
    }

    pub fn last_movement(&self) -> Option<(&str, Duration)> {
        if let Some(timer) = self.last_movement_timer {
            let elapsed = timer.elapsed();
            if elapsed < Duration::from_millis(1500) {
                return Some((self.last_movement, elapsed));
            }
        }
        None
    }

    fn update_level(&mut self) {
        let curr_goal = self.level * GOAL_MULTIPLIER;
        if self.cleaned_lines >= curr_goal {
            self.level += 1;
        }
    }

    fn update_fall_speed(&mut self) {
        if self.level > MAX_FALL_SPEED_LEVEL {
            return;
        }

        self.fall_speed = Duration::from_secs_f32(
            (0.8 - ((self.level as f32 - 1.0) * 0.007)).powf(self.level as f32 - 1.0),
        )
    }

    fn update_metrics(&mut self) {
        self.update_level();
        self.update_fall_speed();
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let board_width = COLUMNS * 2;
        let board_height = ROWS;

        let start_x = area.x + area.width.saturating_sub(board_width) / 2;
        let start_y = area.y + area.height.saturating_sub(board_height) / 2;

        let mut set_cell = |x: usize, y: usize, ch: char, style: Style| {
            let cell_x = start_x + (x as u16 * 2) + 1;
            let cell_y = start_y + y as u16;

            if cell_x < area.right() && cell_y < area.bottom() {
                let cell = &mut buf[(cell_x, cell_y)];
                cell.reset();
                cell.set_char(ch).set_style(style);
            }
        };

        for y in 0..ROWS {
            for x in 0..COLUMNS {
                if y == 0 || y == 1 {
                    set_cell(x as usize, y as usize, ' ', Style::default());
                } else {
                    set_cell(x as usize, y as usize, '.', Style::default().dim());
                }
            }
        }

        for y in 0..ROWS {
            for x in 0..COLUMNS {
                if let Some(color) = self.board[y as usize][x as usize] {
                    set_cell(x as usize, y as usize, '■', Style::default().fg(color));
                }
            }
        }

        if let Some(block) = self.current_block {
            let (square_x, square_y) = self.current_square_coord;
            let active_coords = block.get_coordinates(self.current_rotation);

            if let Some((ghost_x, ghost_y)) = self.get_ghost_coord() {
                if (ghost_x, ghost_y) != (square_x, square_y) {
                    for (block_x, block_y, _color) in active_coords {
                        let board_x = ghost_x + block_x as isize;
                        let board_y = ghost_y + block_y as isize;
                        let overlaps_active = active_coords.iter().any(|(ax, ay, _)| {
                            square_x + *ax as isize == board_x && square_y + *ay as isize == board_y
                        });

                        if !overlaps_active
                            && board_x >= 0
                            && board_x < COLUMNS as isize
                            && board_y >= 0
                            && board_y < ROWS as isize
                        {
                            set_cell(
                                board_x as usize,
                                board_y as usize,
                                '□',
                                Style::default().fg(Color::White).dim(),
                            );
                        }
                    }
                }
            }

            for (block_x, block_y, color) in active_coords {
                let board_x = square_x + block_x as isize;
                let board_y = square_y + block_y as isize;
                if board_x >= 0
                    && board_x < COLUMNS as isize
                    && board_y >= 0
                    && board_y < ROWS as isize
                {
                    set_cell(
                        board_x as usize,
                        board_y as usize,
                        '□',
                        Style::default().fg(color),
                    );
                }
            }
        }
    }
}
