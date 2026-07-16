use crate::blocks::{Block, BLOCKS};

use rand::{rngs::SmallRng, seq::SliceRandom};

pub struct BlocksManager {
    blocks_buffer: [Block; 7],
    current_index: usize,
    rng: SmallRng,
}

impl BlocksManager {
    pub fn new() -> Self {
        let mut rng = rand::make_rng();
        let mut blocks_buffer = BLOCKS;

        blocks_buffer.shuffle(&mut rng);

        Self {
            blocks_buffer,
            current_index: 0,
            rng,
        }
    }

    pub fn get_next_block(&mut self) -> &Block {
        if self.current_index == BLOCKS.len() {
            self.blocks_buffer.shuffle(&mut self.rng);
            self.current_index = 0;
        }

        let next_block: &Block = self.blocks_buffer.get(self.current_index).unwrap();

        self.current_index += 1;

        next_block
    }
}
