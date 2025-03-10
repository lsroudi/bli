use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn calculate_hash(index: u64, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let input = format!("{},{},{},{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    fn genesis() -> Self {
        let index = 0;
        let timestamp = Utc::now().timestamp();
        let data = String::from("the is the first block");
        let previous_hash = String::from("0");
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

#[derive(Debug)]
struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    fn new() -> Self {
        BlockChain {
            chain: vec![Block::genesis()],
        }
    }

    fn addBlock(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let index = previous_block.index + 1;
        let timestamp = Utc::now().timestamp();
        let previous_hash = previous_block.hash.clone();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        let new_block = Block {
            index: index,
            timestamp: timestamp,
            data: data,
            previous_hash: previous_hash,
            hash: hash,
        };
        self.chain.push(new_block);
    }
}

fn main() {
    println!("our BlockChain from scratch");

    let mut blockchain = BlockChain::new();
    blockchain.addBlock(String::from("this is the first block after genesis block"));
    println!("the genesis block is {:#?}", blockchain);
}
