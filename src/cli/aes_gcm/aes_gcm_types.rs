// src/cli/aes_gcm/aes_gcm_types.rs

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum AESGCMTypes {
    AES128GCM,
    AES256GCM,
}