use blockchainlibrary::*;

fn main() {
    let block = Block::new(0, 0, String::from("0"), 0, String::from("Genesis"));

    println!("{:?}", block);
}