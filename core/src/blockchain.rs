use crate::block;

pub struct BlockChian {
    pub blocks: Vec<block::Block>,
}

impl BlockChian {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    pub fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is the genesis block".to_string(), String::from(""))
    }

    pub fn new_blockchain() -> BlockChian {
        BlockChian {
            blocks: vec![BlockChian::new_genesis_block()],
        }
    }
}