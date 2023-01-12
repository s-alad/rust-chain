use std::fmt::{self, Debug, Formatter};
use super::*;

pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    NonChronologicalTime,
    MismatchedPrevious,
    InvalidGenesis,
    InvalidInput,
    InsufficientInput,
    invalidBaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&mut self, block: Block) -> Result<(), BlockValidationError> {
        
        let i = self.blocks.len();

        if block.index != i as u32 {
            println!("[{i}] - FAIL INDEX MISMATCH {} != {}", &block.index, &i);
            return Err(BlockValidationError::MismatchedIndex);
        }
        else if (!block::difficulty(&block.hash(), block.difficulty)) {
            println!("[{i}] - FAIL DIFFICULTY");
            return Err(BlockValidationError::InvalidHash);
        }
        else if (i != 0) {

            let previousblock = &self.blocks[i - 1];

            if previousblock.time >= block.time  {
                println!("[{i}] - FAIL TIME");
                return Err(BlockValidationError::NonChronologicalTime)
            }

            if previousblock.hash != block.previous {
                println!("[{i}] - FAIL HASH");
                return Err(BlockValidationError::MismatchedPrevious);
            }
        }
        else {
            if block.previous != vec![0; 32] {
                println!("[{i}] - FAIL GENISIS");
                return Err(BlockValidationError::InvalidGenesis)
            }
        }
        
        Ok(())
    }
}