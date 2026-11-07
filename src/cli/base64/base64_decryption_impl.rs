// src/cli/base64/base64_decryption_impl.rs

use crate::cli::base64::base64_types::Base64Types;
use crate::cli::err::base64_err::Base64Error;

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
        for text in &self.encoded_texts {
            let decoded_bytes = self.base64_type.decode(text)?;
            let result = String::from_utf8(decoded_bytes)
                .map_err(|_| Base64Error::FailedToDecodeUTF8)?;

            if !self.no_verbose {
                println!("Original: {}", text);
                println!("Result: {}", result);
            } else {
                println!("{}", result);
            }
        }

        Ok(())
    }
}