use block::*;

mod block;
mod hashable;
mod libs;
mod types;

fn main() {
    let index = 0u32;
    let timestamp = libs::now();
    let previous_block_hash = vec![0; 32];
    let nonce = 0u64;

    let payload = String::from("Genesis Block");
    let block = Block::new(index, timestamp, previous_block_hash, nonce, payload);

    println!("Block is {:?}", block);

    let my_u256 = 254u32;
    let right = my_u256 >> 2;
    let left = my_u256 << 2;

    println!("is {:?} for {:b}", my_u256, my_u256);
    println!("is {:?} for {:b}", right, right);
    println!("is {:?} for {:b}", left, left);
}
