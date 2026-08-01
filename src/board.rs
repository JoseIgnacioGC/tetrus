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
pub const LOCK_DELAY_DURATION: Duration = Duration::from_millis(500);
pub const MAX_LOCK_RESETS: usize = 15;
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
    pub combo_count: usize,
    pub combo_timer: Option<Instant>,
    pub lock_delay_timer: Option<Instant>,
    pub lock_resets: usize,
    pub last_action_was_rotation: bool,
    pub is_b2b: bool,

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
        self.combo_count = 0;
        self.combo_timer = None;
        self.lock_delay_timer = None;
        self.lock_resets = 0;
        self.last_action_was_rotation = false;
        self.is_b2b = false;

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
                self.last_action_was_rotation = true;
                self.update_lock_delay_on_move();
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
        self.lock_delay_timer = None;
        self.lock_resets = 0;
        self.last_action_was_rotation = false;

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
            self.last_action_was_rotation = false;
            self.update_lock_delay_on_move();
        }
    }

    pub fn is_grounded(&self) -> bool {
        let Some(block) = self.current_block else {
            return false;
        };
        let (x, y) = self.current_square_coord;
        !self.can_place(block, (x, y + 1), self.current_rotation)
    }

    fn update_lock_delay_on_move(&mut self) {
        if self.is_grounded() {
            if self.lock_resets < MAX_LOCK_RESETS {
                self.lock_delay_timer = Some(Instant::now());
                self.lock_resets += 1;
            }
        } else {
            self.lock_delay_timer = None;
        }
    }

    pub fn detect_t_spin(&self) -> bool {
        let Some(block) = self.current_block else {
            return false;
        };
        if block != Block::T || !self.last_action_was_rotation {
            return false;
        }

        let (x, y) = self.current_square_coord;
        let center_x = x + 1;
        let center_y = y + 1;

        let corners = [
            (center_x - 1, center_y - 1),
            (center_x + 1, center_y - 1),
            (center_x - 1, center_y + 1),
            (center_x + 1, center_y + 1),
        ];

        let mut occupied_corners = 0;
        for (cx, cy) in corners {
            if cx < 0
                || cx >= COLUMNS as isize
                || cy < 0
                || cy >= ROWS as isize
                || (cy >= 0 && self.board[cy as usize][cx as usize].is_some())
            {
                occupied_corners += 1;
            }
        }

        occupied_corners >= 3
    }

    pub fn is_board_empty(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(Option::is_none))
    }

    pub fn lock_current_block(&mut self) {
        let Some(block) = self.current_block else {
            return;
        };
        let is_t_spin = self.detect_t_spin();
        let (x, y) = self.current_square_coord;
        for (block_x, block_y, color) in block.get_coordinates(self.current_rotation) {
            let board_x = x + block_x as isize;
            let board_y = y + block_y as isize;
            if board_x >= 0 && board_x < COLUMNS as isize && board_y >= 0 && board_y < ROWS as isize
            {
                self.board[board_y as usize][board_x as usize] = Some(color);
            }
        }
        self.current_block = None;
        self.is_block_falling = false;
        self.lock_delay_timer = None;
        self.lock_resets = 0;
        self.clear_lines_with_tspin(is_t_spin);
    }

    pub fn check_lock_delay(&mut self) {
        if self.is_grounded() {
            if let Some(timer) = self.lock_delay_timer {
                if timer.elapsed() >= LOCK_DELAY_DURATION || self.lock_resets >= MAX_LOCK_RESETS {
                    self.lock_current_block();
                }
            } else {
                self.lock_delay_timer = Some(Instant::now());
            }
        } else {
            self.lock_delay_timer = None;
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
            self.last_action_was_rotation = false;
            if self.is_grounded() {
                if self.lock_delay_timer.is_none() {
                    self.lock_delay_timer = Some(Instant::now());
                }
            } else {
                self.lock_delay_timer = None;
            }
            true
        } else {
            self.check_lock_delay();
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

    fn clear_lines_with_tspin(&mut self, is_t_spin: bool) {
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

        let is_perfect_clear = cleared > 0 && self.is_board_empty();
        let is_difficult = cleared == 4 || (is_t_spin && cleared > 0);

        let mut base_score = 0;
        let mut movement_name: &'static str = "";

        if is_t_spin {
            match cleared {
                0 => {
                    base_score = 400;
                    movement_name = "T-Spin";
                }
                1 => {
                    base_score = 800;
                    movement_name = if self.is_b2b {
                        "B2B T-Spin Single"
                    } else {
                        "T-Spin Single"
                    };
                }
                2 => {
                    base_score = 1200;
                    movement_name = if self.is_b2b {
                        "B2B T-Spin Double"
                    } else {
                        "T-Spin Double"
                    };
                }
                3 => {
                    base_score = 1600;
                    movement_name = if self.is_b2b {
                        "B2B T-Spin Triple"
                    } else {
                        "T-Spin Triple"
                    };
                }
                _ => {}
            }
        } else {
            match cleared {
                1 => {
                    base_score = 100;
                    movement_name = "single";
                }
                2 => {
                    base_score = 300;
                    movement_name = "double";
                }
                3 => {
                    base_score = 500;
                    movement_name = "triple";
                }
                4 => {
                    base_score = 800;
                    movement_name = if self.is_b2b { "B2B quad" } else { "quad" };
                }
                _ => {}
            }
        }

        if is_difficult {
            if self.is_b2b {
                base_score = (base_score as f64 * 1.5) as usize;
            }
            self.is_b2b = true;
        } else if cleared > 0 {
            self.is_b2b = false;
        }

        if is_perfect_clear {
            let pc_bonus = match cleared {
                1 => 800,
                2 => 1200,
                3 => 1800,
                4 => 2000,
                _ => 2000,
            };
            base_score += pc_bonus;
            movement_name = "Perfect Clear!";
        }

        if !movement_name.is_empty() {
            self.last_movement = movement_name;
            self.last_movement_timer = Some(Instant::now());
        }

        if cleared > 0 {
            if self.combo_count > 0 {
                let combo_bonus = 50 * self.combo_count * self.level;
                self.score += combo_bonus;
            }
            self.combo_count += 1;
            self.combo_timer = Some(Instant::now());
        } else {
            self.combo_count = 0;
        }

        self.score += base_score * self.level;
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

    pub fn current_combo(&self) -> Option<(usize, Duration)> {
        if let Some(timer) = self.combo_timer {
            let elapsed = timer.elapsed();
            if elapsed < Duration::from_millis(1500) && self.combo_count > 1 {
                return Some((self.combo_count - 1, elapsed));
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
