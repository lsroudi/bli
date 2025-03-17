mod runtime;

use blockchain::BlockChain;
use transaction::Transaction;
fn main() {
    println!("our BlockChain from scratch");

    let mut blockchain = BlockChain::new();
    let tx1 = Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 150,
        signature: "signature_1".to_string(),
    };

    let tx2 = Transaction {
        sender: "Alice".to_string(),
        receiver: "Charlie".to_string(),
        amount: 10,
        signature: "signature_2".to_string(),
    };

    let tx3 = Transaction {
        sender: "Bob".to_string(),
        receiver: "Charlie".to_string(),
        amount: 50,
        signature: "signature_3".to_string(),
    };
    blockchain.add_block(vec![tx1, tx2, tx3]);
    println!("the genesis block is {:#?}", blockchain.chain[0]);
    println!("the full blockchain is {:#?}", blockchain);
    println!("our block chain is valid : {}", blockchain.is_valid())
}
