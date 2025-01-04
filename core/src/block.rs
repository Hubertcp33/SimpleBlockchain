use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq ,Clone)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,  //transaction data merkle root hash
    pub pre_hash: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq ,Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,  //transaction data
}

impl Block {
    fn set_hash(&mut self) {
        self.header.time = Utc::now().timestamp(); //时间戳
        let header = coder::my_serialize(&(self.header));//序列化
        self.hash = coder::get_hash(&header[..]);

    }

    pub fn new_block(data:String, pre_hashn:String) -> Block {
        let transaction = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transaction[..]);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,
                pre_hash: pre_hashn,
            },
            hash: "".to_string(),
            data: data,
        };

        block.set_hash();
        block
    }
}