// src/cli/err/chacha20_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ChaCha20Error {
    FailedToCreateCipher,
}

impl std::error::Error for ChaCha20Error {}
impl std::fmt::Display for ChaCha20Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}