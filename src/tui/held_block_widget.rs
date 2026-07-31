use ratatui::{
    buffer::Buffer,
    layout::Rect,
    macros::text,
    style::Style,
    widgets::Widget,
};

use crate::{
    blocks::{Block, Rotation},
    board::Board,
};

pub struct HeldBlockWidget {
    held_block: Option<Block>,
}

impl HeldBlockWidget {
    pub fn new() -> Self {
        Self { held_block: None }
    }

    pub fn copy_metrics(&mut self, board: &Board) {
        self.held_block = board.held_block;
    }
}

impl Widget for &HeldBlockWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hold_text = text!["", "", "hold"].right_aligned();
        hold_text.render(area, buf);

        if let Some(block) = self.held_block {
            let block_width = block.side_len() * 2;
            let start_x = area.right().saturating_sub(block_width);
            let start_y = area.y + 3;

            for (bx, by, color) in block.get_coordinates(Rotation::Deg0) {
                let cell_x = start_x + (bx * 2);
                let cell_y = start_y + by;

                if cell_x + 1 <= area.right() && cell_y < area.bottom() {
                    buf[(cell_x, cell_y)]
                        .set_char('□')
                        .set_style(Style::default().fg(color));
                }
            }
        }
    }
}
