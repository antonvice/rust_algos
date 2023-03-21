// Import necessary libraries
extern crate rand;
extern crate sha2;

use rand::Rng;
use sha2::{Sha256, Digest};

// Define the structure of a CryptoToken
#[derive(Debug)]
struct CryptoToken {
    id: u64,
    hash: String,
}

// This program generates a CryptoToken with a unique ID and a hash based on the ID.
use std::error::Error;

impl CryptoToken {
    // Function to create a new CryptoToken with a random ID and a hash based on the ID.
    fn new() -> Result<CryptoToken, Box<dyn Error>> {
        const MAX_ID: u64 = 1_000_000;
        let id = rand::thread_rng().gen_range(1..=MAX_ID);
        let mut hasher = Sha256::new();
        hasher.update(id.to_string().as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Ok(CryptoToken { id, hash })
    }
}
fn main() {
    let num_tokens = 5;
    for _ in 0..num_tokens {
        match CryptoToken::new() {
            Ok(token) => println!("Generated CryptoToken: {:?}", token),
            Err(e) => eprintln!("Error generating CryptoToken: {}", e),
        }
    }
}

// Add unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_crypto_token() {
        let token = CryptoToken::new().unwrap();
        assert!(token.id > 0 && token.id <= 1_000_000);
        assert!(!token.hash.is_empty());
    }
}
