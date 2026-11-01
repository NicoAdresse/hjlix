// src/cli/sha/sha_impl.rs

use crate::cli::consts::constants::FILE_NAME;
use crate::cli::err::io_err::IOError;
use crate::cli::sha2::sha2_types::Sha2Types;
use crate::cli::traits::digest::HjlixDigest;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha2Impl {
    strings: Vec<String>,
    sha_type: Sha2Types,
    write_into_file: bool,
}

impl Sha2Impl {
    pub fn new(strings: Vec<String>, sha_type: Sha2Types, write_into_file: bool) -> Self {
        Self {
            strings,
            sha_type,
            write_into_file,
        }
    }

    pub fn execute(&self) -> Result<(), IOError> {
        let mut buf_writer = BufWriter::new(file);

        for str in &self.strings {
            let bytes = str.as_bytes();
            let digested_bytes = self.sha_type.digest(bytes);
            let result = hex::encode(digested_bytes);

            if !self.write_into_file {
                println!("Original: {}", str);
                println!("Result: {}", result);
            } else {
                let path = Path::new(FILE_NAME);

                if path.exists() {
                    return Err(IOError::FileAlreadyExists);
                }

                let file = File::create(path).map_err(|_| IOError::FailedToCreateFile)?;
                let mut buf_writer = BufWriter::new(file);

                writeln!(buf_writer, "{}", result).map_err(|_| IOError::FailedToWrite)?;
            }
        }

        buf_writer
            .flush()
            .map_err(|_| IOError::FailedToFlushBufWriter)?;

        Ok(())
    }
}
