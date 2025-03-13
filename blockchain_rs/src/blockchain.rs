use crate::block::Block;
use chrono::Utc;

#[derive(Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let index = previous_block.index + 1;
        let timestamp = Utc::now().timestamp();
        let previous_hash = previous_block.hash.clone();

        let new_block = Block::mine_block(index, timestamp, &data, &previous_hash);

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let current_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce,
            );

            if current_hash != current.hash {
                return false;
            }
        }
        true
    }
}
