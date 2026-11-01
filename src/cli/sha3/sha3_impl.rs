// src/cli/sha3/sha3_impl.rs

use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::cli::sha3::sha3_types::Sha3Types;
use crate::cli::err::io_err::IOError;
use crate::cli::consts::constants::FILE_NAME;
use crate::cli::traits::digest::HjlixDigest;

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

    pub fn execute(&self) -> Result<(), IOError> {
        let bytes = self.string.as_bytes();
        let digested_bytes = self.sha3_type.digest(bytes);
        let result = hex::encode(digested_bytes);

        if !self.write_into_file {
            println!("Original: {}", self.string);
            println!("Result: {}", result);
        } else {
            let path = Path::new(FILE_NAME);

            if path.exists() {
                return Err(IOError::FileAlreadyExists);
            }

            let file = File::create(path)
                .map_err(|_| IOError::FailedToCreateFile)?;
            let mut buf_writer = BufWriter::new(file);

            writeln!(buf_writer, "{}", result)
                .map_err(|_| IOError::FailedToWrite)?;

            buf_writer.flush()
                .map_err(|_| IOError::FailedToFlushBufWriter)?;
        }

        Ok(())
    }
}