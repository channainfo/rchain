use crate::types::BlockHashType;

pub trait Hashable {
    fn bytes(&self) -> BlockHashType;

    fn hash(&self) -> BlockHashType {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
