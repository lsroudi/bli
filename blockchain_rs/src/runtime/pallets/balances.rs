use std::collections::HashMap;

#[derive(Debug)]
pub trait Config {
    type AccountId: Eq + std::hash::Hash + Clone;
    type Balance: Copy + Default + std::ops::SubAssign + std::ops::AddAssign;
}

#[derive(Debug)]
pub struct Balances<T:Config> {
    balances: HashMap<T::AccountId,T::Balance>,
        
}

impl<T:Config> Balances<T>{
    pub fn new()->Self{
        Balances{
            balances: HashMap::new(),
        }
    }

    pub fn transfer(&mut self, sender:T::AccountId, receiver:T::AccountId, amount:T::Balance)-> bool{
        if self.get(&sender).unwrap_or(&T::Balance::default()) >= &amount {
            *self.balances.entry(&sender).or_insert(T::Balance::default()) -= amount;
            *self.balances.entry(&receiver).or_insert(T::Balance::default()) += amount;
            true
        }else{
            false
        }
    }
}
