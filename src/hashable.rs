use super::*;
pub trait Hashable{
    fn bytes (&self) -> vec<u8>;
    fn hash(&self) -> Hash{
        crypto_hash::digit(crypto_hash::Algorithm::SHA256,&self.bytes())
    }
}