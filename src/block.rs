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
    ) -> Self {
        let block_hash = vec![0u8; 32];
        let mut block = Self {
            index,
            timestamp,
            block_hash,
            previous_block_hash,
            nonce,
            payload,
        };

        block.block_hash = block.hash();
        block
    }
}

pub fn check_difficulty(hash: &BlockHashType, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_to_u8(hash)
}
