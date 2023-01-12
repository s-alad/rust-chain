use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Block {
    pub index: u32,
    pub time: u128,
    pub hash: Hash,
    pub previous: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block {{ index: {}, time: {}, hash: {}, previous: {}, nonce: {}, data: {}, difficulty: {} }}",
            self.index, self.time, &hex::encode(&self.hash), &hex::encode(&self.previous), self.nonce, self.transactions.len(), self.difficulty
        )
    }
}

impl Block {
    pub fn new(index: u32, time: u128, previous: Hash, transactions: Vec<Transaction>, difficulty: u128) -> Self {
        return Block {
            index,
            time,
            hash: vec![0; 32],
            previous,
            nonce: 0,
            transactions,
            difficulty
        }
    }

    pub fn mine(&mut self) {
        for nonce in 0..u64::max_value() {
            self.nonce = nonce;

            let hash = self.hash();

            if difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.time));
        bytes.extend(&self.previous);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions.iter().flat_map(|txn| txn.bytes()).collect::<Vec<u8>>());
        bytes.extend(&u128_bytes(&self.difficulty));

        return bytes;
    }
}

pub fn difficulty(hash: &Hash, difficulty: u128) -> bool {
    return difficulty > difficulty_bytes_as_u128(hash)
}
