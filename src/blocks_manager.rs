use crate::blocks::Block;

use rand::{rngs::SmallRng, seq::SliceRandom};
use strum::{EnumCount, VariantArray};

pub struct BlocksManager {
    blocks_buffer: [Block; Block::COUNT],
    current_index: usize,
    rng: SmallRng,
}

impl BlocksManager {
    pub fn new() -> Self {
        let mut rng = rand::make_rng();
        let mut blocks_buffer: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();

        blocks_buffer.shuffle(&mut rng);

        Self {
            blocks_buffer,
            current_index: 0,
            rng,
        }
    }

    pub fn get_next_block(&mut self) -> &Block {
        if self.current_index == Block::COUNT {
            self.blocks_buffer.shuffle(&mut self.rng);
            self.current_index = 0;
        }

        let next_block: &Block = self.blocks_buffer.get(self.current_index).unwrap();

        self.current_index += 1;

        next_block
    }
}
