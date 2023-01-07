use crate::transaction::{Transaction, Transactions};
use crate::utility::hash_utils::{find_merkle_root, hash_to_binary_representation};

use crate::hashable::Hashable;
use crate::Hash;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::Formatter;
use std::time::Instant;

const DIFFICULTY_PREFIX: &str = "0";

pub struct Block {
    pub id: u64,
    pub previous_hash: String,
    pub hash: String,
    pub timestamp: i64,
    pub merkle_root: String,
    pub transactions: Transactions,
    pub nonce: u64,
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

impl Hashable for Block {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.id.to_be_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.nonce.to_be_bytes());
        let tx_list: Vec<String> = self.transactions.0.iter().map(|tx| tx.id.clone()).collect();
        let merkle_root = find_merkle_root(tx_list);
        hasher.update(merkle_root.as_bytes());
        let hash = hasher.finalize();
        hash.into()
    }
}

impl Block {
    pub fn new(id: u64, previous_hash: String) -> Self {
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
    pub fn mine(&mut self) {
        let s = Instant::now();
        println!("Started to mine block #{}", self.id);
        let tx_list: Vec<String> = self.transactions.0.iter().map(|tx| tx.id.clone()).collect();
        self.merkle_root = find_merkle_root(tx_list);
        loop {
            if self.nonce > 0 && self.nonce % 1000000 == 0 {
                println!("Still working on block #{}, nonce={}.", self.id, self.nonce);
            }
            let hash: Hash = self.hash();
            let binary_hash = hash_to_binary_representation(hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                let hash_str = hex::encode(hash);
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

    pub fn add_transaction(&mut self, transaction: Option<Transaction>) -> bool {
        if let Some(tx) = transaction {
            self.transactions.0.push(tx);
            return true;
        };
        false
    }
}
