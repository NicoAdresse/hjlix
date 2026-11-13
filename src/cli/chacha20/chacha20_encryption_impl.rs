// src/cli/chacha20/chacha20_encryption_impl.rs

use crate::cli::chacha20::chacha20_types::{ChaCha20Types, ChaChaCipher};
use crate::cli::traits::generate_chacha20_key::GenerateChaCha20Key;
use crate::cli::err::chacha20_err::ChaCha20Error;

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
        let cipher = self.chacha20_type.create_cipher(&key)
            .map_err(|_| ChaCha20Error::FailedToCreateCipher)?;

        Ok(())
    }
}