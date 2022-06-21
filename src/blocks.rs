use crate::types::*;
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

impl Block {
    pub fn new(
        index: u32,
        timestamp: TimestampType,
        block_hash: BlockHashType,
        previous_block_hash: BlockHashType,
        nonce: NonceType,
        payload: String,
    ) -> Self {
        Self {
            index,
            timestamp,
            block_hash,
            previous_block_hash,
            nonce,
            payload,
        }
    }
}
