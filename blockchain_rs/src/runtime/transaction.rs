use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
}
//impl Transaction { pub fn new(&self) -> Self {
//        Self {
//            self.sender,
//            self.receiver,
//            self.amount,
//            self.signature,
//        }
//    }
//  }
