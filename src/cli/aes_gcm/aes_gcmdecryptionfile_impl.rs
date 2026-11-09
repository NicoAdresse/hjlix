// src/cli/aes_gcm/aes_gcmdecryptionfile_impl.rs

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct AesGcmDecryptionFileImpl {
    files: Vec<PathBuf>,
    aes_key: PathBuf,
    no_verbose: bool,
}

impl AesGcmDecryptionFileImpl {
    pub fn new(files: Vec<PathBuf>, aes_key: PathBuf, no_verbose: bool) -> Self {
        Self { files, aes_key, no_verbose }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        Ok(())
    }
}