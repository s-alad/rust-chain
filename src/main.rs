use std::hash;

use blockchainlibrary::*;

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, String::from("Genesis block"));

    println!("{:?}", &block);

    let hash = block.hash();

    println!("{:?}", &hash);

    block.hash = hash;

    println!("{:?}", &block);
}