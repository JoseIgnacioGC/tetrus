use crate::blocks::Block;

use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use strum::{EnumCount, VariantArray};

pub struct BlocksManager {
    bags: [[Block; Block::COUNT]; 2],
    active_bag: u8,
    current_index: u8,
    rng: SmallRng,
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
        }
    }

    #[cfg(any(debug_assertions, vhs, feature = "vhs"))]
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
        }
    }

    pub fn reset(&mut self) {
        let mut bag_1: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();
        let mut bag_2: [Block; Block::COUNT] = Block::VARIANTS.try_into().unwrap();

        bag_1.shuffle(&mut self.rng);
        bag_2.shuffle(&mut self.rng);

        self.bags = [bag_1, bag_2];
        self.active_bag = 0;
        self.current_index = 0;
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
