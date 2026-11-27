// src/cli/chacha20/chacha20_file_impl.rs

use std::path::PathBuf;
use crate::cli::chacha20::chacha20_types::ChaCha20Types;

#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20EncryptionFileImpl {
    files: Vec<PathBuf>,
    chacha20_type: ChaCha20Types,
    no_verbose: bool,
}

impl ChaCha20EncryptionFileImpl {
    pub fn new(files: Vec<PathBuf>, chacha20_type: ChaCha20Types, no_verbose: bool) -> Self {
        Self { files, chacha20_type, no_verbose }
    }
}