use super::*;
pub trait Hashable{
    fn bytes (&self) -> Vec<u8>; // FIX: Changed 'vec' to 'Vec' (capitalized)
    fn hash(&self) -> Hash{
        // Assuming 'digest' is the correct function name in 'crypto_hash'
        crypto_hash::digest(crypto_hash::Algorithm::SHA256,&self.bytes()) 
    }
}
