// src/cli/sha2/sha2file_impl.rs

use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
        if self.files.is_empty() {
            return Err(IOError::NoGivenFile);
        }

        for file in &self.files {
            let file = File::open(file)
                .map_err(|_| IOError::FailedToOpenFile)?;
            let buf_reader = BufReader::new(file);

            for lines in buf_reader.lines() {
                let line = lines.map_err(|_| IOError::FailedToReadLines)?;
                let line_bytes = line.as_bytes();
                let result = hex::encode(line_bytes);

                println!("Original: {}", line);
                println!("Result: {}", result);
            }
        }

        Ok(())
    }
}