use ratatui::style::Color;

use strum::{EnumCount, VariantArray};

use crate::{board::Coords, colors::ORANGE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Rotation {
    #[default]
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    pub const fn rotate_clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg90,
            Self::Deg90 => Self::Deg180,
            Self::Deg180 => Self::Deg270,
            Self::Deg270 => Self::Deg0,
        }
    }

    pub const fn rotate_counter_clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg270,
            Self::Deg90 => Self::Deg0,
            Self::Deg180 => Self::Deg90,
            Self::Deg270 => Self::Deg180,
        }
    }

    pub const fn rotate_180(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg180,
            Self::Deg90 => Self::Deg270,
            Self::Deg180 => Self::Deg0,
            Self::Deg270 => Self::Deg90,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumCount, VariantArray, PartialEq, Eq)]
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
    pub const fn side_len(self) -> u16 {
        match self {
            Self::Square => 2,
            Self::Line => 4,
            _ => 3,
        }
    }

    pub const fn color(self) -> Color {
        match self {
            Self::Square => Color::Yellow,
            Self::T => Color::Magenta,
            Self::Line => Color::Cyan,
            Self::L => ORANGE,
            Self::J => Color::Blue,
            Self::Z => Color::Red,
            Self::S => Color::Green,
        }
    }

    const fn base_coordinates(self) -> &'static [(u16, u16); 4] {
        match self {
            Self::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
            Self::T => &[(1, 0), (0, 1), (1, 1), (2, 1)],
            Self::Line => &[(0, 1), (1, 1), (2, 1), (3, 1)],
            Self::L => &[(2, 0), (0, 1), (1, 1), (2, 1)],
            Self::J => &[(0, 0), (0, 1), (1, 1), (2, 1)],
            Self::Z => &[(0, 0), (1, 0), (1, 1), (2, 1)],
            Self::S => &[(1, 0), (2, 0), (0, 1), (1, 1)],
        }
    }

    pub fn get_coordinates(self, rotation: Rotation) -> [Coords; 4] {
        let len = self.side_len();
        let color = self.color();

        self.base_coordinates().map(|(x, y)| {
            let (rotated_x, rotated_y) = match rotation {
                Rotation::Deg0 => (x, y),
                Rotation::Deg90 => (len - 1 - y, x),
                Rotation::Deg180 => (len - 1 - x, len - 1 - y),
                Rotation::Deg270 => (y, len - 1 - x),
            };
            (rotated_x, rotated_y, color)
        })
    }
}
