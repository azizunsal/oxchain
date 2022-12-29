use crate::block::Block;
use crate::transaction::Transaction;

use crate::wallet::Wallet;
use std::fmt;
use std::fmt::Formatter;

pub struct Blocks(pub Vec<Block>);
pub struct Mempool(pub Vec<Transaction>);

impl fmt::Display for Mempool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, tx| result.and_then(|_| writeln!(f, "{}", tx)))
    }
}

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, block| result.and_then(|_| writeln!(f, "{}", block)))
    }
}

pub struct Blockchain {
    pub blocks: Blocks,
    pub mempool: Mempool,
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Blockchain:\nMempool\t:{}\nBlocks\t:{}", self.mempool, self.blocks)
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Blocks(Vec::new()),
            mempool: Mempool(Vec::new()),
        }
    }
    pub fn genesis(&mut self) {
        let genesis_wallet_1 = Wallet::new("ABC", Some("wallet-1"));
        let genesis_wallet_2 = Wallet::new("XYZ", Some("wallet-2"));
        let genesis_tx = genesis_wallet_1.send(genesis_wallet_2.public_key, 30.0);

        let mut genesis_block = Block::new(0, "".to_string());
        genesis_block.add_transaction(genesis_tx);
        genesis_block.mine();
        self.blocks.0.push(genesis_block);
    }

    fn add_block(&mut self, block: Block) -> bool {
        let latest_block = self.blocks.0.last().unwrap();
        if latest_block.hash != block.previous_hash {
            println!("Block #{} previous hash and latest block hash doesn't match!", block.id);
            return false;
        }
        if block.hash != hex::encode(block.calculate_hash()) {
            println!("Block #{} hash and the calculated hash doesn't match!", block.id);
            return false;
        }
        self.blocks.0.push(block);
        true
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.0.len() {
            let block = &self.blocks.0[i];
            println!("there are {} transactions in block #{}", block.transactions.0.len(), block.id);
            for tx in &block.transactions.0 {
                println!("tx={}", tx);
            }
            let previous_block = &self.blocks.0[i - 1];
            println!(
                "there are {} transactions.0 in block #{}",
                previous_block.transactions.0.len(),
                previous_block.id
            );
            for tx in &previous_block.transactions.0 {
                println!("tx={}", tx);
            }
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
