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

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\t\tid={}", self.id)
    }
}

pub struct Transactions(pub Vec<Transaction>);

impl fmt::Display for Transactions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |result, transaction| result.and_then(|_| writeln!(f, "{}", transaction)))
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
        Self::process(&mut tx);
        tx
    }
    pub fn process(&mut self) {
        self.id = hex::encode(self.calculate_hash());
    }

    fn calculate_hash(&mut self) -> [u8; 32] {
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
        let hash: [u8; 32] = hash.into();
        hash
    }
}
