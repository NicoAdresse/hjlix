// src/cli/aes_gcm/aes_gcm_types.rs

use clap::ValueEnum;
use crate::cli::err::aes_err::AESError;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum AESGCMTypes {
    AES128GCM,
    AES256GCM,
}

impl AESGCMTypes {
    pub fn figure_out_type(key: &[u8]) -> Result<Self, AESError> {
        match key.len() {
            16 => Ok(AESGCMTypes::AES128GCM),
            32 => Ok(AESGCMTypes::AES256GCM),
            _ => Err(AESError::InvalidKeyLength),
        }
    }
}
