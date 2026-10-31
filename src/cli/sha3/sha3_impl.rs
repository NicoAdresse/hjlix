// src/cli/sha3/sha3_impl.rs

use crate::cli::sha3::sha3_types::Sha3Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha3Impl {
    string: String,
    sha3_type: Sha3Types,
    write_into_file: bool,
}

impl Sha3Impl {
    pub fn new(string: String, sha3_type: Sha3Types, write_into_file: bool) -> Self {
        Self { string, sha3_type, write_into_file }
    }
}