use std::collections::HashSet;

use crossterm::style::{self, Color, Stylize};

use crate::blocks::Block;

#[derive(Default)]
pub struct Board {
    columns_len: usize,
    rows_len: usize,
    coordinates: HashSet<(usize, usize, Color)>,
    block_coordinates: HashSet<(usize, usize, Color)>,
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

    fn is_block_collinding_with_blocks(
        &self,
        block_coordinates: &HashSet<(usize, usize, Color)>,
    ) -> bool {
        let board_coordinates: HashSet<(usize, usize)> =
            self.coordinates.iter().map(|(x, y, _)| (*x, *y)).collect();
        let block_coordinates: HashSet<(usize, usize)> =
            block_coordinates.iter().map(|(x, y, _)| (*x, *y)).collect();
        !board_coordinates.is_disjoint(&block_coordinates)
            || block_coordinates.iter().any(|(_, y)| *y > self.rows_len)
    }

    pub fn insert_block(&mut self, block: &Block) -> Result<(), &str> {
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

    pub fn move_block_x_axis(&mut self) {
        let moved_block: HashSet<(usize, usize, Color)> = self
            .block_coordinates
            .iter()
            .map_while(|(x, y, color)| {
                let y = y + 1;
                (y <= self.rows_len).then_some((*x, y, *color))
            })
            .collect();

        if moved_block.len() != self.block_coordinates.len() {
            self.coordinates.extend(&self.block_coordinates);
            return;
        }
        if self.is_block_collinding_with_blocks(&moved_block) {
            self.coordinates.extend(&self.block_coordinates);
            return;
        }
        self.block_coordinates = moved_block;
    }

    pub fn move_block_down_or_set(&mut self) -> Result<(), &str> {
        let moved_block: HashSet<(usize, usize, Color)> = self
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
            self.is_block_falling = false;

            return Err("block collides with shape");
        }
        if self.is_block_collinding_with_blocks(&moved_block) {
            self.coordinates.extend(&self.block_coordinates);
            self.block_coordinates.clear();
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

        shape
            .iter()
            .map(|row| {
                row.iter()
                    .fold(String::new(), |acc, s| format!("{} {}", acc, s))
            })
            // .map(|s| format!("{: ^term_width$}", s))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
