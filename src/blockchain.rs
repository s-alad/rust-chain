use std::{fmt::{self, Debug, Formatter}, collections::HashSet};
use super::*;

#[derive(Debug)]

pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    NonChronologicalTime,
    MismatchedPrevious,
    InvalidGenesis,
    InvalidInput,
    InsufficientInput,
    InvalidBaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent: HashSet<Hash>,
}

impl Blockchain {

    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent: HashSet::new()
        }
    }

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
        else if i != 0 {

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

        if let Some((base, transactions)) = block.transactions.split_first() {

            if !base.is_base() {
                return Err(BlockValidationError::InvalidBaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total = 0;

            for txn in transactions {
                let input_hashes = txn.input_hashes();

                if !(&input_hashes - &self.unspent).is_empty() || !(&input_hashes & &block_spent).is_empty() {
                    return Err(BlockValidationError::InvalidInput);
                }

                let input_value = txn.input_value();
                let output_value = txn.output_value();

                if output_value > input_value {
                    return Err(BlockValidationError::InsufficientInput);
                }

                let fee = input_value - output_value;
                total += fee;

                block_spent.extend(input_hashes);
                block_created.extend(txn.output_hashes());
            }   
        
            if base.output_value() < total {
                return Err(BlockValidationError::InvalidBaseTransaction)
            }
            else {
                block_created.extend(base.output_hashes())
            }

            self.unspent.retain(|output| !block_spent.contains(output));
            self.unspent.extend(block_created)
        }
        
        self.blocks.push(block);
        Ok(())
    }
}