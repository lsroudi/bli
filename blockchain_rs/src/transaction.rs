use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {}
impl Transaction {
    pub fn new() -> Self {
        Transaction {}
    }
}
