// src/cli/sha/sha2str_impl.rs

use crate::cli::err::io_err::IOError;
use crate::cli::sha2::sha2_types::Sha2Types;
use crate::cli::traits::digest::HjlixDigest;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha2StrImpl {
    strings: Vec<String>,
    sha_type: Sha2Types,
    no_verbose: bool,
}

impl Sha2StrImpl {
    pub fn new(
        strings: Vec<String>,
        sha_type: Sha2Types,
        no_verbose: bool
    ) -> Self {
        Self {
            strings,
            sha_type,
            no_verbose
        }
    }

    pub fn execute(&self) -> Result<(), IOError> {

        for str in &self.strings {
            let bytes = str.as_bytes();
            let digested_bytes = self.sha_type.digest(bytes);
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
