// src/cli/sha/sha_impl.rs

use crate::cli::consts::constants::FILE_NAME;
use crate::cli::err::io_err::IOError;
use crate::cli::sha2::sha2_types::Sha2Types;
use crate::cli::traits::digest::HjlixDigest;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Sha2Impl {
    strings: Vec<String>,
    sha_type: Sha2Types,
    write_into_file: bool,
    optional_filename: Option<String>,
}

impl Sha2Impl {
    pub fn new(strings: Vec<String>, sha_type: Sha2Types, write_into_file: bool, optional_filename: Option<String>) -> Self {
        Self {
            strings,
            sha_type,
            write_into_file,
            optional_filename,
        }
    }

    fn init_buffered_writer<P: AsRef<Path>>(
        &self,
        options: &OpenOptions,
        path: P,
    ) -> std::io::Result<BufWriter<File>> {
        let file = options.open(path)?;
        Ok(BufWriter::new(file))
    }

    pub fn execute(&self) -> Result<(), IOError> {
        let filename = &self.optional_filename.clone().unwrap_or(FILE_NAME.to_owned());
        let path = Path::new(filename);

        let mut buf_writer = if self.write_into_file {
            if path.exists() {
                return Err(IOError::FileAlreadyExists);
            }

            let mut options = OpenOptions::new();
            options.write(true).create(true);

            Some(self.init_buffered_writer(&options, path).map_err(|_| IOError::FailedToCreateFile)?)            
        } else {
            None
        };

        for str in &self.strings {
            let bytes = str.as_bytes();
            let digested_bytes = self.sha_type.digest(bytes);
            let result = hex::encode(digested_bytes);

            if let Some(ref mut writer) = buf_writer {
                writeln!(writer, "{}", result).map_err(|_| IOError::FailedToWrite)?;
            } else {
                println!("Original: {}", str);
                println!("Result: {}", result);
            }
        }

        if let Some(ref mut writer) = buf_writer {
            writer
                .flush()
                .map_err(|_| IOError::FailedToFlushBufWriter)?;
        }

        Ok(())
    }
}
