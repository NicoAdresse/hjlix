// src/cli/sha3/sha3file_impl.rs

use std::path::PathBuf;
use crate::cli::err::io_err::IOError;
use crate::cli::sha3::sha3_types::Sha3Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha3FileImpl {
    files: Vec<PathBuf>,
    sha_type: Sha3Types,
    no_verbose: bool,
}

impl Sha3FileImpl {
    pub fn new(files: Vec<PathBuf>, sha_type: Sha3Types, no_verbose: bool) -> Self {
        Self { files, sha_type, no_verbose }
    }

    pub fn execute(&self) -> Result<(), IOError> {
        if self.files.is_empty() {
            return Err(IOError::NoGivenFile);
        }

        Ok(())
    }
}