use crate::block::Block;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn print(&self) -> () {
        println!("Blockchain length: {}", self.blocks.len());

        for index in 0..self.blocks.len() {
            let block = &self.blocks[index];
            println!("{:?}", block);
        }
    }

    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            return true;
        }
        false
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if !block.valid() {
            return false;
        }

        if block.index > self.blocks.len() as u32 {
            return false;
        }

        if block.previous_block_hash != self.blocks[self.blocks.len() - 1].previous_block_hash {
            return false;
        }

        self.blocks.push(block);
        true
    }
}
