// src/cli/aes_gcm/aes_gcm_impl.rs

use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;

#[derive(Debug, Clone, PartialEq)]
pub struct AESGCMImpl {
    strings: Vec<String>,
    aes_gcm_type: AESGCMTypes,
}