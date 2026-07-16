use std::collections::HashSet;

use ratatui::style::Color;

use crate::board::Coords;
use strum::{EnumCount, VariantArray};

pub const ORANGE: Color = Color::Rgb(255, 127, 0);

#[derive(Debug, Clone, Copy, EnumCount, VariantArray)]
pub enum Block {
    Square,
    T,
    Line,
    L,
    J,
    Z,
    S,
}

impl Block {
    pub fn get_columns_len(&self) -> usize {
        self.shape().0[0].len()
    }

    pub fn get_coordinates(&self) -> HashSet<Coords> {
        self.shape()
            .0
            .iter()
            .enumerate()
            .flat_map(|(i, rows)| {
                rows.iter().enumerate().filter_map(move |(j, &symbol)| {
                    (symbol == "x").then_some((j, i, self.shape().1))
                })
            })
            .collect()
    }

    fn shape(&self) -> (&'static [&'static [&'static str]], Color) {
        match *self {
            Self::Square => (&[&["x", "x"], &["x", "x"]], Color::Yellow),
            Self::T => (&[&[".", "x", "."], &["x", "x", "x"]], Color::Magenta),
            Self::Line => (&[&["x", "x", "x", "x"]], Color::Cyan),
            Self::L => (&[&[".", ".", "x"], &["x", "x", "x"]], ORANGE),
            Self::J => (&[&["x", ".", "."], &["x", "x", "x"]], Color::Blue),
            Self::Z => (&[&["x", "x", "."], &[".", "x", "x"]], Color::Red),
            Self::S => (&[&[".", "x", "x"], &["x", "x", "."]], Color::Green),
        }
    }
}
