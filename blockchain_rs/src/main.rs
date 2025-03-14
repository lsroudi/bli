mod block;
mod blockchain;
mod transaction;

use blockchain::BlockChain;

fn main() {
    println!("our BlockChain from scratch");

    let mut blockchain = BlockChain::new();
    blockchain.add_block(String::from("this is the first block after genesis block"));
    println!("the genesis block is {:#?}", blockchain);
    println!("our block chain is valid : {}", blockchain.is_valid())
}
