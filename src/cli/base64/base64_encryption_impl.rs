// src/cli/base64/base64_encryption_impl.rs

use crate::cli::base64::base64_types::Base64Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Base64EncryptionImpl {
    texts: Vec<String>,
    base64_type: Base64Types,
    no_verbose: bool,
}

impl Base64EncryptionImpl {
    pub fn new(texts: Vec<String>, base64_type: Base64Types, no_verbose: bool) -> Self {
        Self { texts, base64_type, no_verbose }
    }

    pub fn execute(&self) {
        for text in &self.texts {
            let result = self.base64_type.encode(text);

            if !self.no_verbose {
                println!("Original: {}", text);
                println!("Result: {}", result);
            } else {
                println!("{}", result);
            }
        }
    }
}