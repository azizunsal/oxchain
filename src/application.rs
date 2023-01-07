use crate::blockchain::Blocks;
use crate::{Block, Blockchain, Transaction, Wallet};

pub struct MemPool(Vec<Transaction>);

pub struct App {
    pub blockchain: Blockchain,
    pub mem_pool: MemPool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            blockchain: Blockchain::default(),
            mem_pool: MemPool(Vec::new()),
        }
    }
}

impl App {
    pub fn start(&mut self) {
        create_genesis_block(&mut self.blockchain.blocks);
        println!("Application started");
    }
}

fn create_genesis_block(blocks: &mut Blocks) {
    let wallet_1 = Wallet::new("abc", Some("wallet-1"));
    let wallet_2 = Wallet::new("xyz", Some("wallet-2"));
    let tx = Transaction::new(wallet_1.public_key, wallet_2.public_key, 30.0);
    let mut genesis_block = Block::new(0, "".to_string());
    genesis_block.add_transaction(Some(tx));
    genesis_block.mine();
    blocks.0.push(genesis_block);
}
