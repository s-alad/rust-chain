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
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to: "Alice".to_string(),
                        value: 50,
                    },
                    transaction::Output {
                        to: "Alice".to_string(),
                        value: 10,
                    },
                ]
            }
        ], 
        diff);

    genesis.mine();
    println!("GENISIS MINED - {:?}", &genesis);

    let mut lasthash = genesis.hash.clone();

    let mut chain = Blockchain::new();

    chain.verify(genesis).expect("FAIL TO ADD GENESIS");


}