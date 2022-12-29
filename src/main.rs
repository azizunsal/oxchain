mod block;
mod blockchain;
mod transaction;
mod utility;
mod wallet;

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::wallet::Wallet;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.genesis();
    blockchain.is_valid();

    let last_block = blockchain.blocks.0.last().unwrap();
    let mut block = Block::new(last_block.id + 1, last_block.hash.clone());

    let wallet1 = Wallet::new("", None);
    let wallet2 = Wallet::new("", None);
    let tx1 = wallet1.send(wallet2.public_key, 2.1);
    let tx2 = wallet2.send(wallet1.public_key, 0.1);
    block.add_transaction(tx1);
    block.add_transaction(tx2);
    block.mine();
    blockchain.blocks.0.push(block);

    println!("{}", blockchain);
}
