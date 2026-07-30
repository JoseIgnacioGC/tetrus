use crate::{blocks::Block, utils::timer::Timer};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use std::{
    cmp::{max, min},
    collections::HashSet,
    time::Duration,
};

pub const GOAL_MULTIPLIER: usize = 5;
pub const MOVEMENT_SETS: [(&str, usize); 5] = [
    ("", 0),
    ("single", 100),
    ("double", 300),
    ("triple", 500),
    ("quad", 800),
];

const MAX_FALL_SPEED: usize = 20;

pub type Coords = (u16, u16, Color);

#[derive(Default)]
pub struct Board {
    pub is_block_falling: bool,
    pub cleaned_lines: usize,
    pub score: usize,
    pub level: usize,
    pub fall_speed: Duration,
    pub timer: Timer,

    columns_len: u16,
    rows_len: u16,
    coordinates: HashSet<Coords>, // TODO: replace by a HashMap or Vector of HashSet maybe
    block_coordinates: HashSet<Coords>,
}

impl Board {
    pub fn new(columns_len: u16, rows_len: u16) -> Self {
        Self {
            level: 1,
            columns_len,
            rows_len,
            ..Default::default()
        }
    }

    pub fn new_game(&mut self) {
        self.is_block_falling = false;
        self.cleaned_lines = 0;
        self.score = 0;
        self.level = 1;
        self.fall_speed = Duration::ZERO;

        self.coordinates.clear();
        self.block_coordinates.clear();
        self.timer.reset();

        self.timer.start();
    }

    pub fn rotate_block(&mut self, key: KeyCode) -> bool {
        if self.block_coordinates.is_empty() {
            return false;
        }

        let rotated_block: HashSet<Coords> = self.rotate_block_coordinates(key);

        if rotated_block.len() != self.block_coordinates.len()
            || self.is_block_colliding_with_blocks(&rotated_block)
        {
            return false;
        }

        let _ = std::mem::replace(&mut self.block_coordinates, rotated_block);
        true
    }

    pub fn spawn_next_block(&mut self, block: &Block) -> bool {
        let center_coordinates_x =
            self.columns_len / 2 - block.get_columns_len() / 2 - (block.get_columns_len() % 2);
        let block_coordinates = block
            .get_coordinates()
            .iter()
            .map(|(x, y, color)| (x + center_coordinates_x, *y, *color))
            .collect();

        if self.is_block_colliding_with_blocks(&block_coordinates) {
            return false;
        }

        self.block_coordinates = block_coordinates;
        self.is_block_falling = true;

        self.update_metrics();

        true
    }

    pub fn move_block_x_axis(&mut self, key: KeyCode) {
        let moved_block: HashSet<Coords> = self
            .block_coordinates
            .iter()
            .map_while(|(x, y, color)| {
                if (*x == 0 && key == KeyCode::Left)
                    || (*x == self.columns_len - 1 && key == KeyCode::Right)
                {
                    return None;
                }
                let x = if key == KeyCode::Left { x - 1 } else { x + 1 };
                Some((x, *y, *color))
            })
            .collect();

        if moved_block.len() != self.block_coordinates.len()
            || self.is_block_colliding_with_blocks(&moved_block)
        {
            return;
        }
        self.block_coordinates = moved_block;
    }

    pub fn move_block_down_or_set(&mut self) -> bool {
        let moved_block: HashSet<Coords> = self
            .block_coordinates
            .iter()
            .map_while(|(x, y, color)| {
                let y = y + 1;
                (y < self.rows_len).then_some((*x, y, *color))
            })
            .collect();

        if moved_block.len() != self.block_coordinates.len() {
            self.coordinates.extend(&self.block_coordinates);
            self.block_coordinates.clear();
            self.clear_lines();
            self.is_block_falling = false;

            return false;
        }
        if self.is_block_colliding_with_blocks(&moved_block) {
            self.coordinates.extend(&self.block_coordinates);
            self.block_coordinates.clear();
            self.clear_lines();
            self.is_block_falling = false;

            return false;
        }
        self.block_coordinates = moved_block;

        true
    }

    fn is_block_colliding_with_blocks(&self, block_coordinates: &HashSet<Coords>) -> bool {
        let board_coords: HashSet<(u16, u16)> =
            self.coordinates.iter().map(|(x, y, _)| (*x, *y)).collect();
        let block_coords: HashSet<(u16, u16)> =
            block_coordinates.iter().map(|(x, y, _)| (*x, *y)).collect();
        !board_coords.is_disjoint(&block_coords)
            || block_coords.iter().any(|(_, y)| *y > self.rows_len)
    }

    fn clear_lines(&mut self) {
        let shorted_coords: Vec<HashSet<Coords>> = self.coordinates.iter().fold(
            vec![HashSet::new(); self.rows_len as usize],
            |mut acc: Vec<HashSet<Coords>>, coordinates| {
                let y = coordinates.1;
                acc[y as usize].insert(*coordinates);
                acc
            },
        );

        let mut cleaned_lines_counter = 0;
        let uncompleted_columns: Vec<&HashSet<Coords>> = shorted_coords
            .iter()
            .filter(|coords| {
                if coords.len() as u16 == self.columns_len {
                    self.coordinates.retain(|c| !coords.contains(c));
                    cleaned_lines_counter += 1;
                    return false;
                }
                !coords.is_empty()
            })
            .collect();

        self.score += MOVEMENT_SETS[cleaned_lines_counter].1 * self.level;
        self.cleaned_lines += cleaned_lines_counter;

        let shifted_coords = uncompleted_columns.iter().rev().enumerate().fold(
            HashSet::new(),
            |mut acc: HashSet<Coords>, (i, coords)| {
                let shifted: Vec<Coords> = coords
                    .iter()
                    .map(|(x, _, colors)| (*x, self.rows_len - 1 - (i as u16), *colors))
                    .collect();
                acc.extend(shifted);
                acc
            },
        );

        self.coordinates = shifted_coords;
    }

    // TODO: Improve blocks rotation behavior
    fn rotate_block_coordinates(&self, key: KeyCode) -> HashSet<Coords> {
        if self.block_coordinates.is_empty() {
            return HashSet::new();
        }

        let mut block_color: Option<Color> = None;
        let (y_axis_min_max_coords, x_axis_min_max_coords) = self.block_coordinates.iter().fold(
            ((u16::MAX, 0), (u16::MAX, 0)),
            |((y_min, y_max), (x_min, x_max)), (x, y, color)| {
                if block_color.is_none() {
                    block_color = Some(*color);
                }

                (
                    (min(y_min, *y), max(y_max, *y)),
                    (min(x_min, *x), max(x_max, *x)),
                )
            },
        );

        let max_row_len = (y_axis_min_max_coords.1 as usize - y_axis_min_max_coords.0 as usize) + 1;
        let max_column_len =
            (x_axis_min_max_coords.1 as usize - x_axis_min_max_coords.0 as usize) + 1;
        let matrix_len = max(max_row_len, max_column_len) as usize;

        let mut matrix: Vec<Vec<bool>> = vec![vec![false; matrix_len]; matrix_len];

        self.block_coordinates.iter().for_each(|(x, y, _)| {
            let x = x - x_axis_min_max_coords.0;
            let y = y - y_axis_min_max_coords.0;
            matrix[y as usize][x as usize] = true;
        });

        if key == KeyCode::Char('x') {
            matrix = matrix.into_iter().rev().collect::<Vec<Vec<bool>>>();

            matrix = (0usize..matrix[0].len())
                .map(|i| matrix.iter().map(|row| row[i]).collect())
                .collect();
        } else {
            matrix = (0usize..matrix[0].len())
                .map(|i| matrix.iter().map(|row| row[i]).collect())
                .collect();

            matrix = matrix.into_iter().rev().collect::<Vec<Vec<bool>>>();
        }

        let mut rotated_block = HashSet::<Coords>::new();

        matrix.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, value)| {
                if !*value {
                    return;
                }
                let x = x as u16 + x_axis_min_max_coords.0;
                let y = y as u16 + y_axis_min_max_coords.0;

                if x >= self.columns_len || y >= self.rows_len {
                    return;
                }

                rotated_block.insert((x, y, block_color.unwrap()));
            })
        });

        rotated_block
    }

    fn update_level(&mut self) {
        let curr_goal = self.level * GOAL_MULTIPLIER;
        if self.cleaned_lines >= curr_goal {
            self.level += 1;
        }
    }

    fn update_fall_speed(&mut self) {
        if self.level > MAX_FALL_SPEED {
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
        let board_width = self.columns_len * 2;
        let board_height = self.rows_len;

        let start_x = area.x + area.width.saturating_sub(board_width) / 2;
        let start_y = area.y + area.height.saturating_sub(board_height) / 2;

        let mut set_cell = |x: u16, y: u16, ch: char, style: Style| {
            let cell_x = start_x + (x * 2) + 1;
            let cell_y = start_y + y;

            if cell_x < area.right() && cell_y < area.bottom() {
                buf[(cell_x, cell_y)].set_char(ch).set_style(style);
            }
        };

        (0..self.rows_len)
            .flat_map(|y| (0..self.columns_len).map(move |x| (x, y)))
            .for_each(|(x, y)| {
                let symbol = if y == 0 { ' ' } else { '.' };
                set_cell(x, y, symbol, Style::default());
            });

        self.coordinates.iter().for_each(|&(x, y, color)| {
            set_cell(x, y, '■', Style::default().fg(color));
        });

        self.block_coordinates.iter().for_each(|&(x, y, color)| {
            set_cell(x, y, '□', Style::default().fg(color));
        });
    }
}
