extern crate core;

mod utility;

use crate::utility::hash_utils::find_merkle_root;
use chrono::Utc;
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::Formatter;
use std::time::Instant;
use x25519_dalek::{PublicKey, StaticSecret};

const DIFFICULTY_PREFIX: &str = "0";

type Hash = [u8; 32];

struct Block {
    id: u64,
    previous_hash: String,
    hash: String,
    timestamp: i64,
    merkle_root: String,
    transactions: Transactions,
    nonce: u64,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n\tid={}\n\thash={}\n\tprevious hash={}\n\tmerkle root={}\n\ttransactions=\n{}",
            self.id, self.hash, self.previous_hash, self.merkle_root, self.transactions
        )
    }
}

impl Block {
    fn new(id: u64, previous_hash: String) -> Self {
        Self {
            id,
            previous_hash,
            hash: "".to_string(),
            timestamp: Utc::now().timestamp(),
            merkle_root: "".to_string(),
            transactions: Transactions(Vec::new()),
            nonce: 0,
        }
    }
    fn mine(&mut self) {
        let s = Instant::now();
        println!("Started to mine block #{}", self.id);
        let mut tx_list: Vec<String> = self.transactions.0.iter().map(|tx| tx.id.clone()).collect();
        self.merkle_root = find_merkle_root(&mut tx_list);
        loop {
            if self.nonce > 0 && self.nonce % 1000000 == 0 {
                println!("Still working on block #{}, nonce={}.", self.id, self.nonce);
            }
            let hash = self.calculate_hash();
            let binary_hash = Block::hash_to_binary_representation(hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                let hash_str = hex::encode(&hash);
                self.hash = hash_str;
                break;
            }
            self.nonce += 1;
        }
        println!(
            "Block #{} mined in '{}' sec. Nonce={}, Hash={:?}",
            s.elapsed().as_secs(),
            self.id,
            self.nonce,
            self.hash
        );
    }
    fn calculate_hash(&self) -> Hash {
        let data = serde_json::json!({
            "id": self.id,
            "previous_hash" : self.previous_hash,
            "timestamp": self.timestamp,
            "nonce": self.nonce,
            "merkle_root": self.merkle_root
        });

        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        let hash = hasher.finalize();
        hash.into()
    }

    fn hash_to_binary_representation(hash: Hash) -> String {
        let mut res: String = String::default();

        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }

    fn add_transaction(&mut self, transaction: Option<Transaction>) -> bool {
        if let Some(tx) = transaction {
            self.transactions.0.push(tx);
            return true;
        };
        false
    }
}

struct Mempool(Vec<Transaction>);

impl fmt::Display for Mempool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, tx| result.and_then(|_| writeln!(f, "{}", tx)))
    }
}

struct Blocks(Vec<Block>);

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, block| result.and_then(|_| writeln!(f, "{}", block)))
    }
}

struct Blockchain {
    blocks: Blocks,
    mempool: Mempool,
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Blockchain:\nMempool\t:{}\nBlocks\t:{}", self.mempool, self.blocks)
    }
}

impl Blockchain {
    fn new() -> Self {
        Self {
            blocks: Blocks(Vec::new()),
            mempool: Mempool(Vec::new()),
        }
    }
    fn genesis(&mut self) {
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

    fn is_valid(&self) -> bool {
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

struct Wallet {
    name: String,
    private_key: StaticSecret,
    public_key: PublicKey,
}

impl Wallet {
    const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
    fn new(passphrase: &str, name: Option<&str>) -> Self {
        let name = match name {
            Some(name) => name.to_string(),
            _ => "".to_string(),
        };

        let (private_key, public_key) = Wallet::generate_keypair();

        Self { name, private_key, public_key }
    }

    fn generate_keypair() -> (StaticSecret, PublicKey) {
        let mut rand_generator = OsRng {};
        let mut nonce = [0u8; Wallet::XCHACHA20_POLY1305_NONCE_SIZE];
        rand_generator.fill_bytes(&mut nonce);

        let private_key = StaticSecret::new(rand_generator);
        let public_key = PublicKey::from(&private_key);

        (private_key, public_key)
    }

    fn send(&self, recipient: PublicKey, amount: f64) -> Option<Transaction> {
        if amount < 0.0 {
            return None;
        }

        let mut tx = Transaction::new(self.public_key, recipient, amount);
        tx.process();
        Some(tx)
    }
    fn sign(&self) {
        todo!("sign the tx")
    }
}

#[derive(Debug)]
pub struct Transaction {
    id: String,
    sender: PublicKey,
    recipient: PublicKey,
    amount: f64,
    timestamp: i64,
    sequence: u64,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\t\tid={}", self.id)
    }
}

struct Transactions(Vec<Transaction>);

impl fmt::Display for Transactions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |result, transaction| result.and_then(|_| writeln!(f, "{}", transaction)))
    }
}

impl Transaction {
    fn new(sender: PublicKey, recipient: PublicKey, amount: f64) -> Self {
        let timestamp = Utc::now().timestamp();
        let sequence = 0;
        let id = "".to_string();
        let mut tx = Self {
            id,
            sender,
            recipient,
            amount,
            timestamp,
            sequence,
        };
        Self::process(&mut tx);
        tx
    }
    fn process(&mut self) {
        self.id = hex::encode(self.calculate_hash());
    }

    fn calculate_hash(&mut self) -> Hash {
        let data = serde_json::json!({
            "sender": self.sender.as_bytes(),
            "recipient": self.recipient.as_bytes(),
            "amount": self.amount,
            "sequence":self.sequence,
        });
        self.sequence += 1;

        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        let hash = hasher.finalize();
        let hash: Hash = hash.into();
        hash
    }
}

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
