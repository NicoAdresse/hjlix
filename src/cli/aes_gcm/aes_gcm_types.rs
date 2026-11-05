// src/cli/aes_gcm/aes_gcm_types.rs

use aes_gcm::{Aes128Gcm, Aes256Gcm, Key, aead::Generate};
use clap::ValueEnum;
use crate::cli::err::aes_err::AESError;
use crate::cli::traits::generate_aes_key::HjlixGenerateAESKey;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum AESGCMTypes {
    AES128GCM,
    AES256GCM,
}

impl AESGCMTypes {
    pub fn figure_out_type(&self, key: &[u8]) -> Result<Self, AESError> {
        match key.len() {
            16 => Ok(AESGCMTypes::AES128GCM),
            32 => Ok(AESGCMTypes::AES256GCM),
            _ => Err(AESError::InvalidKeyLength),
        }
    }
}

impl HjlixGenerateAESKey for AESGCMTypes {
    fn generate_key_for_aes_type(&self) -> Vec<u8> {
        match self {
            AESGCMTypes::AES128GCM => {
                let key = Key::<Aes128Gcm>::generate();
                key.to_vec()
            }
            AESGCMTypes::AES256GCM => {
                let key = Key::<Aes256Gcm>::generate();
                key.to_vec()
            }
        }
    }
}
