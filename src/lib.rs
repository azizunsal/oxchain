type Hash = [u8; 32];

mod application;
mod block;
mod blockchain;
mod hashable;
mod transaction;
mod utility;
mod wallet;

pub use crate::block::Block;
pub use crate::blockchain::Blockchain;
pub use crate::transaction::Transaction;
pub use crate::wallet::Wallet;
pub use crate::application::App;
