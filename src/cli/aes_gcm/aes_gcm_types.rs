// src/cli/aes_gcm/aes_gcm_types.rs

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum AESGCMTypes {
    AES128GCM,
    AES256GCM,
}

impl AESGCMTypes {
    pub fn figure_out_type(key: &[u8]) -> Result<Self, String> {
        match key.len() {
            16 => Ok(AESGCMTypes::AES128GCM),
            32 => Ok(AESGCMTypes::AES256GCM),
            _ => Err(format!(
                "Invalid key length: {} bytes. AES-GCM requires 16 bytes (128-bit) or 32 bytes (256-bit).",
                key.len()
            )),
        }
    }
}
