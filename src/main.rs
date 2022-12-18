extern crate core;

use chrono::Utc;
use sha2::{Sha256, Digest};

const DIFFICULTY_PREFIX: &str = "00";

type Hash = [u8; 32];

#[derive(Debug)]
struct Block {
    id: u64,
    previous_hash: String,
    hash: String,
    timestamp: i64,
    data: String,
    nonce: u64,
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let mut block = Self {
            id,
            previous_hash,
            hash: "".to_string(),
            timestamp: Utc::now().timestamp(),
            data,
            nonce: 0,
        };
        block.mine();
        block
    }
    fn mine(&mut self) {
        println!("Mining block #{}", self.id);
        loop {
            if self.nonce % 100000 == 0 {
                println!("Still working on block #{}, nonce={}.", self.id, self.nonce);
            }
            let hash = self.calculate_hash();
            let binary_hash = Block::hash_to_binary_representation(hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                let hash_str = hex::encode(&hash);
                self.hash = hash_str;
                println!("Block #{} mined. Nonce={}, Hash={:?}", self.id, self.nonce, self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
    fn calculate_hash(&self) -> Hash {
        let hash = Sha256::new()
            .chain_update(self.id.to_string())
            .chain_update(&self.previous_hash)
            .chain_update(self.timestamp.to_string())
            .chain_update(&self.data)
            .chain_update(self.nonce.to_string())
            .finalize();

        hash.into()
    }

    fn hash_to_binary_representation(hash: Hash) -> String {
        let mut res: String = String::default();

        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
        }
    }
    fn genesis(&mut self) {
        let genesis_block = Block::new(0, "".to_string(), "Genesis block".to_string());
        self.blocks.push(genesis_block);
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let last_block_id = prev_block.id;
        let prev_block_hash = prev_block.hash.clone();
        let block = Block::new(last_block_id + 1, prev_block_hash, data);
        self.blocks.push(block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];
            if block.previous_hash != previous_block.hash {
                println!("Block #{} has different hash!", block.id);
                return false;
            }
            if block.hash != hex::encode(block.calculate_hash()) {
                println!("Block #{} hash and calculated hashes are different! ", block.id);
                return false;
            }
        }
        println!("Chain is valid.");
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.genesis();
    blockchain.add_block("block-1 data".to_string());
    blockchain.add_block("block-2 data".to_string());
    blockchain.is_valid();
}



