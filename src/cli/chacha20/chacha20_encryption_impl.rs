// src/cli/chacha20/chacha20_encryption_impl.rs

use crate::cli::chacha20::chacha20_types::ChaCha20Types;
use crate::cli::traits::generate_chacha20_key::GenerateChaCha20Key;
use crate::cli::err::chacha20_err::ChaCha20Error;
use chacha20poly1305::{Nonce, aead::Generate};

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20EncryptionImpl {
    texts: Vec<String>,
    chacha20_type: ChaCha20Types,
    no_verbose: bool,
    gen_key: bool,
}

impl ChaCha20EncryptionImpl {
    pub fn new(texts: Vec<String>, chacha20_type: ChaCha20Types, no_verbose: bool, gen_key: bool) -> Self {
        Self { texts, chacha20_type, no_verbose, gen_key }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let key = self.chacha20_type.generate_chacha20_key();

        for text in &self.texts {
            let nonce = Nonce::generate();
            let ciphertext = self.chacha20_type.encrypt(&key, &nonce, text.as_bytes())
                .map_err(|_| ChaCha20Error::FailedToEncrypt)?;
            let result = hex::encode(ciphertext);

            if !self.no_verbose {
                println!("Original: {}", text);
                println!("Result: {}", result);
            } else {
                println!("{}", result);
            }
        }

        Ok(())
    }
}