// src/cli/err/aes_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum AESError {
    InvalidKeyLength,
}