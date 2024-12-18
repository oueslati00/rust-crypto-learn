use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::U256;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
pub struct Block {
    pub header: BlockHeader,
    pub transaction: Vec<Transaction>,
}
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,  // time when the block was created
    pub nonce: u64,                // we increment it to mine the block
    pub prev_block_hash: [u8; 32], //the hash of the previous block in the chain
    pub merkle_root: [u8; 32],     //  the hash of the merkle tree root
    pub target: U256,              // number, which hash to be higher than the hash if this block
}
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}
// called also Txin
pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32], // which we are linkin into this transaction as input
    pub signature: [u8; 64], // user can proves they can use the output of the previous transaction
}
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33],
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header: header,
            transaction: transactions,
        }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { outputs, inputs }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
