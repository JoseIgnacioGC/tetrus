use crate::blocks::Block;
use crossterm::{
    event::KeyCode,
    style::{self, Color, Stylize},
};
use std::{
    cmp::{max, min},
    collections::HashSet,
};

pub type Coords = (usize, usize, Color);

#[derive(Default)]
pub struct Board {
    columns_len: usize,
    rows_len: usize,
    title: String,
    coordinates: HashSet<Coords>, // TODO: replace by a HashMap or Vector of HashSet maybe
    block_coordinates: HashSet<Coords>,
    board: Box<[Box<[style::StyledContent<&'static str>]>]>,
    pub is_block_falling: bool,
}

impl Board {
    pub fn new(columns_len: usize, rows_len: usize) -> Self {
        let title = [
            "T".red(),
            "E".with(Color::Rgb {
                r: 255,
                g: 127,
                b: 0,
            }),
            "T".yellow(),
            "R".green(),
            "U".cyan(),
            "S".magenta(),
        ]
        .map(|s| s.to_string())
        .join("");

        Self {
            columns_len,
            rows_len,
            title,
            board: vec![vec!["".stylize(); columns_len].into_boxed_slice(); rows_len]
                .into_boxed_slice(),
            ..Default::default()
        }
    }

    fn is_block_collinding_with_blocks(&self, block_coordinates: &HashSet<Coords>) -> bool {
        let board_coords: HashSet<(usize, usize)> =
            self.coordinates.iter().map(|(x, y, _)| (*x, *y)).collect();
        let block_coords: HashSet<(usize, usize)> =
            block_coordinates.iter().map(|(x, y, _)| (*x, *y)).collect();
        !board_coords.is_disjoint(&block_coords)
            || block_coords.iter().any(|(_, y)| *y > self.rows_len)
    }

    fn clear_lines(&mut self) {
        let shorted_coords: Vec<HashSet<Coords>> = self.coordinates.iter().fold(
            vec![HashSet::new(); self.rows_len],
            |mut acc: Vec<HashSet<Coords>>, coordinates| {
                let y = coordinates.1;
                acc[y].insert(*coordinates);
                acc
            },
        );
        let uncompleted_columns: Vec<&HashSet<Coords>> = shorted_coords
            .iter()
            .filter(|coords| {
                if coords.len() == self.columns_len {
                    self.coordinates.retain(|c| !coords.contains(c));
                    return false;
                };
                !coords.is_empty()
            })
            .collect();

        let shifted_coords = uncompleted_columns.iter().rev().enumerate().fold(
            HashSet::new(),
            |mut acc: HashSet<Coords>, (i, coords)| {
                let shifted: Vec<Coords> = coords
                    .iter()
                    .map(|(x, _, colors)| (*x, self.rows_len - 1 - i, *colors))
                    .collect();
                acc.extend(shifted);
                acc
            },
        );

        self.coordinates = shifted_coords;
    }

    fn rotate_block_coordinates(&self, key: KeyCode) -> HashSet<Coords> {
        let mut block_color: Option<Color> = None;
        let (y_axis_min_max_coords, x_axis_min_max_coords) = self.block_coordinates.iter().fold(
            ((usize::MAX, 0), (usize::MAX, 0)),
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

        let max_row_len = (y_axis_min_max_coords.1 - y_axis_min_max_coords.0) + 1;
        let max_column_len = (x_axis_min_max_coords.1 - x_axis_min_max_coords.0) + 1;
        let matrix_len = max(max_row_len, max_column_len);

        let mut matrix: Vec<Vec<usize>> = vec![vec![0; matrix_len]; matrix_len];

        self.block_coordinates.iter().for_each(|(x, y, _)| {
            let x = x - x_axis_min_max_coords.0;
            let y = y - y_axis_min_max_coords.0;
            matrix[y][x] = 1;
        });

        if key == KeyCode::Char('x') {
            matrix = matrix.into_iter().rev().collect::<Vec<Vec<usize>>>();

            matrix = (0usize..matrix[0].len())
                .map(|i| matrix.iter().map(|row| row[i]).collect())
                .collect();
        } else {
            matrix = (0usize..matrix[0].len())
                .map(|i| matrix.iter().map(|row| row[i]).collect())
                .collect();

            matrix = matrix.into_iter().rev().collect::<Vec<Vec<usize>>>();
        }

        let mut rotated_block = HashSet::<Coords>::new();

        matrix.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, value)| {
                if *value != 1 {
                    return;
                }
                let x = x + x_axis_min_max_coords.0;
                let y = y + y_axis_min_max_coords.0;

                if x >= self.columns_len || y >= self.rows_len {
                    return;
                }

                rotated_block.insert((x, y, block_color.unwrap()));
            })
        });

        rotated_block
    }

    pub fn try_rotate_block(&mut self, key: KeyCode) -> Result<(), &str> {
        // mave should not return an error
        let rotated_block: HashSet<Coords> = self.rotate_block_coordinates(key);

        if rotated_block.len() != self.block_coordinates.len()
            || self.is_block_collinding_with_blocks(&rotated_block)
        {
            return Err("Cannot rotate left");
        }

        let _ = std::mem::replace(&mut self.block_coordinates, rotated_block);
        Ok(())
    }

    pub fn try_insert_block(&mut self, block: &Block) -> Result<(), &str> {
        let center_coordinates_x =
            self.columns_len / 2 - block.get_columns_len() / 2 - (block.get_columns_len() % 2);
        let block_coordinates = block.get_coordinates();
        let block_coordinates = block_coordinates
            .iter()
            .map(|(x, y, color)| (x + center_coordinates_x, *y, *color))
            .collect();

        if self.is_block_collinding_with_blocks(&block_coordinates) {
            return Err("no more blocks can be inserted");
        };

        self.block_coordinates = block_coordinates.clone();
        self.is_block_falling = true;
        Ok(())
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
            || self.is_block_collinding_with_blocks(&moved_block)
        {
            return;
        }
        self.block_coordinates = moved_block;
    }

    pub fn try_move_block_down_or_set(&mut self) -> Result<(), &str> {
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

            return Err("block collides with shape");
        }
        if self.is_block_collinding_with_blocks(&moved_block) {
            self.coordinates.extend(&self.block_coordinates);
            self.block_coordinates.clear();
            self.clear_lines();
            self.is_block_falling = false;

            return Err("block collides with a block");
        }
        self.block_coordinates = moved_block;
        Ok(())
    }

    pub fn get_formated_board(&mut self, term_width: usize) -> String {
        for x in 0..self.columns_len {
            self.board[0][x] = " ".stylize()
        }
        for y in 1..self.rows_len {
            for x in 0..self.columns_len {
                self.board[y][x] = ".".stylize()
            }
        }

        self.coordinates
            .iter()
            .for_each(|&(x, y, color)| self.board[y][x] = "■".with(color));
        self.block_coordinates
            .iter()
            .for_each(|&(x, y, color)| self.board[y][x] = "□".with(color));

        let padding_left = " ".repeat(term_width / 2 - 3);
        let title = format!("{}{}", padding_left, self.title);

        let padding_left = " ".repeat(term_width / 2 - self.columns_len);
        let board = self
            .board
            .iter()
            .map(|row| {
                row.iter()
                    .fold(String::new(), |acc, s| format!("{} {}", acc, s))
            })
            .map(|s| format!("{}{}", padding_left, s))
            .collect::<Vec<String>>()
            .join("\n");
        format!("{}\n{}", title, board)
    }
}
