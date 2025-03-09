use serde::{Serialize,Deserialize};
use sha2::{Sha256,Digest};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block{

    fn calculate_hash(index:u64, timestamp:i64, data: &str, previous_hash: &str) -> String{
        let input = format!("{},{},{},{}" ,index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }
}

fn main(){
    println!(" this is the first line in our journey of 
               a BlockChain from scratch");
}
