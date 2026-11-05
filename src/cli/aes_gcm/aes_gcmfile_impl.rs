// src/cli/aes_gcm/aes_gcmfile_impl.rs

use std::fs::File;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use aes_gcm::{Key, Nonce, aead::{Generate, Aead}, Aes128Gcm, Aes256Gcm, KeyInit};
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::err::{aes_err::AESError, io_err::IOError};
use crate::cli::traits::generate_aes_key::HjlixGenerateAESKey;

#[derive(Debug, Clone, PartialEq)]
pub struct AesGcmFileImpl {
    files: Vec<PathBuf>,
    aes_gcm_type: AESGCMTypes,
    no_verbose: bool,
}

impl AesGcmFileImpl {
    pub fn new(files: Vec<PathBuf>, aes_gcm_type: AESGCMTypes, no_verbose: bool) -> Self {
        Self { files, aes_gcm_type, no_verbose }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        if self.files.is_empty() {
            return Err(IOError::NoGivenFile.into());
        }

        let key = self.aes_gcm_type.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        match aes_gcm_type {
            AESGCMTypes::AES128GCM => {
                for file in &self.files {
                    let file = File::open(file)
                        .map_err(|_| IOError::FailedToOpenFile)?;
                    let buf_writer = BufReader::new(file);

                    let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                        .map_err(|_| AESError::FailedToCreateKey)?;
                    let cipher = Aes128Gcm::new(&aes_key);

                    for line in buf_writer.lines() {
                        let line = line.map_err(|_| IOError::FailedToReadLines)?;

                        let nonce = Nonce::generate();
                        let ciphertext = cipher.encrypt(&nonce, line.as_ref())
                            .map_err(|_| AESError::FailedToCreateCiphertext)?;
                        let result = hex::encode(ciphertext);

                        if !self.no_verbose {
                            println!("Original: {}", line);
                            println!("Result: {}", result);
                        } else {
                            println!("Result: {}", result);
                        }
                    }

                }
            },
            AESGCMTypes::AES256GCM => {
                for file in &self.files {
                    let file = File::open(file)
                        .map_err(|_| IOError::FailedToOpenFile)?;
                    let buf_reader = BufReader::new(file);

                    let aes_key = Key::<Aes256Gcm>::try_from(key.as_slice())
                        .map_err(|_| AESError::FailedToCreateKey)?;
                    let cipher = Aes256Gcm::new(&aes_key);

                    for line in buf_reader.lines() {
                        let line = line.map_err(|_| IOError::FailedToReadLines)?;

                        let nonce = Nonce::generate();
                        let ciphertext = cipher.encrypt(&nonce, line.as_ref())
                            .map_err(|_| AESError::FailedToCreateCiphertext)?;
                        let result = hex::encode(ciphertext);

                        if !self.no_verbose {
                            println!("Original: {}", line);
                            println!("Result: {}", result);
                        } else {
                            println!("{}", result);
                        }
                    }
                }
            },
        }


        Ok(())
    }
}