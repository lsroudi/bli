use crate::transaction::Transaction;
use chrono::Utc;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const HASH_DIFFICULTY: usize = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn calculate_hash(
        index: u64,
        timestamp: i64,
        transactions: &Vec<Transaction>,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let tx_data: String = transactions
            .iter()
            .map(|tx| format!("{},{},{}", tx.sender, tx.receiver, tx.amount))
            .collect();
        let input = format!(
            "{},{},{},{},{}",
            index, timestamp, tx_data, previous_hash, nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    pub fn genesis() -> Self {
        let index = 0;
        let timestamp = Utc::now().timestamp();
        let transactions = vec![];
        let previous_hash = String::from("0");
        let nonce = 0;
        let hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash, nonce);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn mine_block(
        index: u64,
        timestamp: i64,
        transactions: &Vec<Transaction>,
        previous_hash: &str,
    ) -> Self {
        let mut nonce = 0;
        loop {
            let hash = Block::calculate_hash(index, timestamp, transactions, previous_hash, nonce);

            if hash.starts_with(&"0".repeat(HASH_DIFFICULTY)) {
                return Block {
                    index,
                    timestamp,
                    transactions: transactions.to_vec(),
                    previous_hash: previous_hash.to_string(),
                    hash,
                    nonce,
                };
            }
            nonce += 1;
        }
    }
}
