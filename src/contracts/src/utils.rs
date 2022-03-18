




use borsh::{BorshDeserialize, BorshSerialize};





#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contract<T>{
    pub sign: T,
}

impl<T> Contract<T>{
    pub fn new(&self) -> Option<&T>{ //-- we need to define the first argument as &self which is an immutable pointer to all Contract struct fields to satisfy the return type of new() method which is Option<&T> 
        Some(&self.sign) //-- returning a pointer to the sign field inside Option by using self which is &self in first argument of the new() method
    }
}