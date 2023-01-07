use oxchainlib::{App, Block, Transaction, Wallet};
fn main() {
    let mut app = App::default();
    app.start();

    let last_block = app.blockchain.blocks.0.last().unwrap();
    let mut block = Block::new(last_block.id + 1, last_block.hash.clone());

    let wallet1 = Wallet::new("", None);
    let wallet2 = Wallet::new("", None);

    let tx1 = Transaction::new(wallet1.public_key, wallet2.public_key, 2.1);
    let tx2 = Transaction::new(wallet2.public_key, wallet1.public_key, 0.1);
    let tx3 = Transaction::new(wallet2.public_key, wallet1.public_key, 0.2);

    block.add_transaction(Some(tx1));
    block.add_transaction(Some(tx2));
    block.add_transaction(Some(tx3));
    block.mine();
}
