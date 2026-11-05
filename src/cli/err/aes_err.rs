// src/cli/err/aes_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum AESError {
    InvalidKeyLength,
    FailedToFigureOutType,
    FailedToCreateKey,
    FailedToCreateCiphertext,
    FailedToDecrypt,
}

impl std::error::Error for AESError {}
impl std::fmt::Display for AESError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}