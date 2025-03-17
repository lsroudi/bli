pub mod block;
pub mod blockchain;
pub mod pallets;
pub mod transaction;

use block::Block;
use blockchain::Blockchain;
use pallets::{balances::Balances, system::System, timestamp::Timestamp};
use transaction::Transaction;

pub struct Runtime {
    pub system: System,
    pub timestamp: Timestamp,
    pub balances: Balances,
    pub blockchain: Blockchain,
}
