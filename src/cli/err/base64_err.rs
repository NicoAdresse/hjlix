// src/cli/err/base64_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Base64Error {
    FailedToDecodeUTF8,
    FailedToDecode,
}

impl std::error::Error for Base64Error {}
impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}