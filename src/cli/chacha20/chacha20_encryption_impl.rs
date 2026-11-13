// src/cli/chacha20/chacha20_encryption_impl.rs

use crate::cli::chacha20::chacha20_types::ChaCha20Types;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20EncryptionImpl {
    texts: Vec<String>,
    chacha20_type: ChaCha20Types,
    no_verbose: bool,
}

impl ChaCha20EncryptionImpl {
    pub fn new(texts: Vec<String>, chacha20_type: ChaCha20Types, no_verbose: bool) -> Self {
        Self { texts, chacha20_type, no_verbose }
    }
}