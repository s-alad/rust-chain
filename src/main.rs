use std::hash::{self, Hash};

use blockchainlibrary::*;

fn main() {
    let diff:u128 = 0x000fffffffffffffffffffffffffffff;
    let mut genesis = Block::new(
        0, 
        now(),
        vec![0; 32],
        vec![
            Transaction {
                inputs: vec![
                    
                ],
                outputs: vec![
                    transaction::Output {
                        to: "Alice".to_string(),
                        value: 50,
                    },
                    transaction::Output {
                        to: "Bob".to_string(),
                        value: 7,
                    },
                ]
            }
        ], 
        diff);

    genesis.mine();
    println!("GENISIS MINED - {:?}", &genesis);

    let lasthash = genesis.hash.clone();

    let mut chain = Blockchain::new();

    chain.verify(genesis).expect("FAIL TO ADD GENESIS");


    let mut block = Block::new(
        1, 
        now(),
        lasthash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to: "Chris".to_string(),
                        value: 536,
                    },
                ]
            },
            Transaction {
                inputs: vec![
                    chain.blocks[0].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to: "Alice".to_string(),
                        value: 36,
                    },
                    transaction::Output {
                        to: "Bob".to_string(),
                        value: 4,
                    },
                ]
            }
        ], 
        diff);

    block.mine();
    println!("BLOCK MINED - {:?}", &block);

    let lasthash = block.hash.clone();

    chain.verify(block).expect("FAIL TO ADD BLOCK");


}