use crate::hashable::Hashable;
use crate::Hash;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::Formatter;
use x25519_dalek::PublicKey;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub sender: PublicKey,
    pub recipient: PublicKey,
    pub amount: f64,
    pub timestamp: i64,
    pub sequence: u64,
}

pub struct Transactions(pub Vec<Transaction>);

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\t\tid={}", self.id)
    }
}

impl fmt::Display for Transactions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |result, transaction| result.and_then(|_| writeln!(f, "{}", transaction)))
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(self.recipient.as_bytes());
        hasher.update(self.amount.to_be_bytes());
        hasher.update(self.sequence.to_be_bytes());
        let hash = hasher.finalize();
        let hash: Hash = hash.into();
        hash
    }
}

impl Transaction {
    pub fn new(sender: PublicKey, recipient: PublicKey, amount: f64) -> Self {
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
        process(&mut tx);
        tx
    }
}

pub fn verify(transaction: &Transaction) -> bool {
    false
}

pub fn process(transaction: &mut Transaction) {
    let hash = transaction.hash();
    transaction.sequence += 1;
    transaction.id = hex::encode(hash);
}
