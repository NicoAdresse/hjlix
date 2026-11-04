// src/cli/sha3/sha3file_impl.rs

use std::fs::File;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use crate::cli::err::io_err::IOError;
use crate::cli::sha3::sha3_types::Sha3Types;
use crate::cli::traits::digest::HjlixDigest;

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

        for file in &self.files {
            let file = File::open(file)
                .map_err(|_| IOError::FailedToOpenFile)?;
            let buf_reader = BufReader::new(file);

            for lines in buf_reader.lines() {
                let line = lines.map_err(|_| IOError::FailedToReadLines)?;
                let line_bytes = self.sha_type.digest(line.as_bytes());
                let result = hex::encode(line_bytes);

                if !self.no_verbose {
                    println!("Original: {}", line);
                    println!("Result: {}", result);
                } else {
                    println!("{}", result);
                }
            }
        }

        Ok(())
    }
}