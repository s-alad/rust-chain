use std::hash;

use blockchainlibrary::*;

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, String::from("Genesis block"), 0x000fffffffffffffffffffffffffffff);

    println!("{:?}", &block);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);


}