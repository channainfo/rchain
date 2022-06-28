use crate::libs::{difficulty_bytes_to_u8, u32_to_bytes, u64_to_bytes};
use crate::types::*;
use crate::{hashable::*, libs::u128_to_bytes};
use std::fmt::{self, Debug};

pub struct Block {
    pub index: u32,
    pub timestamp: TimestampType,
    pub block_hash: BlockHashType,
    pub previous_block_hash: BlockHashType,
    pub nonce: NonceType,
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            r#" 
            Block:
                index: {},
                timestamp: {},
                block_hash: {},
                previous_block_hash: {},
                nonce: {},
                payload: {},
            "#,
            &self.index,
            &self.timestamp,
            &hex::encode(&self.block_hash),
            &hex::encode(&self.previous_block_hash),
            &self.nonce,
            &self.payload
        )
    }
}

impl Hashable for Block {
    fn bytes(&self) -> BlockHashType {
        let mut bytes = vec![];
        bytes.extend(u32_to_bytes(&self.index));
        bytes.extend(u128_to_bytes(&self.timestamp));
        bytes.extend(u64_to_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(u128_to_bytes(&self.difficulty));
        bytes
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: TimestampType,
        previous_block_hash: BlockHashType,
        nonce: NonceType,
        payload: String,
        difficulty: u128,
    ) -> Self {
        let block_hash = vec![0u8; 32];
        let mut block = Self {
            index,
            timestamp,
            block_hash,
            previous_block_hash,
            nonce,
            payload,
            difficulty,
        };

        block.block_hash = block.hash();
        block
    }

    pub fn mine(&mut self) -> () {
        for nonce_attempt in 0..u64::max_value() {
            self.nonce = nonce_attempt;
            let hash = self.hash();

            if check_difficulty(&hash, self.difficulty) {
                self.block_hash = hash;

                println!("found block with nonce:{}", &self.nonce);
                println!("Block is {:?}", self);

                return ();
            } else {
                print!("running for {}\r", nonce_attempt);
            }
        }
    }
}

pub fn check_difficulty(hash: &BlockHashType, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_to_u8(hash)
}
