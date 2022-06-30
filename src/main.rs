use block::*;

use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod constant;
mod hashable;
mod libs;
mod types;

fn main() {
    let index = 0u32;
    let timestamp = libs::now();
    let previous_block_hash = vec![0; 32];
    let nonce = 0u64;
    let difficulty = block::DIFFICULTY;

    let payload = String::from("Genesis Block");
    let mut block = Block::new(
        index,
        timestamp,
        previous_block_hash,
        nonce,
        payload,
        difficulty,
    );

    println!("Block is {:?}", block);

    let my_u256 = 254u32;
    let right = my_u256 >> 2;
    let left = my_u256 << 2;

    println!("is {:?} for {:b}", my_u256, my_u256);
    println!("is {:?} for {:b}", right, right);
    println!("is {:?} for {:b}", left, left);

    block.mine();

    let blocks = vec![block];

    let mut blockchain = Blockchain { blocks: blocks };

    for index in 1..=10 {
        let timestamp = libs::now();

        let payload = format!("Block: {}", index);

        let last_block_hash = blockchain.blocks[index - 1].block_hash.clone();

        let mut next_block = Block::new(
            index as u32,
            timestamp,
            last_block_hash,
            nonce,
            payload,
            difficulty,
        );

        next_block.mine();

        blockchain.blocks.push(next_block);

        blockchain.print();
    }

    // println!("blockchain is {:?}", blockchain);
}
