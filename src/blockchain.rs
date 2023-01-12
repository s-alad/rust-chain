use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&mut self, block: Block) -> Result<(), > {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("[{i}] - FAIL INDEX MISMATCH {} != {}", &block.index, &i);
                return false;
            }
            else if (!block::difficulty(&block.hash(), block.difficulty)) {
                println!("[{i}] - FAIL DIFFICULTY");
                return false;
            }
            else if (i != 0) {

                let previousblock = &self.blocks[i - 1];

                if previousblock.time >= block.time  {
                    println!("[{i}] - FAIL TIME");
                    return false
                }

                if previousblock.hash != block.previous {
                    println!("[{i}] - FAIL HASH");
                    return false
                }
            }
            else {
                if block.previous != vec![0; 32] {
                    println!("[{i}] - FAIL GENISIS");
                    return false
                }
            }
        }
        return true;
    }
}