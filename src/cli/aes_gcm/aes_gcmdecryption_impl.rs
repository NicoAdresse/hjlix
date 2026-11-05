// src/cli/aes_gcm/aes_gcmdecryption_impl.rs

#[derive(Debug, Clone, PartialEq)]
pub struct AesGcmDecryptionImpl {
    ciphertext: Vec<String>,
}

impl AesGcmDecryptionImpl {
    pub fn new(ciphertext: Vec<String>) -> Self {
        Self { ciphertext }
    }
}