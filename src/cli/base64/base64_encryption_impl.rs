// src/cli/base64/base64_encryption_impl.rs

use crate::cli::base64::base64_types::Base64Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Base64EncryptionImpl {
    texts: Vec<String>,
    base64_type: Base64Types,
}