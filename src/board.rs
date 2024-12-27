use std::collections::HashSet;

use crossterm::{
    event::KeyCode,
    style::{self, Color, Stylize},
};

use crate::blocks::Block;

type Coords = (usize, usize, Color);

#[derive(Default)]
pub struct Board {
    columns_len: usize,
    rows_len: usize,
    coordinates: HashSet<Coords>, // TODO: replace by a HashMap or Vector of HashSet
    block_coordinates: HashSet<Coords>,
    // board: Vec<Vec<style::StyledContent<String>>> or a slice, // TODO: create a board for efficiency
    pub is_block_falling: bool,
}

impl Board {
    pub fn new(columns_len: usize, rows_len: usize) -> Self {
        Self {
            columns_len,
            rows_len,
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

    fn remove_completed_lines(&mut self) {
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

    pub fn try_rotate_block(&self) -> Result<(), &str> {
        todo!()
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
            self.remove_completed_lines();
            self.is_block_falling = false;

            return Err("block collides with shape");
        }
        if self.is_block_collinding_with_blocks(&moved_block) {
            self.coordinates.extend(&self.block_coordinates);
            self.block_coordinates.clear();
            self.remove_completed_lines();
            self.is_block_falling = false;

            return Err("block collides with a block");
        }
        self.block_coordinates = moved_block;
        Ok(())
    }

    pub fn get_formated_board(&self, term_width: usize) -> String {
        let mut shape: Vec<Vec<style::StyledContent<&str>>> = [
            vec![vec![" ".stylize(); self.columns_len]; 1],
            vec![vec![".".stylize(); self.columns_len]; self.rows_len - 1],
        ]
        .concat();
        self.coordinates
            .iter()
            .for_each(|&(x, y, color)| shape[y][x] = "■".with(color));
        self.block_coordinates
            .iter()
            // fix: aout of bounds bug
            .for_each(|&(x, y, color)| shape[y][x] = "□".with(color));

        let padding_left = " ".repeat(term_width / 2 - 2);
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
        let title = format!("{}{}", padding_left, title);

        let padding_left = " ".repeat(term_width / 2 - self.columns_len);
        let board = shape
            .iter()
            .map(|row| {
                row.iter()
                    .fold(String::new(), |acc, s| format!("{} {}", acc, s))
            })
            // .map(|s| format!("{:-^term_width$}", s))
            .map(|s| format!("{}{}", padding_left, s))
            .collect::<Vec<String>>()
            .join("\n");
        format!("{}\n{}", title, board)
    }
}
