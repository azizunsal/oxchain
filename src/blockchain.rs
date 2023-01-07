use crate::block::Block;
use crate::transaction::Transaction;

use crate::hashable::Hashable;
use crate::wallet::Wallet;
use std::fmt;
use std::fmt::Formatter;

pub struct Blocks(pub Vec<Block>);

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, block| result.and_then(|_| writeln!(f, "{}", block)))
    }
}

pub struct Blockchain {
    pub blocks: Blocks,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self { blocks: Blocks(Vec::new()) }
    }
}

impl Blockchain {
    pub fn genesis(&mut self) {
        let genesis_wallet_1 = Wallet::new("ABC", Some("wallet-1"));
        let genesis_wallet_2 = Wallet::new("XYZ", Some("wallet-2"));
        let genesis_tx = genesis_wallet_1.send(genesis_wallet_2.public_key, 30.0);

        let mut genesis_block = Block::new(0, "".to_string());
        genesis_block.add_transaction(genesis_tx);
        self.blocks.0.push(genesis_block);
    }

    fn add_block(&mut self, block: Block) -> bool {
        let latest_block = self.blocks.0.last().unwrap();
        if latest_block.hash != block.previous_hash {
            println!("Block #{} previous hash and latest block hash doesn't match!", block.id);
            return false;
        }
        if block.hash != hex::encode(block.hash()) {
            println!("Block #{} hash and the calculated hash doesn't match!", block.id);
            return false;
        }
        self.blocks.0.push(block);
        true
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.0.len() {
            let block = &self.blocks.0[i];
            let previous_block = &self.blocks.0[i - 1];
            println!("there are {} transactions in block #{}", block.transactions.0.len(), block.id);
            println!("there are {} transactions in block #{}", previous_block.transactions.0.len(), previous_block.id);

            if block.previous_hash != previous_block.hash {
                println!("Block #{} has different hash!", block.id);
                return false;
            }

            let calculated_hash = hex::encode(block.hash());

            println!("block #{} hash={} vs calculated_hash={}", block.id, block.hash, calculated_hash);
            if block.hash != calculated_hash {
                println!("Block #{} hash and calculated hashes are different! ", block.id);
                return false;
            }
        }
        println!("Chain is valid.");
        true
    }
}
