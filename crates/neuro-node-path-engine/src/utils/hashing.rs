use sha2::{Sha256, Digest};
use hex;

pub struct Hasher;

impl Hasher {
    pub fn hash_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn hash_bytes(input: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    pub fn verify_hash(input: &str, expected_hash: &str) -> bool {
        Self::hash_string(input) == expected_hash
    }
}
