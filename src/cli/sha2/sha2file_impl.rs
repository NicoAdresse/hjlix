// src/cli/sha2/sha2file_impl.rs

use std::path::PathBuf;
use crate::cli::err::io_err::IOError;
use crate::cli::sha2::sha2_types::Sha2Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha2FileImpl {
    files: Vec<PathBuf>,
    sha_type: Sha2Types,
}

impl Sha2FileImpl {
    pub fn new(files: Vec<PathBuf>, sha_type: Sha2Types) -> Self {
        Self { files, sha_type }
    }

    pub fn execute(&self) -> Result<(), IOError> {
        Ok(())
    }
}