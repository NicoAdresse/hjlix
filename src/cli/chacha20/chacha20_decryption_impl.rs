// src/cli/chacha20/chacha20_decryption_impl.rs

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20DecryptionImpl {
    ciphertexts: Vec<String>,
    chacha20_key_file: PathBuf,
    no_verbose: bool,
}

impl ChaCha20DecryptionImpl {
    pub fn new(ciphertexts: Vec<String>, chacha20_key_file: PathBuf, no_verbose: bool) -> Self {
        Self { ciphertexts, chacha20_key_file, no_verbose }
    }
}