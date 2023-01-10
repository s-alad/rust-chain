use std::hash::{self, Hash};

use blockchainlibrary::*;

fn main() {
    let diff:u128 = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0, String::from("Genesis block"), diff);

    block.mine();
    println!("GENISIS MINED - {:?}", &block);

    let mut lasthash = block.hash.clone();

    let mut chain = Blockchain{
        blocks: vec![block]
    };

    println!("VERIFY GENISIS - {}", &chain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, now(), lasthash, 0, String::from(i.to_string()), diff);
        block.mine();
        println!("[{i}] - {:?}", &block);

        lasthash = block.hash.clone();
        chain.blocks.push(block);

        println!("VERIFY {i} - {}", &chain.verify());
    }    

}