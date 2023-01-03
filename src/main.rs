pub struct Block {
    pub index: u64,
}

impl Block {
    pub fn new(index: u64) -> Self {
        Block {
            index
        }
    }
}

fn main() {
    let block = Block::new(1);
    println!("Hello, world! {}", block.index);
}