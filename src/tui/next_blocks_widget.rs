use ratatui::{buffer::Buffer, layout::Rect, macros::text, style::Style, widgets::Widget};

use crate::{
    blocks::{Block, Rotation},
    blocks_manager::BlocksManager,
};

pub struct NextBlocksWidget {
    next_blocks: [Block; 5],
}

impl NextBlocksWidget {
    pub fn new() -> Self {
        Self {
            next_blocks: [Block::Square; 5],
        }
    }

    pub fn copy_metrics(&mut self, blocks_manager: &BlocksManager) {
        self.next_blocks = blocks_manager.get_next_blocks();
    }
}

impl Widget for &NextBlocksWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let next_text = text!["", "", "next"].left_aligned();
        next_text.render(area, buf);

        for (i, &block) in self.next_blocks.iter().enumerate() {
            let start_x = area.x;
            let start_y = area.y + 4 + (i as u16 * 3);

            for (bx, by, color) in block.get_coordinates(Rotation::Deg0) {
                let cell_x = start_x + (bx * 2);
                let cell_y = start_y + by;

                if cell_x + 1 < area.right() && cell_y < area.bottom() {
                    buf[(cell_x, cell_y)]
                        .set_char('□')
                        .set_style(Style::default().fg(color));
                }
            }
        }
    }
}
