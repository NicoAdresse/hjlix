// src/cli/aes_gcm/aes_gcmfile_impl.rs

use std::path::PathBuf;
use crate::cli::{aes_gcm::aes_gcm_types::AESGCMTypes, err::io_err::IOError};

#[derive(Debug, Clone, PartialEq)]
pub struct AesGcmFileImpl {
    files: Vec<PathBuf>,
    aes_gcm_type: AESGCMTypes,
    no_verbose: bool,
}

impl AesGcmFileImpl {
    pub fn new(files: Vec<PathBuf>, aes_gcm_type: AESGCMTypes, no_verbose: bool) -> Self {
        Self { files, aes_gcm_type, no_verbose }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        if self.files.is_empty() {
            return Err(IOError::NoGivenFile.into());
        }

        Ok(())
    }
}