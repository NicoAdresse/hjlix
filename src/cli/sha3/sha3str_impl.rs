// src/cli/sha3/sha3str_impl.rs

use crate::cli::sha3::sha3_types::Sha3Types;
use crate::cli::err::io_err::IOError;
use crate::cli::traits::digest::HjlixDigest;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha3StrImpl {
    strings: Vec<String>,
    sha3_type: Sha3Types,
    no_verbose: bool,
}

impl Sha3StrImpl {
    pub fn new(
        strings: Vec<String>,
        sha3_type: Sha3Types,
        no_verbose: bool,
    ) -> Self {
        Self {
            strings,
            sha3_type,
            no_verbose
        }
    }

    pub fn execute(&self) -> Result<(), IOError> {
        for str in &self.strings {
            let bytes = str.as_bytes();
            let digested_bytes = self.sha3_type.digest(bytes);
            let result = hex::encode(digested_bytes);

            if !self.no_verbose {
                println!("Original: {}", str);
                println!("Result: {}", result);
            } else {
                println!("{}", result);
            }
        }

        Ok(())
    }
}