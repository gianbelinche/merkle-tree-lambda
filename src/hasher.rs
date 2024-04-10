use sha3::{Digest, Keccak256};
use hex;

pub fn hash(data: String) -> String {
    let mut hasher = Keccak256::new();

    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    hex::encode(result)
}
