// src/cli/aes_gcm/aes_gcm_types.rs

use clap::ValueEnum;
use crate::cli::err::aes_err::AESError;
use aes_gcm::{Aes128Gcm, Aes256Gcm, KeyInit};

#[derive(Debug, Clone, ValueEnum)]
pub enum AESGCMTypes {
    #[value(skip)]
    AES128GCM(Aes128Gcm),
    #[value(skip)]
    AES256GCM(Aes256Gcm),
}

impl AESGCMTypes {
    pub fn figure_out_type(&self, key: &[u8]) -> Result<Self, AESError> {
        match key.len() {
            16 => Aes128Gcm::new_from_slice(key)
                .map(AESGCMTypes::AES128GCM)
                .map_err(|_| AESError::InvalidKeyLength),
            32 => Aes256Gcm::new_from_slice(key)
                .map(AESGCMTypes::AES256GCM)
                .map_err(|_| AESError::InvalidKeyLength),
            _ => Err(AESError::InvalidKeyLength),
        }
    }
}
