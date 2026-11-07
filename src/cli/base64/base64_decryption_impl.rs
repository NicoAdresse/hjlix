// src/cli/base64/base64_decryption_impl.rs

use crate::cli::base64::base64_types::Base64Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Base64DecryptionImpl {
    encoded_texts: Vec<String>,
    base64_type: Base64Types,
    no_verbose: bool,
}

impl Base64DecryptionImpl {
    pub fn new(encoded_texts: Vec<String>, base64_type: Base64Types, no_verbose: bool) -> Self {
        Self { encoded_texts, base64_type, no_verbose }
    }

    pub fn execute(&self) -> Result<(), Base64Error> {
        Ok(())
    }
}