use crate::transaction::Transaction;

use rand::{rngs::OsRng, RngCore};
use x25519_dalek::{PublicKey, StaticSecret};

pub struct Wallet {
    pub name: String,
    pub private_key: StaticSecret,
    pub public_key: PublicKey,
}

impl Wallet {
    const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
    pub fn new(passphrase: &str, name: Option<&str>) -> Self {
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

    pub fn send(&self, recipient: PublicKey, amount: f64) -> Option<Transaction> {
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
