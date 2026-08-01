use ratatui::{
    buffer::Buffer,
    layout::Rect,
    macros::text,
    style::{Color, Style},
    widgets::Widget,
};

use crate::{
    blocks::{Block, Rotation},
    board::Board,
};

pub struct HeldBlockWidget {
    held_block: Option<Block>,
    can_hold: bool,
}

impl HeldBlockWidget {
    pub fn new() -> Self {
        Self {
            held_block: None,
            can_hold: true,
        }
    }

    pub fn copy_metrics(&mut self, board: &Board) {
        self.held_block = board.held_block;
        self.can_hold = board.can_hold;
    }
}

impl Widget for &HeldBlockWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hold_text = text!["", "", "hold"].right_aligned();
        hold_text.render(area, buf);

        if let Some(block) = self.held_block {
            let block_width = block.side_len() * 2;
            let start_x = area.right().saturating_sub(block_width);
            let start_y = area.y + 4;

            for (block_x, block_y, color) in block.get_coordinates(Rotation::Deg0) {
                let cell_x = start_x + (block_x * 2);
                let cell_y = start_y + block_y;

                if cell_x + 1 <= area.right() && cell_y < area.bottom() {
                    let style = if self.can_hold {
                        Style::default().fg(color)
                    } else {
                        Style::default().fg(Color::White).dim()
                    };

                    buf[(cell_x, cell_y)].set_char('□').set_style(style);
                }
            }
        }
    }
}
