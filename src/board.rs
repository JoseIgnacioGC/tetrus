use crate::{
    blocks::{Block, Rotation},
    blocks_manager::BlocksManager,
    constants::{
        COLUMNS, COMBO_NOTIFICATION_DURATION, GOAL_MULTIPLIER, LOCK_DELAY_FRAMES_DURATION,
        MAX_DELAY_FRAMES_LOCK_RESETS, MAX_FALL_SPEED_LEVEL, ROWS,
    },
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
pub type Grid = [[Option<Color>; COLUMNS as usize]; ROWS as usize];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayState {
    #[default]
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePiece {
    pub block: Block,
    pub rotation: Rotation,
    pub coord: (isize, isize),
    pub last_action_was_rotation: bool,
}

impl ActivePiece {
    pub fn new(block: Block, coord: (isize, isize)) -> Self {
        Self {
            block,
            rotation: Rotation::Deg0,
            coord,
            last_action_was_rotation: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HoldState {
    pub block: Option<Block>,
    pub can_hold: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LockDelay {
    pub timer: Option<Instant>,
    pub resets: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LastMovement {
    pub name: &'static str,
    pub timer: Option<Instant>,
    pub b2b_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Combo {
    pub count: usize,
    pub timer: Option<Instant>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GameStats {
    pub score: usize,
    pub cleaned_lines: usize,
    pub level: usize,
    pub fall_speed: Duration,
    pub b2b_count: usize,
}

#[derive(Default)]
pub struct Board {
    pub play_state: PlayState,
    pub stats: GameStats,
    pub timer: Timer,
    pub active_piece: Option<ActivePiece>,
    pub hold_state: HoldState,
    pub lock_delay: LockDelay,
    pub last_movement_state: LastMovement,
    pub combo: Combo,

    board: Grid,
}

impl Board {
    pub fn new() -> Self {
        Self {
            stats: GameStats {
                level: 1,
                ..Default::default()
            },
            hold_state: HoldState {
                can_hold: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn new_game(&mut self) {
        self.play_state = PlayState::Playing;
        self.stats = GameStats {
            level: 1,
            fall_speed: Duration::ZERO,
            score: 0,
            cleaned_lines: 0,
            b2b_count: 0,
        };

        self.board.iter_mut().for_each(|row| row.fill(None));
        self.active_piece = None;
        self.hold_state = HoldState {
            block: None,
            can_hold: true,
        };
        self.last_movement_state = LastMovement::default();
        self.combo = Combo::default();
        self.lock_delay = LockDelay::default();

        self.timer.reset();
        self.timer.start();
    }

    pub fn new_with_grid(&mut self, grid: Grid) {
        self.new_game();
        self.board = grid;
    }

    pub fn new_with_grid_and_gravity(&mut self, grid: Grid, gravity: usize) {
        self.new_game();
        self.board = grid;
        self.stats.level = gravity;
        self.update_fall_speed();
    }

    pub fn is_paused(&self) -> bool {
        self.play_state == PlayState::Paused
    }

    pub fn is_block_falling(&self) -> bool {
        self.active_piece.is_some()
    }

    pub fn current_rotation(&self) -> Rotation {
        self.active_piece
            .map(|p| p.rotation)
            .unwrap_or(Rotation::Deg0)
    }

    pub fn pause(&mut self) {
        if self.is_paused() {
            self.timer.start();
            self.play_state = PlayState::Playing;
        } else {
            self.timer.pause();
            self.play_state = PlayState::Paused;
        }
    }

    pub fn get_ghost_coord(&self) -> Option<(isize, isize)> {
        let piece = self.active_piece?;
        let (x, mut ghost_y) = piece.coord;

        while self.can_place(piece.block, (x, ghost_y + 1), piece.rotation) {
            ghost_y += 1;
        }

        Some((x, ghost_y))
    }

    pub fn hold_block(&mut self, blocks_manager: &mut BlocksManager) -> bool {
        if !self.hold_state.can_hold || self.is_paused() {
            return false;
        }

        let Some(current_piece) = self.active_piece else {
            return false;
        };

        self.hold_state.can_hold = false;

        let target_block = if let Some(prev_held) = self.hold_state.block {
            self.hold_state.block = Some(current_piece.block);
            prev_held
        } else {
            self.hold_state.block = Some(current_piece.block);
            blocks_manager.get_next_block()
        };

        let pos_x = (COLUMNS as isize - target_block.side_len() as isize) / 2;
        self.active_piece = Some(ActivePiece::new(target_block, (pos_x, 0)));

        true
    }

    pub fn rotate_block(&mut self, key: KeyCode) -> bool {
        let Some(ref mut piece) = self.active_piece else {
            return false;
        };

        let next_rotation = match key {
            KeyCode::Char('z') | KeyCode::Char('Z') => piece.rotation.rotate_counter_clockwise(),
            KeyCode::Char('a') | KeyCode::Char('A') => piece.rotation.rotate_180(),
            _ => piece.rotation.rotate_clockwise(),
        };

        let (x, y) = piece.coord;
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

        let block = piece.block;
        for (dx, dy) in KICK_OFFSETS {
            let test_coord = (x + dx, y + dy);
            if self.can_place(block, test_coord, next_rotation) {
                if let Some(ref mut p) = self.active_piece {
                    p.coord = test_coord;
                    p.rotation = next_rotation;
                    p.last_action_was_rotation = true;
                }
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

        self.active_piece = Some(ActivePiece::new(*block, (pos_x, pos_y)));
        self.hold_state.can_hold = true;
        self.lock_delay = LockDelay::default();

        self.update_metrics();

        true
    }

    pub fn move_block_x_axis(&mut self, key: KeyCode) {
        let Some(ref mut piece) = self.active_piece else {
            return;
        };

        let (x, y) = piece.coord;
        let next_x = match key {
            KeyCode::Left => x - 1,
            KeyCode::Right => x + 1,
            _ => return,
        };

        let block = piece.block;
        let rotation = piece.rotation;
        if self.can_place(block, (next_x, y), rotation) {
            if let Some(ref mut p) = self.active_piece {
                p.coord = (next_x, y);
                p.last_action_was_rotation = false;
            }
            self.update_lock_delay_on_move();
        }
    }

    pub fn is_grounded(&self) -> bool {
        let Some(piece) = self.active_piece else {
            return false;
        };
        let (x, y) = piece.coord;
        !self.can_place(piece.block, (x, y + 1), piece.rotation)
    }

    fn update_lock_delay_on_move(&mut self) {
        if self.is_grounded() {
            if self.lock_delay.resets < MAX_DELAY_FRAMES_LOCK_RESETS {
                self.lock_delay.timer = Some(Instant::now());
                self.lock_delay.resets += 1;
            }
        } else {
            self.lock_delay.timer = None;
        }
    }

    pub fn detect_t_spin(&self) -> bool {
        let Some(piece) = self.active_piece else {
            return false;
        };
        if piece.block != Block::T || !piece.last_action_was_rotation {
            return false;
        }

        let (x, y) = piece.coord;
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
        let Some(piece) = self.active_piece else {
            return;
        };
        let is_t_spin = self.detect_t_spin();
        let (x, y) = piece.coord;
        for (block_x, block_y, color) in piece.block.get_coordinates(piece.rotation) {
            let board_x = x + block_x as isize;
            let board_y = y + block_y as isize;
            if board_x >= 0 && board_x < COLUMNS as isize && board_y >= 0 && board_y < ROWS as isize
            {
                self.board[board_y as usize][board_x as usize] = Some(color);
            }
        }
        self.active_piece = None;
        self.lock_delay = LockDelay::default();
        self.clear_lines(is_t_spin);
    }

    pub fn check_lock_delay(&mut self) {
        if self.is_grounded() {
            if let Some(timer) = self.lock_delay.timer {
                if timer.elapsed() >= LOCK_DELAY_FRAMES_DURATION
                    || self.lock_delay.resets >= MAX_DELAY_FRAMES_LOCK_RESETS
                {
                    self.lock_current_block();
                }
            } else {
                self.lock_delay.timer = Some(Instant::now());
            }
        } else {
            self.lock_delay.timer = None;
        }
    }

    pub fn move_block_down_or_set(&mut self) -> bool {
        let Some(piece) = self.active_piece else {
            return false;
        };

        let (x, y) = piece.coord;
        let next_pos = (x, y + 1);

        if self.can_place(piece.block, next_pos, piece.rotation) {
            if let Some(ref mut p) = self.active_piece {
                p.coord = next_pos;
                p.last_action_was_rotation = false;
            }
            if self.is_grounded() {
                if self.lock_delay.timer.is_none() {
                    self.lock_delay.timer = Some(Instant::now());
                }
            } else {
                self.lock_delay.timer = None;
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

    fn clear_lines(&mut self, is_t_spin: bool) {
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
                    movement_name = if self.stats.b2b_count > 0 {
                        "B2B T-Spin Single"
                    } else {
                        "T-Spin Single"
                    };
                }
                2 => {
                    base_score = 1200;
                    movement_name = if self.stats.b2b_count > 0 {
                        "B2B T-Spin Double"
                    } else {
                        "T-Spin Double"
                    };
                }
                3 => {
                    base_score = 1600;
                    movement_name = if self.stats.b2b_count > 0 {
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
                    movement_name = if self.stats.b2b_count > 0 {
                        "B2B quad"
                    } else {
                        "quad"
                    };
                }
                _ => {}
            }
        }

        if is_difficult {
            if self.stats.b2b_count > 0 {
                base_score = (base_score as f64 * 1.5) as usize;
            }
            self.stats.b2b_count += 1;
        } else if cleared > 0 {
            self.stats.b2b_count = 0;
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
            self.last_movement_state = LastMovement {
                name: movement_name,
                timer: Some(Instant::now()),
                b2b_count: self.stats.b2b_count,
            };
        }

        if cleared > 0 {
            if self.combo.count > 0 {
                let combo_bonus = 50 * self.combo.count * self.stats.level;
                self.stats.score += combo_bonus;
            }
            self.combo.count += 1;
            self.combo.timer = Some(Instant::now());
        } else {
            self.combo.count = 0;
        }

        self.stats.score += base_score * self.stats.level;
        self.stats.cleaned_lines += cleared;
    }

    pub fn last_movement(&self) -> Option<(&'static str, usize, Duration)> {
        if let Some(timer) = self.last_movement_state.timer {
            let elapsed = timer.elapsed();
            if elapsed < COMBO_NOTIFICATION_DURATION {
                return Some((
                    self.last_movement_state.name,
                    self.last_movement_state.b2b_count,
                    elapsed,
                ));
            }
        }
        None
    }

    pub fn current_combo(&self) -> Option<(usize, Duration)> {
        if let Some(timer) = self.combo.timer {
            let elapsed = timer.elapsed();
            if elapsed < COMBO_NOTIFICATION_DURATION && self.combo.count > 1 {
                return Some((self.combo.count - 1, elapsed));
            }
        }
        None
    }

    fn update_level(&mut self) {
        if self.stats.level == 0 {
            return;
        }

        let curr_goal = self.stats.level * GOAL_MULTIPLIER;
        if self.stats.cleaned_lines >= curr_goal {
            self.stats.level += 1;
        }
    }

    fn update_fall_speed(&mut self) {
        if self.stats.level == 0 {
            self.stats.fall_speed = Duration::ZERO;
            return;
        }

        if self.stats.level > MAX_FALL_SPEED_LEVEL {
            return;
        }

        self.stats.fall_speed = Duration::from_secs_f32(
            (0.8 - ((self.stats.level as f32 - 1.0) * 0.007)).powf(self.stats.level as f32 - 1.0),
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

        if let Some(piece) = self.active_piece {
            let (square_x, square_y) = piece.coord;
            let active_coords = piece.block.get_coordinates(piece.rotation);

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

pub fn grid_from_str(s: &str) -> Grid {
    let mut grid: Grid = [[None; COLUMNS as usize]; ROWS as usize];
    let lines: Vec<&str> = s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let num_lines = lines.len();
    let start_row = (ROWS as usize).saturating_sub(num_lines);

    for (row_idx, line) in lines.iter().enumerate() {
        let grid_row = start_row + row_idx;
        if grid_row >= ROWS as usize {
            break;
        }
        for (col_idx, c) in line.chars().enumerate() {
            if col_idx >= COLUMNS as usize {
                break;
            }
            grid[grid_row][col_idx] = match c {
                'X' | '#' => Some(Color::DarkGray),
                'I' => Some(Color::Cyan),
                'O' => Some(Color::Yellow),
                'T' => Some(Color::Magenta),
                'S' => Some(Color::Green),
                'Z' => Some(Color::Red),
                'J' => Some(Color::Blue),
                'L' => Some(Color::Rgb(255, 127, 0)),
                _ => None,
            };
        }
    }
    grid
}

pub mod presets {
    use super::{grid_from_str, Grid};

    pub fn t_spin_double() -> Grid {
        grid_from_str(
            "XXXX.X.XXX\n\
             XXXX...XXX\n\
             XXXXXXXXXX",
        )
    }

    pub fn t_spin_triple() -> Grid {
        grid_from_str(
            "XXXX..XXXX\n\
             XXXX.XXXXX\n\
             XXXX.XXXXX\n\
             XXXX..XXXX",
        )
    }

    pub fn quad_clear() -> Grid {
        grid_from_str(
            "XXXXXXXXX.\n\
             XXXXXXXXX.\n\
             XXXXXXXXX.\n\
             XXXXXXXXX.",
        )
    }

    pub fn l_spin() -> Grid {
        grid_from_str(
            "XXXX..XXXX\n\
             XXXX.XXXXX\n\
             XXXX..XXXX",
        )
    }

    pub fn j_spin() -> Grid {
        grid_from_str(
            "XXXX..XXXX\n\
             XXXXX.XXXX\n\
             XXXX..XXXX",
        )
    }

    pub fn s_spin() -> Grid {
        grid_from_str(
            "XXXX..XXXX\n\
             XXX..XXXXX\n\
             XXXX..XXXX",
        )
    }

    pub fn z_spin() -> Grid {
        grid_from_str(
            "XXXX..XXXX\n\
             XXXXX..XXX\n\
             XXXX..XXXX",
        )
    }
}
