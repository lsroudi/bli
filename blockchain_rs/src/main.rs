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

fn main(){
    println!(" this is the first line in our journey of 
               a BlockChain from scratch");
}
