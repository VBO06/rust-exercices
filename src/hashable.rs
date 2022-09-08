// SOURCE : https://github.com/GeekLaunch/blockchain-rust/tree/master/src    

use super::*;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}