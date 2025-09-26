use std::fmt::{self, Debug, Formatter};
use super::*; // Assuming this brings in Hash, Transaction, check_difficulty, etc.

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>, // Corrected type name and pluralized for clarity
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}] : {} at : {} with : {} nonce : {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(), // Used corrected field name
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>, // Used Vec and correct field name
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::MAX) { // Use u64::MAX instead of u64::max_value()
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_diffulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}


impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> { // Used Vec
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce)); // Added missing semicolon
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(), // Used Vec
        );
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes
    }
}

// NOTE: check_diffulty is likely defined outside the impl Block block in your original lib.rs,
// but for correctness, I'm including it here as you originally placed it.
pub fn check_diffulty(hash: &Hash, difficulty: u128) -> bool {
    // Assuming difficulty_bytes_as_u128 is available from a parent module
    difficulty > difficulty_bytes_as_u128(&hash)
}
