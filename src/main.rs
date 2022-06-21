use blocks::*;

mod blocks;
mod libs;
mod types;

fn main() {
    let index = 0u32;
    let timestamp = libs::now();
    let block_hash = vec![0; 32];
    let previous_block_hash = vec![0; 32];
    let nonce = 0u64;

    let payload = String::from("Genesis Block");
    let block = Block::new(
        index,
        timestamp,
        block_hash,
        previous_block_hash,
        nonce,
        payload,
    );

    println!("Block is {:?}", block);
}
