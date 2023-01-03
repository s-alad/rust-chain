use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub time: u128,
    pub hash: Vec<u8>,
    pub previous: String,
    pub nonce: u64,
    pub data: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block {{ index: {}, time: {}, hash: {}, previous: {}, nonce: {}, data: {} }}",
            self.index, self.time, &hex::encode(&self.hash), self.previous, self.nonce, self.data
        )
    }
}

impl Block {
    pub fn new(index: u32, time: u128, previous: String, nonce: u64, data: String) -> Self {
        return Block {
            index,
            time,
            hash: vec![0; 32],
            previous,
            nonce,
            data,
        }
    }
}