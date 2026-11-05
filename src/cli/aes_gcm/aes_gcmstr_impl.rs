// src/cli/aes_gcm/aes_gcmstr_impl.rs

use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use aes_gcm::KeyInit;
use aes_gcm::aead::Aead;
use aes_gcm::{Key, Aes128Gcm, Aes256Gcm, aead::Generate, Nonce};
use crate::cli::aes_gcm::aes_gcm_types::AESGCMTypes;
use crate::cli::err::aes_err::AESError;
use crate::cli::consts::constants::FILE_NAME_AES_GCM;
use crate::cli::err::io_err::IOError;
use crate::cli::traits::generate_aes_key::HjlixGenerateAESKey;

#[derive(Debug, Clone, PartialEq)]
pub struct AESGCMStrImpl {
    strings: Vec<String>,
    aes_gcm_type: AESGCMTypes,
    write_into_file: bool,
    optional_filename: Option<String>,
    no_verbose: bool,
}

impl AESGCMStrImpl {
    pub fn new(
        strings: Vec<String>,
        aes_gcm_type: AESGCMTypes,
        write_into_file: bool,
        optional_filename: Option<String>,
        no_verbose: bool,
    ) -> Self {
        Self {
            strings,
            aes_gcm_type,
            write_into_file,
            optional_filename,
            no_verbose
        }
    }

    pub fn execute(&self) -> Result<(), AESError> {
        let key = self.aes_gcm_type.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        match aes_gcm_type {
            AESGCMTypes::AES128GCM => {
                let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes128Gcm::new(&aes_key);

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    if !self.no_verbose {
                        println!("Original: {}", string);
                        println!("Result: {}", result);
                    } else {
                        println!("{}", result);
                    }
                }
            },
            AESGCMTypes::AES256GCM => {
                let aes_key = Key::<Aes256Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes256Gcm::new(&aes_key);

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    if !self.no_verbose {
                        println!("Original: {}", string);
                        println!("Result: {}", result);
                    } else {
                        println!("{}", result);
                    }
                }

            }
        };

        Ok(())
    }

    pub fn execute_write_into_file(&self) -> anyhow::Result<()> {
        let key = self.aes_gcm_type.generate_key_for_aes_type();
        let aes_gcm_type = self.aes_gcm_type.figure_out_type(&key)
            .map_err(|_| AESError::FailedToFigureOutType)?;

        let filename = &self.optional_filename.clone().unwrap_or(FILE_NAME_AES_GCM.to_owned());
        let path = Path::new(filename);
        let file = File::create(path)
            .map_err(|_| IOError::FailedToCreateFile)?;
        let mut buf_writer = BufWriter::new(file);

        match aes_gcm_type {
            AESGCMTypes::AES128GCM => {
                let aes_key = Key::<Aes128Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes128Gcm::new(aes_key.as_ref());

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    writeln!(buf_writer, "{}", result)
                        .map_err(|_| IOError::FailedToWrite)?;
                }

                buf_writer.flush()
                    .map_err(|_| IOError::FailedToFlushBufWriter)?;
            },
            AESGCMTypes::AES256GCM => {
                let aes_key = Key::<Aes256Gcm>::try_from(key.as_slice())
                    .map_err(|_| AESError::FailedToCreateKey)?;
                let cipher = Aes256Gcm::new(aes_key.as_ref());

                for string in &self.strings {
                    let nonce = Nonce::generate();
                    let ciphertext = cipher.encrypt(&nonce, string.as_ref())
                        .map_err(|_| AESError::FailedToCreateCiphertext)?;
                    let result = hex::encode(ciphertext);

                    writeln!(buf_writer, "{}", result)
                        .map_err(|_| IOError::FailedToWrite)?;
                }

                buf_writer.flush()
                    .map_err(|_| IOError::FailedToFlushBufWriter)?;
            }
        }

        Ok(())
    }
}