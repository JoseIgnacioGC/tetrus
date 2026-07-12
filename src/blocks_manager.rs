use crate::blocks::{Block, BLOCKS};

use rand::{rngs::SmallRng, seq::SliceRandom};
use std::cell::RefCell;

thread_local! {
    static RNG_CELL: RefCell<SmallRng> = RefCell::new(rand::make_rng());
}

pub struct BlocksManager {
    blocks_buffer: [Block; 7],
    current_index: usize,
}

impl BlocksManager {
    pub fn new() -> Self {
        let mut manager = Self {
            blocks_buffer: BLOCKS,
            current_index: 0,
        };

        manager.shuffle_blocks_buffer();

        manager
    }

    pub fn get_next_block(&mut self) -> &Block {
        if self.current_index == BLOCKS.len() {
            self.shuffle_blocks_buffer();
            self.current_index = 0;
        }

        let next_block: &Block = self.blocks_buffer.get(self.current_index).unwrap();

        self.current_index += 1;

        next_block
    }

    fn shuffle_blocks_buffer(&mut self) {
        RNG_CELL.with(|rng_cell| {
            self.blocks_buffer.shuffle(&mut rng_cell.borrow_mut());
        });
    }
}
