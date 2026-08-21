use crate::blocks::Block;

use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use strum::{EnumCount, VariantArray};

pub struct BlocksManager {
    bags: [[Block; Block::COUNT]; 2],
    active_bag: u8,
    current_index: u8,
    rng: SmallRng,
    seed: Option<u64>,
}

impl BlocksManager {
    pub fn new() -> Self {
        let mut rng = rand::make_rng();
        let mut bag_1: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();
        let mut bag_2: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();

        bag_1.shuffle(&mut rng);
        bag_2.shuffle(&mut rng);

        Self {
            bags: [bag_1, bag_2],
            active_bag: 0,
            current_index: 0,
            rng,
            seed: None,
        }
    }

    #[cfg(any(debug_assertions, feature = "vhs"))]
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut bag_1: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();
        let mut bag_2: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();

        bag_1.shuffle(&mut rng);
        bag_2.shuffle(&mut rng);

        Self {
            bags: [bag_1, bag_2],
            active_bag: 0,
            current_index: 0,
            rng,
            seed: Some(seed),
        }
    }

    pub fn reset(&mut self) {
        if let Some(seed) = self.seed {
            self.rng = SmallRng::seed_from_u64(seed);
        } else {
            self.rng = rand::make_rng();
        }

        let mut bag_1: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();
        let mut bag_2: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();

        bag_1.shuffle(&mut self.rng);
        bag_2.shuffle(&mut self.rng);

        self.bags = [bag_1, bag_2];
        self.active_bag = 0;
        self.current_index = 0;
    }

    pub fn set_next_blocks_slice(&mut self, blocks: &[Block]) {
        for (i, &block) in blocks.iter().take(Block::COUNT * 2).enumerate() {
            let total_idx = self.current_index as usize + i;
            let bag_idx = if total_idx < Block::COUNT {
                self.active_bag as usize
            } else {
                (1 - self.active_bag) as usize
            };
            let in_bag_idx = total_idx % Block::COUNT;

            let bag = &mut self.bags[bag_idx];
            if let Some(pos) = bag[in_bag_idx..].iter().position(|&b| b == block) {
                bag.swap(in_bag_idx, in_bag_idx + pos);
            } else {
                bag[in_bag_idx] = block;
            }
        }
    }

    pub fn set_next_blocks<const N: usize>(&mut self, blocks: [Block; N]) {
        const MAX_BLOCKS: usize = Block::COUNT * 2;
        const {
            assert!(
                N <= MAX_BLOCKS,
                "Cannot queue more blocks than two full bags"
            );
        }

        self.set_next_blocks_slice(&blocks);
    }

    pub fn get_next_block(&mut self) -> Block {
        let block = self.bags[self.active_bag as usize][self.current_index as usize];
        self.current_index += 1;

        if self.current_index as usize == Block::COUNT {
            self.bags[self.active_bag as usize].shuffle(&mut self.rng);
            self.active_bag = 1 - self.active_bag;
            self.current_index = 0;
        }

        block
    }

    pub fn get_next_blocks(&self) -> [Block; 5] {
        let curr = &self.bags[self.active_bag as usize];
        let next = &self.bags[(1 - self.active_bag) as usize];

        let mut result = [Block::Square; 5];
        for i in 0..5 {
            let idx = self.current_index as usize + i;
            if idx < Block::COUNT {
                result[i] = curr[idx];
            } else {
                result[i] = next[idx - Block::COUNT];
            }
        }
        result
    }
}
