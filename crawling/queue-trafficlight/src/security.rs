use sha2::{Sha256, Digest};
use rand::Rng;
use hex::encode;

pub fn hash(password: &str, salt: Option<&str>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", password, salt.unwrap_or(""))); // Combine password + salt
    encode(hasher.finalize()) // Convert hash to hex string
}